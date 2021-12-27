use std::collections::HashMap;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut};
use std::str::FromStr;

pub struct Display {
    digits: [Digit; 4]
}

impl Display {
    pub fn new() -> Display {
        Display {digits: [Digit::new(); 4]}
    }

    pub fn apply_digit_mapping(&mut self, mapping: &HashMap<Digit, Digit>) {
        for digit in &mut self.digits {
            *digit = mapping[digit];
        }
    }

    pub fn as_number(&self) -> Option<u32> {
        let mut num = 0;
        for (i, digit) in self.digits.iter().enumerate() {
            if let Some(digit) = digit.as_number() {
                num += digit * (10u32.pow(i as u32));
            } else {
                return None;
            }
        }
        Some(num)
    }
}

impl Index<usize> for Display {
    type Output = Digit;

    fn index(&self, index: usize) -> &Self::Output {
        &self.digits[index]
    }
}

impl IndexMut<usize> for Display {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.digits[index]
    }
}

mod digit_data {
    pub const SEGMENTS: [u8; 10] = [
        0b1110111,
        0b0100100,
        0b1011101,
        0b1101101,
        0b0101110,
        0b1101011,
        0b1111011,
        0b0100101,
        0b1111111,
        0b1101111,
    ];

    pub const STRINGS: [&str; 10] = [
        "abcefg",
        "cf",
        "acdeg",
        "acdfg",
        "bcdf",
        "abdfg",
        "abdefg",
        "acf",
        "abcdefg",
        "abcdfg",
    ];
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
pub struct Digit {
    segments: [bool; 7],
}

impl Digit {
    pub fn new() -> Digit {
        return Digit {
            segments: [false; 7]
        }
    }

    pub fn from_binary(num: u8) -> Digit {
        let mut segments = [false; 7];
        for i in 0..7 {
            segments[i] = ((num >> i) & 1) != 0;
        }
        Digit {segments}
    }

    pub fn from_number(num: u8) -> Digit {
        Digit::from_binary(digit_data::SEGMENTS[num as usize])
    }

    pub fn count_active_segments(&self) -> u8 {
        let mut count = 0;
        for &segment in &self.segments {
            if segment {
                count += 1;
            }
        }
        count
    }

    pub fn print_large(&self) {
        let to_char = |segment| if segment { '█' } else { ' ' };
        println!(" {0} {0} {0} ", to_char(self.segments[0]));
        for _ in 0..3 {
            println!("{0}     {1}", to_char(self.segments[1]), to_char(self.segments[2]));
        }
        println!(" {0} {0} {0} ", to_char(self.segments[3]));
        for _ in 0..3 {
            println!("{0}     {1}", to_char(self.segments[4]), to_char(self.segments[5]));
        }
        println!(" {0} {0} {0} ", to_char(self.segments[6]));
    }

    pub fn as_number(&self) -> Option<u32> {
        let segments: u8 = self.into();
        for i in 0..=9 {
            if segments == digit_data::SEGMENTS[i] {
                return Some(i as u32);
            }
        }
        None
    }
}

impl FromStr for Digit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut digit = Digit::new();
        for c in s.chars() {
            if 'a' <= c && c <= 'g' {
                let index = ((c as u8) - ('a' as u8)) as usize;
                digit.segments[index] = true;
            } else {
                return Err(format!("Invalid character: `{}`", c));
            }
        }
        Ok(digit)
    }
}

impl std::fmt::Display for Digit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (i, &segment) in self.segments.iter().enumerate() {
            if segment {
                let c: char = (('a' as u8) + (i as u8)) as char;
                write!(f, "{}", c)?;
            }
        }
        Ok(())
    }
}

impl std::fmt::Debug for Digit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)?;
        Ok(())
    }
}

impl std::ops::BitAnd for Digit {
    type Output = Digit;

    fn bitand(self, rhs: Self) -> Self::Output {
        let mut results = self.segments;
        for i in 0..results.len() {
            results[i] &= rhs.segments[i];
        }
        Digit {
            segments: results
        }
    }
}

impl std::ops::BitAndAssign for Digit {
    fn bitand_assign(&mut self, rhs: Self) {
        for (i, segment) in self.segments.iter_mut().enumerate() {
            *segment &= rhs.segments[i];
        }
    }
}

impl std::ops::Not for Digit {
    type Output = Digit;

    fn not(self) -> Self::Output {
        let mut output = Digit::new();
        for (i, &segment) in self.segments.iter().enumerate() {
            output.segments[i] = !segment;
        }
        output
    }
}

impl core::convert::From<&Digit> for u8 {
    fn from(digit: &Digit) -> Self {
        let mut result = 0u8;
        for (i, &segment) in digit.segments.iter().enumerate() {
            if segment {
                result |= 1 << i;
            }
        }
        result
    }
}

/* Tests **************************************************************************************************************/
#[cfg(test)]
mod tests {
    use crate::display::Digit;
    type TestResult = Result<(), String>;

    #[test]
    fn test_digit_strings() -> TestResult {
        // Test the basic ten numbers
        use super::digit_data::{STRINGS, SEGMENTS};
        for i in 0..=9 {
            assert_eq!(STRINGS[i].parse::<Digit>()?, Digit::from_binary(SEGMENTS[i]));
        }
        Ok(())
    }

    #[test]
    fn test_digit_to_string() -> TestResult {
        use super::digit_data::{STRINGS, SEGMENTS};
        for i in 0..=9 {
            assert_eq!(STRINGS[i].parse::<Digit>()?.to_string(), STRINGS[i]);
        }
        Ok(())
    }

    #[test]
    fn test_op_and() {
        use super::digit_data;
        for &data1 in &digit_data::SEGMENTS {
            for &data2 in &digit_data::SEGMENTS {
                let digit1 = Digit::from_binary(data1);
                let digit2 = Digit::from_binary(data2);
                let binary_res: u8 = (&(digit1 & digit2)).into();
                assert_eq!(binary_res, data1 & data2);
            }
        }
    }
}