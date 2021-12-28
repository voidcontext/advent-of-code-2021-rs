use crate::file_utils::read_and_parse;

pub fn solve() {
    let input: Vec<usize> = read_and_parse("data/day6.input", |input| {
        input
            .trim()
            .split(',')
            .map(|s| s.parse::<usize>())
            .collect()
    })
    .unwrap();

    let mut state_a: Vec<usize> = input.to_vec();
    time_machine(&mut state_a, 80);

    println!("Day6 result A: {}", state_a.len());

    let count: usize = time_machine_optimised(&input_to_optimised_state(&input), 256)
        .iter()
        .sum();
    println!("Day6 result B: {}", count);
}

fn input_to_optimised_state(input: &[usize]) -> [usize; 9] {
    let mut state = [0; 9];

    for i in input {
        state[*i] += 1;
    }

    state
}

fn time_machine(state: &mut Vec<usize>, remaining: usize) {
    if remaining != 0 {
        for i in 0..state.len() {
            match state[i] {
                0 => {
                    state[i] = 6;
                    state.push(8);
                }
                _ => state[i] -= 1,
            }
        }

        time_machine(state, remaining - 1);
    }
}

fn time_machine_optimised(initial_state: &[usize; 9], days: usize) -> [usize; 9] {
    let mut state = *initial_state;

    for _ in 0..days {
        let lantern_fish_timedout = state[0];

        for i in 0..state.len() - 1 {
            state[i] = state[i + 1];
        }
        state[6] += lantern_fish_timedout; // timer reset
        state[8] = lantern_fish_timedout; // new lantern fish
    }

    state
}

#[cfg(test)]
mod tests {
    use crate::puzzles::day6::{input_to_optimised_state, time_machine_optimised};

    use super::time_machine;

    static INPUT: [usize; 5] = [3, 4, 3, 1, 2];
    static AFTER_18DAYS: [usize; 26] = [
        6, 0, 6, 4, 5, 6, 0, 1, 1, 2, 6, 0, 1, 1, 1, 2, 2, 3, 3, 4, 6, 7, 8, 8, 8, 8,
    ];

    static INITIAL_STATE: [usize; 9] = [0, 1, 1, 2, 1, 0, 0, 0, 0];

    #[test]
    fn test_time_machine() {
        let mut state: Vec<usize> = INPUT.to_vec();

        time_machine(&mut state, 18);

        assert_eq!(state, AFTER_18DAYS)
    }

    #[test]
    fn test_input_to_optimised_state() {
        let input = vec![3, 4, 3, 1, 2];

        assert_eq!(
            input_to_optimised_state(&input),
            [0, 1, 1, 2, 1, 0, 0, 0, 0]
        )
    }

    #[test]
    fn test_time_machine_optimised() {
        assert_eq!(
            time_machine_optimised(&INITIAL_STATE, 18),
            input_to_optimised_state(&AFTER_18DAYS)
        )
    }
}
