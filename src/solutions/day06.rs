//! Day 6: Tuning Trouble
//!
//! solution to the day 06 of AoC2022
//!
//! https://adventofcode.com/2022/day/6

use {
    crate::helpers::{read_lines, Answer},
    std::time::SystemTime,
};

/// solves the part 1 of day 05 and return its result and elapsed time
pub fn pt1(filename: &str) -> Answer {
    let time = SystemTime::now();

    let signal = read_lines(filename).next().expect("could not read input");

    let answer = end_of_marker(&signal, 4);

    Answer::new(
        answer.to_string(),
        time.elapsed().unwrap().as_millis() as u32,
    )
}

/// solves the part 2 of day 05 and return its result and elapsed time
pub fn pt2(filename: &str) -> Answer {
    let time = SystemTime::now();

    let signal = read_lines(filename).next().expect("could not read input");

    let answer = end_of_marker(&signal, 14);

    Answer::new(
        answer.to_string(),
        time.elapsed().unwrap().as_millis() as u32,
    )
}

/// The position of the last digit of the first `size` unique character sequence
fn end_of_marker(signal: &str, size: usize) -> usize {
    signal
        .chars()
        .collect::<Vec<char>>()
        .windows(size)
        .enumerate()
        .find(|(_, window)| !(1..size).any(|i| window.iter().skip(i).any(|c| *c == window[i - 1])))
        .expect("could not find marker")
        .0
        + size
}

#[cfg(test)]
mod tests {
    use super::{end_of_marker, pt1, pt2};

    const FILENAME: &str = "./data/examples/example06.txt";

    #[test]
    fn pt01() {
        let answer = pt1(FILENAME);
        assert_eq!(String::from("7"), answer.value());
    }

    #[test]
    fn pt02() {
        let answer = pt2(FILENAME);
        assert_eq!(String::from("19"), answer.value());
    }

    #[test]
    fn test_end_of_marker() {
        let example_1 = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let example_2 = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let example_3 = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let example_4 = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

        assert_eq!(5, end_of_marker(&example_1, 4));
        assert_eq!(6, end_of_marker(&example_2, 4));
        assert_eq!(10, end_of_marker(&example_3, 4));
        assert_eq!(11, end_of_marker(&example_4, 4));

        assert_eq!(23, end_of_marker(&example_1, 14));
        assert_eq!(23, end_of_marker(&example_2, 14));
        assert_eq!(29, end_of_marker(&example_3, 14));
        assert_eq!(26, end_of_marker(&example_4, 14));
    }
}
