//! Day 1: Calorie Counting
//!
//! solution to the day 01 of AoC2022
//!
//! https://adventofcode.com/2022/day/1

use {
    crate::helpers::{read_lines, Answer},
    std::time::SystemTime,
};

/// solves the part 1 of day 01 and return its result and elapsed time
pub fn pt1(filename: &str) -> Answer {
    let time = SystemTime::now();

    let mut biggest = 0;
    let mut current = 0;

    read_lines(filename).for_each(|line| {
        match line.as_str() {
            // if empty line, compare current block with biggest found until then
            "" => {
                biggest = biggest.max(current);
                current = 0;
            }
            // if number, continue the sum of current block
            _ => current += line.parse::<u32>().expect("unable to parse line {line}"),
        }
    });

    biggest = biggest.max(current);

    Answer::new(
        biggest.to_string(),
        time.elapsed().unwrap().as_millis() as u32,
    )
}

/// solves the part 2 of day 01 and return its result and elapsed time
pub fn pt2(filename: &str) -> Answer {
    let time = SystemTime::now();

    // we want to find the sum of the 3 biggest blocks
    let mut biggest = vec![0, 0, 0];
    let mut current = 0;

    // iterating through each line of the file
    read_lines(filename).for_each(|line| {
        match line.as_str() {
            // if empty line, compare current block with 3 biggest known
            "" => {
                if current > biggest[0] {
                    biggest[2] = biggest[1];
                    biggest[1] = biggest[0];
                    biggest[0] = current;
                } else if current > biggest[1] {
                    biggest[2] = biggest[1];
                    biggest[1] = current;
                } else if current > biggest[2] {
                    biggest[2] = current;
                }
                current = 0;
            }
            // if number, continue the sum of current block
            _ => current += line.parse::<u32>().expect("unable to parse line {line}"),
        }
    });

    if current > biggest[0] {
        biggest[2] = biggest[1];
        biggest[1] = biggest[0];
        biggest[0] = current;
    } else if current > biggest[1] {
        biggest[2] = biggest[1];
        biggest[1] = current;
    } else if current > biggest[2] {
        biggest[2] = current;
    }

    Answer::new(
        biggest.iter().sum::<u32>().to_string(),
        time.elapsed().unwrap().as_millis() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::{pt1, pt2};

    const FILENAME: &str = "./data/examples/example01.txt";

    #[test]
    fn pt01() {
        let answer = pt1(FILENAME);
        assert_eq!(24000.to_string(), answer.value());
    }

    #[test]
    fn pt02() {
        let answer = pt2(FILENAME);
        assert_eq!(45000.to_string(), answer.value());
    }
}
