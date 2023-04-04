//! Day 3: Rucksack Reorganization
//!
//! solution to the day 03 of AoC2022
//!
//! https://adventofcode.com/2022/day/3

use {crate::helpers::read_lines, std::time::SystemTime};

/// solves the part 1 of day 03 and return its result and elapsed time
pub fn pt1(filename: &str) -> (u32, u32) {
    let time = SystemTime::now();

    let answer = read_lines(filename)
        .map(|line| Rucksack::from_line(line).value_of_common())
        .sum();

    let time = time.elapsed().unwrap().as_millis() as u32;

    (answer, time)
}

/// solves the part 2 of day 03 and return its result and elapsed time
pub fn pt2(filename: &str) -> (u32, u32) {
    let time = SystemTime::now();

    let mut current_group = Group {
        first: String::from(""),
        second: String::from(""),
        third: String::from(""),
    };

    let mut current_index = 1;

    let mut answer = 0;

    for line in read_lines(filename) {
        match current_index {
            1 => {
                current_group.first = line;
                current_index += 1;
            }
            2 => {
                current_group.second = line;
                current_index += 1;
            }
            // tuple is complete with group, so find the common item and its priority
            3 => {
                current_group.third = line;
                answer += current_group.value_of_common();
                current_index = 1; // reset counter
            }
            _ => panic!("current index not covered"),
        }
    }

    let time = time.elapsed().unwrap().as_millis() as u32;

    (answer, time)
}

/// represents a Rucksack and the content on its 1st and 2nd compartments
struct Rucksack {
    comp_1: String,
    comp_2: String,
}

impl Rucksack {
    /// returns the priority value of the common item
    fn value_of_common(&self) -> u32 {
        let common = self.find_common();

        char_to_priority(common)
    }

    /// finds the item present in the first and second compartments of the rucksack
    fn find_common(&self) -> char {
        self.comp_1
            .chars()
            .find(|c| self.comp_2.contains(c.to_owned()))
            .expect("no common found")
    }

    /// parses a line into a `Rucksack`
    fn from_line(line: String) -> Self {
        let (comp_1, comp_2) = line.split_at(line.len() / 2);

        Rucksack {
            comp_1: comp_1.to_string(),
            comp_2: comp_2.to_string(),
        }
    }
}

struct Group {
    first: String,
    second: String,
    third: String,
}

impl Group {
    /// returns the priority value of the common item
    fn value_of_common(&self) -> u32 {
        let common = self.find_common();

        char_to_priority(common)
    }

    /// finds the common item that all elfs in the group have
    fn find_common(&self) -> char {
        self.first
            .chars()
            .find(|c| self.second.contains(c.to_owned()) && self.third.contains(c.to_owned()))
            .expect("no common found")
    }
}

/// converts a char into its correnponding priority
fn char_to_priority(c: char) -> u32 {
    if !c.is_alphabetic() {
        panic!("can't convert common to priority. char is not alphabetic");
    } else if c.is_lowercase() {
        // 'a' char has a value of 97. this converts 'a' to 1 and so forth
        c as u32 - 97 + 1
    } else {
        // 'A' char has a value of 65. this converts 'A' to 27 and so forth
        c as u32 - 65 + 27
    }
}

#[cfg(test)]
mod tests {
    use crate::solutions::day03::{pt1, pt2};

    use super::Rucksack;

    const FILENAME: &str = "./data/examples/example03.txt";

    #[test]
    fn pt01() {
        let (answer, _) = pt1(FILENAME);
        assert_eq!(157, answer);
    }

    #[test]
    fn pt02() {
        let (answer, _) = pt2(FILENAME);
        assert_eq!(70, answer);
    }

    #[test]
    fn test_find_common() {
        let sack = Rucksack {
            comp_1: String::from("vJrwpWtwJgWr"),
            comp_2: String::from("hcsFMMfFFhFp"),
        };
        assert_eq!('p', sack.find_common());
    }
}
