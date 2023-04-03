//! Day 3: Rucksack Reorganization
//!
//! solution to the day 03 of AoC2022
//!
//! https://adventofcode.com/2022/day/3

use {crate::helpers::read_lines, std::time::SystemTime};

const VALUE_SMALL_A: u32 = 'a' as u32;
const VALUE_BIG_A: u32 = 'A' as u32;

pub fn pt1(filename: &str) -> (u32, u32) {
    let time = SystemTime::now();

    let answer = read_lines(filename)
        .map(|line| match line {
            Ok(line) => Rucksack::from_line(line).value_of_common(),
            _ => panic!("unable to read line"),
        })
        .sum();

    let time = time.elapsed().unwrap().as_millis() as u32;

    (answer, time)
}

pub fn pt2(filename: &str) -> (u32, u32) {
    let time = SystemTime::now();

    let time = time.elapsed().unwrap().as_millis() as u32;

    (0, time)
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

        if !common.is_alphabetic() {
            panic!("sack content isn't alhabetic");
        }

        if common.is_lowercase() {
            common as u32 - VALUE_SMALL_A + 1
        } else {
            common as u32 - VALUE_BIG_A + 27
        }
    }

    /// finds the item present in the first and second compartments of the rucksack
    fn find_common(&self) -> char {
        self.comp_1
            .chars()
            .find(|c| self.comp_2.contains(c.to_owned()))
            .unwrap()
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

    // #[test]
    // fn pt02() {
    //     let (answer, _) = pt2(FILENAME);
    //     assert_eq!(, answer);
    // }

    #[test]
    fn test_find_common() {
        let sack = Rucksack {
            comp_1: String::from("vJrwpWtwJgWr"),
            comp_2: String::from("hcsFMMfFFhFp"),
        };
        assert_eq!('p', sack.find_common());
    }
}
