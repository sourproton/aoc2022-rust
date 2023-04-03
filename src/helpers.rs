//! Helpers for AoC. Includes macros, reading functions, etc

use {
    std::fs::File,
    std::io::{self, BufRead},
    std::path::Path,
};

/// solves and displays pt1 and pt2 of `day`
pub fn solve_day(day: u8) {
    let filename = format!("./data/inputs/input{day:02}.txt");

    /// macro takes a `dayXX` module and displays its `dayXX::pt1` and `dayXX::pt2` solutions
    macro_rules! solve {
        ($dayXX:ident) => {{
            let (answer1, time1) = crate::solutions::$dayXX::pt1(&filename);
            let (answer2, time2) = crate::solutions::$dayXX::pt2(&filename);

            println!("Day {day:02}");
            println!("    part 1: {answer1}, elapsed time: {time1} ms");
            println!("    part 2: {answer2}, elapsed time: {time2} ms");
            println!("");
        }};
    }

    // solving `day`
    match day {
        1 => solve!(day01),
        2 => solve!(day02),
        3 => solve!(day03),
        // 4 => solve!(day04),
        // 5 => solve!(day05),
        // 6 => solve!(day06),
        // 7 => solve!(day07),
        // 8 => solve!(day08),
        // 9 => solve!(day09),
        // 10 => solve!(day10),
        // 11 => solve!(day11),
        // 12 => solve!(day12),
        // 13 => solve!(day13),
        // 14 => solve!(day14),
        // 15 => solve!(day15),
        // 16 => solve!(day16),
        // 17 => solve!(day17),
        // 18 => solve!(day18),
        // 19 => solve!(day19),
        // 20 => solve!(day20),
        // 21 => solve!(day21),
        // 22 => solve!(day22),
        // 23 => solve!(day23),
        // 24 => solve!(day24),
        // 25 => solve!(day25),
        _ => println!("Day {day:02}\n    not implemented!\n"),
    }
}

/// returns an iterator over each line in the input file
pub fn read_lines<P>(filename: P) -> std::io::Lines<std::io::BufReader<std::fs::File>>
where
    P: AsRef<Path>,
{
    io::BufReader::new(File::open(filename).expect("not able to open file")).lines()
}
