use std::isize;

use crate::file_utils::read_string_list;

pub fn solve() {
    let diagnostic = read_string_list("data/day3.input");
    let number_of_bits = diagnostic[0].len();
    let numeric_diagnostic = parse_str_diagnostic(&diagnostic);

    let (gamma_rate, epsilon_rate) =
        calc_power_consumption_rates(&numeric_diagnostic, number_of_bits);

    println!(
        "Day3 A result: gamma_rate: {}, epsilon_rate: {}, power consumption: {}",
        gamma_rate,
        epsilon_rate,
        gamma_rate * epsilon_rate
    );

    let oxygen_generator_rating = calc_oxigen_generator_rating(&numeric_diagnostic, number_of_bits);
    let co2_scrubber_rating = calc_co2_scrubber_rating(&numeric_diagnostic, number_of_bits);

    println!(
        "Day3 B result: Oxygen Generator rating: {}, CO2 Scrubber rating: {}, Life Support rating: {}",
        oxygen_generator_rating,
        co2_scrubber_rating,
        oxygen_generator_rating * co2_scrubber_rating
    );
}

fn parse_str_diagnostic(diagnostic: &[String]) -> Vec<isize> {
    diagnostic
        .iter()
        .map(|bin_str| isize::from_str_radix(bin_str, 2).unwrap())
        .collect()
}

fn calc_power_consumption_rates(diagnostic: &[isize], nr_of_bits: usize) -> (i32, i32) {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    most_common_bits(diagnostic, nr_of_bits)
        .iter()
        .enumerate()
        .for_each(|(n, most_comon_bit)| {
            if most_comon_bit == &1 {
                gamma_rate |= 1 << n
            } else {
                epsilon_rate |= 1 << n
            }
        });

    (gamma_rate, epsilon_rate)
}

fn calc_oxigen_generator_rating(diagnostic: &[isize], nr_of_bits: usize) -> isize {
    filter_diagnostic(
        diagnostic,
        nr_of_bits,
        nr_of_bits - 1,
        |bit, most_common_bit| bit == most_common_bit,
    )
}

fn calc_co2_scrubber_rating(diagnostic: &[isize], nr_of_bits: usize) -> isize {
    filter_diagnostic(
        diagnostic,
        nr_of_bits,
        nr_of_bits - 1,
        |bit, most_common_bit| bit != most_common_bit,
    )
}

fn filter_diagnostic(
    diagnostic: &[isize],
    nr_of_bits: usize,
    bit: usize,
    pred: fn(usize, usize) -> bool,
) -> isize {
    let most_common_bit = most_common_bit(diagnostic, bit);

    let filtered: Vec<isize> = diagnostic
        .iter()
        .filter(|diagnostic| pred(nth_bit(diagnostic, bit), most_common_bit))
        .cloned()
        .collect();

    if filtered.len() == 1 {
        filtered[0]
    } else {
        filter_diagnostic(&filtered, nr_of_bits, bit - 1, pred)
    }
}

fn most_common_bit(diagnostic: &[isize], n_bit: usize) -> usize {
    let mut frequencies = vec![0; 2];

    diagnostic
        .iter()
        .for_each(|diagnostic| frequencies[nth_bit(diagnostic, n_bit)] += 1);

    if frequencies[0] > frequencies[1] {
        0
    } else {
        1
    }
}

fn most_common_bits(diagnostic: &[isize], nr_of_bits: usize) -> Vec<usize> {
    (0..nr_of_bits)
        .map(|n_bit| most_common_bit(diagnostic, n_bit))
        .collect()
}

fn nth_bit(x: &isize, n: usize) -> usize {
    let mask = 1 << n;
    ((x & mask) >> n).try_into().unwrap()
}

#[cfg(test)]

mod tests {
    use super::*;

    fn diagnostic() -> Vec<String> {
        vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ]
        .iter()
        .map(|s| (*s).to_owned())
        .collect()
    }

    #[test]
    fn test_nth_bit() {
        diagnostic().iter().for_each(|bin_str| {
            let n = isize::from_str_radix(bin_str, 2).unwrap();
            let reconstructed = (0..5).fold("".to_owned(), |acc, index| {
                nth_bit(&n, index).to_string() + &acc
            });
            assert_eq!(reconstructed, *bin_str)
        });
    }

    #[test]
    fn test_calc_power_consumption_rates() {
        assert_eq!(
            calc_power_consumption_rates(&parse_str_diagnostic(&diagnostic()), 5),
            (22, 9)
        );
    }

    #[test]
    fn test_calc_oxygen_rating() {
        assert_eq!(
            calc_oxigen_generator_rating(&parse_str_diagnostic(&diagnostic()), 5),
            23
        )
    }

    #[test]
    fn test_calc_co2_scrubber_rating() {
        assert_eq!(
            calc_co2_scrubber_rating(&parse_str_diagnostic(&diagnostic()), 5),
            10
        )
    }
}
