use std::fmt::{Debug, Formatter};

pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            items: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Stack<T> {
        Stack {
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn top(&self) -> Option<&T> {
        self.items.get(self.items.len().checked_sub(1).unwrap_or(0))
    }
}

impl<T: Debug> Debug for Stack<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.items.fmt(f)
    }
}