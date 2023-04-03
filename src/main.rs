mod helpers;
mod solutions;

fn main() {
    let time = std::time::SystemTime::now();

    // reading command line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        // if no arguments given, run all available solutions
        (1..=25).for_each(|day| {
            helpers::solve_day(day);
        })
    } else {
        // if arguments are given, run the respective solutions
        args.into_iter().skip(1).for_each(|arg| {
            if let Ok(day) = arg.parse() {
                helpers::solve_day(day);
            }
        })
    }

    println!(
        "total elapsed time: {} ms",
        time.elapsed().unwrap().as_millis()
    );
}
