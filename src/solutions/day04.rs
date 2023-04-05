//! Day 4: Camp Cleanup
//!
//! solution to the day 04 of AoC2022
//!
//! https://adventofcode.com/2022/day/4

use {
    crate::helpers::{read_lines, Answer},
    std::time::SystemTime,
};

/// solves the part 1 of day 04 and return its result and elapsed time
pub fn pt1(filename: &str) -> Answer {
    let time = SystemTime::now();

    let answer = read_lines(filename)
        .map(|line| parse_line(line).fully_contains())
        .filter(|p| *p)
        .count();

    Answer::new(answer as u32, time.elapsed().unwrap().as_millis() as u32)
}

/// solves the part 2 of day 04 and return its result and elapsed time
pub fn pt2(filename: &str) -> Answer {
    let time = SystemTime::now();

    let answer = read_lines(filename)
        .map(|line| parse_line(line).contains())
        .filter(|p| *p)
        .count();

    Answer::new(answer as u32, time.elapsed().unwrap().as_millis() as u32)
}

// parses a line into a SectionPair
fn parse_line(line: String) -> SectionPair {
    let splits: Vec<u32> = line
        .replace(',', "-")
        .split('-')
        .map(|x| x.parse().unwrap())
        .collect();

    assert!(splits.len() == 4, "failed to parse line");

    let first_section = Section::new(splits[0], splits[1]);
    let second_section = Section::new(splits[2], splits[3]);

    SectionPair::new(first_section, second_section)
}

/// represents a pair of `Section`s
struct SectionPair {
    first: Section,
    second: Section,
}

impl SectionPair {
    // determines if one section fully contains the other
    fn fully_contains(&self) -> bool {
        self.first.start <= self.second.start && self.first.end >= self.second.end
            || self.first.start >= self.second.start && self.first.end <= self.second.end
    }

    // determines if one section contains the other
    fn contains(&self) -> bool {
        // checks for both possibilities of no overlap and takes the negative
        !(
            // checking that first isn't completely before second
            self.first.end < self.second.start
                // checking that first isn't completely after second
                || self.first.start > self.second.end
        )
    }

    // constructor
    fn new(first: Section, second: Section) -> Self {
        SectionPair { first, second }
    }
}

/// represents the start and the end of the section the elf needs to clean
struct Section {
    start: u32,
    end: u32,
}

impl Section {
    // contructor
    fn new(start: u32, end: u32) -> Self {
        Section { start, end }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_line, pt1, pt2};

    const FILENAME: &str = "./data/examples/example04.txt";

    #[test]
    fn test_pt1() {
        let answer = pt1(FILENAME);
        assert_eq!(2, answer.answer());
    }

    #[test]
    fn test_pt2() {
        let answer = pt2(FILENAME);
        assert_eq!(4, answer.answer());
    }

    #[test]
    fn test_parse_line() {
        let line = String::from("2-4,6-8");
        let pair = parse_line(line);

        assert_eq!(2, pair.first.start);
        assert_eq!(4, pair.first.end);
        assert_eq!(6, pair.second.start);
        assert_eq!(8, pair.second.end);
    }

    #[test]
    fn test_fully_contains() {
        let pair1 = parse_line(String::from("2-4,6-8"));
        let pair2 = parse_line(String::from("2-8,3-7"));

        assert!(!pair1.fully_contains());
        assert!(pair2.fully_contains());
    }
}
