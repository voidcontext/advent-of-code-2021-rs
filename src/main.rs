mod day1;
mod file_utils;


fn main() {
    let day1_input = file_utils::read_i32_list("data/day1_a.input");
    println!("Day1 A result: {}",day1::count_increases(&day1_input));
    println!("Day1 B result: {}",day1::count_sliding(&day1_input));
}
