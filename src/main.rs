use std::env;

mod helpers;
mod solutions;

fn main() {
    // reading command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        // if no arguments given, run all available solutions
        (1..=25).for_each(|day| {
            helpers::solve_day(day);
        })
    } else {
        // if arguments are given, run the respective solutions
        args.into_iter().skip(1).for_each(|arg| {
            let day = arg
                .parse()
                .unwrap_or_else(|_| panic!("not able to parse command line argument {arg}"));
            helpers::solve_day(day);
        })
    }
}
