use days::*;

mod days;
mod file_utils;
mod submarine;

fn main() {
    let day1_input = file_utils::read_i32_list("data/day1_a.input");
    println!("Day1 A result: {}", day1::count_increases(&day1_input));
    println!("Day1 B result: {}", day1::count_sliding(&day1_input));

    let day2_input = file_utils::read_submarine_commands("data/day2_a.input");
    println!("Day2 A Result {}", day2::calc_position_mul(&day2_input));
    println!("Day2 A Result {}", day2::calc_position_aimed(&day2_input));
}
