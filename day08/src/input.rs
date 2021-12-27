use std::str::FromStr;

#[derive(Debug)]
pub struct InputLine {
    pub patterns: [String; 10],
    pub output: [String; 4]
}

impl InputLine {
    fn parse_patterns(string: &str) -> [String; 10] {
        let mut patterns: [String; 10] = Default::default();
        for (i, pattern) in string.split_whitespace().enumerate() {
            patterns[i] = String::from(pattern);
        }
        patterns
    }

    fn parse_output(string: &str) -> [String; 4] {
        let mut outputs: [String; 4] = Default::default();
        for (i, output) in string.split_whitespace().enumerate() {
            outputs[i] = String::from(output);
        }
        outputs
    }
}

impl FromStr for InputLine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("|").collect();
        if parts.len() != 2 {
            return Err(String::from("Malformed input"));
        }
        let patterns = InputLine::parse_patterns(parts[0].trim());
        let output   = InputLine::parse_output(parts[1].trim());
        Ok (Self { patterns, output })
    }
}