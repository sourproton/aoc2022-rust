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

    /// the macro takes a `dayXX` submodule and displays its `dayXX::pt1` and `dayXX::pt2` solutions
    macro_rules! solve {
        ($dayXX:ident) => {{
            let (answer1, time1) = solutions::$dayXX::pt1(&filename);
            let (answer2, time2) = solutions::$dayXX::pt2(&filename);

            println!("Day {day:02}");
            println!("    part 1: {answer1}, elapsed time: {time1} ms");
            println!("    part 2: {answer2}, elapsed time: {time2} ms");
        }};
    }

    // solving the `day`
    match day {
        1 => solve!(day01),
        2 => solve!(day02),
        // 3 => solve!(day03),
        // 4 => solve!(day04),
        // 5 => solve!(day05),
        // 6 => solve!(day06),
        // 7 => solve!(day07),
        // 8 => solve!(day08),
        // 9 => solve!(day09),
        // 10 => solve!(day010),
        // 11 => solve!(day011),
        // 12 => solve!(day012),
        // 13 => solve!(day013),
        // 14 => solve!(day014),
        // 15 => solve!(day015),
        // 16 => solve!(day016),
        // 17 => solve!(day017),
        // 18 => solve!(day018),
        // 19 => solve!(day019),
        // 20 => solve!(day020),
        // 21 => solve!(day021),
        // 22 => solve!(day022),
        // 23 => solve!(day023),
        // 24 => solve!(day024),
        // 25 => solve!(day025),
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
