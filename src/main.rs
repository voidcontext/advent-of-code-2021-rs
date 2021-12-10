use std::env;

mod file_utils;
mod puzzles;
mod submarine;

fn main() {
    let args: Vec<String> = env::args().collect();

    let default = "all".to_owned();
    let filter = args.get(1).unwrap_or(&default);

    let puzzles: Vec<fn()> = vec![
        puzzles::day1::solve,
        puzzles::day2::solve,
        puzzles::day3::solve,
    ];

    puzzles.iter().enumerate().for_each(|(i, solve)| {
        if filter == "all" || filter == &(i + 1).to_string() {
            solve()
        }
    });
}
