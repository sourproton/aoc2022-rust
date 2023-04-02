use {
    path::Path,
    std::fs::File,
    std::io::{self, BufRead},
    std::{env::args, path},
};

mod day01;

fn main() {
    args().skip(1).for_each(|arg| {
        let day = arg
            .parse()
            .expect("not able to parse command line argument {arg}");
        solve_day(day);
    })
}

/// call functions to solve pt1 and pt2 of `day`
fn solve_day(day: u8) {
    let filename = format!("./data/input{day:02}.txt");
    match day {
        1 => {
            let (answer1, time1) = day01::pt1(&filename);
            let (answer2, time2) = day01::pt2(&filename);

            println!("Day {day:02}");
            println!("    part 1: {answer1}, elapsed time: {time1} ms");
            println!("    part 2: {answer2}, elapsed time: {time2} ms");
        }
        _ => println!("Day {day:02}\n    not implemented!"),
    }
}

/// returns an iterator over each line in the input file
fn read_lines<P>(filename: P) -> std::io::Lines<std::io::BufReader<std::fs::File>>
where
    P: AsRef<Path>,
{
    io::BufReader::new(File::open(filename).expect("not able to open file")).lines()
}
