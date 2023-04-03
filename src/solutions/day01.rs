//! Day 1: Calorie Counting
//!
//! solution to the day 01 of AoC2022
//!
//! https://adventofcode.com/2022/day/1

use {crate::helpers::read_lines, std::time::SystemTime};

pub fn pt1(filename: &str) -> (u32, u32) {
    let time = SystemTime::now();

    let mut biggest = 0;
    let mut current = 0;

    // iterating through each line of the file
    read_lines(filename).for_each(|line| {
        if let Ok(line) = line {
            match line.as_str() {
                // if empty line, compare current block with biggest found until then
                "" => {
                    biggest = biggest.max(current);
                    current = 0;
                }
                // if number, continue the sum of current block
                _ => current += line.parse::<u32>().expect("unable to parse line {line}"),
            }
        }
    });

    biggest = biggest.max(current);

    let answer = biggest;
    let time = time.elapsed().unwrap().as_millis() as u32;

    (answer, time)
}

pub fn pt2(filename: &str) -> (u32, u32) {
    let time = SystemTime::now();

    // we want to find the sum of the 3 biggest blocks
    let mut biggest = vec![0, 0, 0];
    let mut current = 0;

    // iterating through each line of the file
    read_lines(filename).for_each(|line| {
        if let Ok(line) = line {
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

    let answer = biggest.iter().sum();
    let time = time.elapsed().unwrap().as_millis() as u32;

    (answer, time)
}

#[cfg(test)]
mod tests {
    use crate::solutions::day01::{pt1, pt2};

    const FILENAME: &str = "./data/examples/example01.txt";

    #[test]
    fn pt01() {
        let (answer, _) = pt1(FILENAME);
        assert_eq!(24000, answer);
    }

    #[test]
    fn pt02() {
        let (answer, _) = pt2(FILENAME);
        assert_eq!(45000, answer);
    }
}
