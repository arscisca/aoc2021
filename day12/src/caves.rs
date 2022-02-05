use std::collections::HashMap;
use cave::{Cave, Kind};
use navigation::Navigator;

mod cave {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    use std::fmt::{Display, Debug, Formatter};

    #[derive(Copy, Clone)]
    pub enum Kind {
        Big,
        Small,
    }

    pub struct Cave {
        name: String,
        hash: u64,
        kind: Kind,
        connections: Vec<u64>
    }

    impl Cave {
        pub fn new(name: &str) -> Cave {
            let name = String::from(name);
            let kind = if name.to_lowercase() == name {
                Kind::Small
            } else {
                Kind::Big
            };
            let mut hasher = DefaultHasher::new();
            name.hash(&mut hasher);
            let hash = hasher.finish();
            Cave { name, kind, hash, connections: Vec::new() }
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn kind(&self) -> Kind {
            self.kind
        }

        pub fn hash(&self) -> u64 {
            self.hash
        }

        pub fn add_connection(&mut self, other: &Cave) {
            self.connections.push(other.hash);
        }

        pub fn is_connected(&self, other: &Cave) -> bool {
            self.connections.contains(&other.hash)
        }

        pub fn connections(&self) -> &Vec<u64> {
            &self.connections
        }
    }

    impl Display for Cave {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.name)
        }
    }

    impl Debug for Cave {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{} {:?}", self.name, self.connections)
        }
    }
}

mod navigation {
    use super::*;

    pub struct Navigator<'a> {
        current: u64,
        history: Vec<u64>,
        cavesystem: &'a CaveSystem,
        small_cave_revisits: u32,
    }

    impl<'a> Navigator<'a> {
        pub fn new(system: &'a CaveSystem, cave: &'a Cave) -> Navigator<'a> {
            Navigator {
                current: cave.hash(),
                history: Vec::new(),
                cavesystem: system,
                small_cave_revisits: 0,
            }
        }

        pub fn is_interested(&self, cave: &Cave, max_small_cave_revisits: u32) -> bool {
            match cave.kind() {
                Kind::Big   => true,
                Kind::Small => {
                    if cave.name() == "start" {
                        return false;
                    }
                    if !self.history.contains(&cave.hash()) {
                        return true;
                    }
                    return self.small_cave_revisits < max_small_cave_revisits;
                }
            }
        }

        pub fn walk_in(&mut self, cave: &Cave) {
            self.history.push(self.current);
            if let Kind::Small = cave.kind() {
                if self.history.contains(&cave.hash()) {
                    self.small_cave_revisits += 1;
                }
            }
            self.current = cave.hash();
        }

        fn get_cave_name_by_hash(&self, cave: u64, cavesystem: &CaveSystem) -> Option<String> {
            if let Some(cave) = cavesystem.cave_by_hash(cave) {
                Some(cave.name().into())
            } else {
                None
            }
        }

        pub fn report(&self, cavesystem: &CaveSystem) -> String {
            let mut report = String::new();
            for step in &self.history {
                report += &self.get_cave_name_by_hash(*step, cavesystem).unwrap();
                report.push(',');
            }
            report += &self.get_cave_name_by_hash(self.current, cavesystem).unwrap();
            report
        }

        pub fn current(&self) -> u64 {
            self.current
        }
    }

    impl<'a> Clone for Navigator<'a> {
        fn clone(&self) -> Self {
            Navigator {
                current: self.current,
                history: self.history.clone(),
                cavesystem: self.cavesystem,
                small_cave_revisits: self.small_cave_revisits,
            }
        }
    }
}

pub struct CaveSystem {
    caves: Vec<Cave>,
}

impl CaveSystem {
    pub fn new() -> CaveSystem {
        CaveSystem {
            caves: Vec::new(),
        }
    }

    pub fn cave_by_name(&self, name: &str) -> Option<&Cave> {
        self.caves.iter().find(|cave| cave.name() == name)
    }

    pub fn cave_by_name_mut(&mut self, name: &str) -> Option<&mut Cave> {
        self.caves.iter_mut().find(|cave| cave.name() == name)
    }

    pub fn cave_by_hash(&self, hash: u64) -> Option<&Cave> {
        self.caves.iter().find(|cave| cave.hash() == hash)
    }

    pub fn cave_by_hash_mut(&mut self, hash: u64) -> Option<&mut Cave> {
        self.caves.iter_mut().find(|cave| cave.hash() == hash)
    }

    pub fn insert(&mut self, cave: &str) {
        let cave = Cave::new(cave);
        self.caves.push(cave);
    }

    pub fn contains(&self, cave: &str) -> bool {
        self.caves.iter().any(|c| c.name() == cave)
    }

    pub fn connect(&mut self, c1: &str, c2: &str) -> Result<(), ()> {
        if c1 == c2 {
            return Err(());
        }

        unsafe {
            let c1 = self.cave_by_name_mut(c1).unwrap() as *mut Cave;
            let c2 = self.cave_by_name_mut(c2).unwrap() as *mut Cave;
            c1.as_mut().unwrap().add_connection(c2.as_ref().unwrap());
            c2.as_mut().unwrap().add_connection(c1.as_ref().unwrap());
        }

        Ok(())
    }

    pub fn find_paths(&self, start: &str, end: &str, revisits: u32) -> Result<Vec<String>, ()> {
        const MAX_ITER: u64 = 1000000;
        // Prepare cave endpoints
        let start = self.cave_by_name(start).ok_or(())?;
        let end = self.cave_by_name(end).ok_or(())?;
        // Spawn navigators
        let mut active_navigators: Vec<Navigator> = Vec::new();
        active_navigators.push(Navigator::new(self, start));
        let mut exited_navigators: Vec<Navigator> = Vec::new();
        // Iterate
        let mut iterations = 0u64;
        while !active_navigators.is_empty() && iterations < MAX_ITER {
            // Pick a navigator
            let mut navigator = active_navigators.pop().unwrap();
            let current = self.cave_by_hash(navigator.current()).unwrap();
            if current.hash() == end.hash() {
                exited_navigators.push(navigator);
                iterations += 1;
                continue;
            }
            let neighbors = current.connections()
                                   .iter()
                                   .filter(|hash| start.hash() != **hash)
                                   .map(|hash| self.cave_by_hash(*hash).unwrap())
                                   .collect::<Vec<&Cave>>();
            if neighbors.is_empty() {
                // Somehow reached a dead-end, let the current navigator die
                iterations += 1;
                continue;
            }
            if neighbors.len() > 1 {
                // Spawn additional navigators
                for neighbor in &neighbors[1..] {
                    let mut new_navigator = navigator.clone();
                    if new_navigator.is_interested(neighbor, revisits) {
                        new_navigator.walk_in(neighbor);
                        active_navigators.push(new_navigator);
                    }
                }
            }
            if navigator.is_interested(neighbors[0], revisits) {
                navigator.walk_in(neighbors[0]);
                active_navigators.push(navigator);
            }
            iterations += 1;
        }

        Ok(exited_navigators.iter().map(|navigator| navigator.report(&self)).collect())
    }
}