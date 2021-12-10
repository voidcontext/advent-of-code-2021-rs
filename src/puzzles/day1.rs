use crate::file_utils;

pub fn solve() {
    let day1_input = file_utils::read_i32_list("data/day1.input");
    println!("Day1 A result: {}", count_increases(&day1_input));
    println!("Day1 B result: {}", count_sliding(&day1_input));
}

fn count_increases(measurements: &[i32]) -> i32 {
    (measurements[1..])
        .iter()
        .fold((0, &measurements[0]), |(count, previous), current| {
            (count + if previous < current { 1 } else { 0 }, current)
        })
        .0
}

fn count_sliding(measurements: &[i32]) -> i32 {
    (measurements[0..])
        .iter()
        .zip(&measurements[1..])
        .zip(&measurements[2..])
        .fold(
            (0, measurements[0] + measurements[1] + measurements[2]),
            |(count, previous_sum), ((a, b), c)| {
                let current = a + b + c;
                (count + if previous_sum < current { 1 } else { 0 }, current)
            },
        )
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> Vec<i32> {
        vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263]
    }

    #[test]
    fn test_example() {
        assert_eq!(count_increases(&example()), 7)
    }

    #[test]
    fn test_sliding_example() {
        assert_eq!(count_sliding(&example()), 5)
    }
}
