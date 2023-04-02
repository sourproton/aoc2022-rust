use {
    path::Path,
    std::fs::File,
    std::io::{self, BufRead},
    std::{env::args, path},
};

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
