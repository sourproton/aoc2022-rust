//! Helpers for AoC. Includes macros, reading functions, etc

use {
    crate::solutions,
    std::fs::File,
    std::io::{self, BufRead},
    std::path::Path,
};

/// solves and displays pt1 and pt2 of `day`
pub fn solve_day(day: u8) {
    let filename = format!("./data/inputs/input{day:02}.txt");
    match day {
        1 => {
            let (answer1, time1) = solutions::day01::pt1(&filename);
            let (answer2, time2) = solutions::day01::pt2(&filename);

            println!("Day {day:02}");
            println!("    part 1: {answer1}, elapsed time: {time1} ms");
            println!("    part 2: {answer2}, elapsed time: {time2} ms");
        }
        2 => {
            let (answer1, time1) = solutions::day02::pt1(&filename);
            let (answer2, time2) = solutions::day02::pt2(&filename);

            println!("Day {day:02}");
            println!("    part 1: {answer1}, elapsed time: {time1} ms");
            println!("    part 2: {answer2}, elapsed time: {time2} ms");
        }
        _ => println!("Day {day:02}\n    not implemented!"),
    }
}

/// returns an iterator over each line in the input file
pub fn read_lines<P>(filename: P) -> std::io::Lines<std::io::BufReader<std::fs::File>>
where
    P: AsRef<Path>,
{
    io::BufReader::new(File::open(filename).expect("not able to open file")).lines()
}
