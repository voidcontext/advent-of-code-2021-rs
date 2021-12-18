/// In this puzzle I am going to use parser combinators to parse the
/// numbers drawn and the bingo boards. While the puzzle would
/// necessarily require this, it is a good excercise and
/// demonstration.
mod parser;

use crate::file_utils::read_and_parse;

type BoardState<'a> = Vec<(&'a u32, bool)>;

// TODO: memory consumption seems to be high, this needs to be investigated
pub fn solve() {
    let input = read_and_parse("data/day4.input", parser::parse).unwrap();

    let initial_board_states: Vec<BoardState> = input
        .bingo_boards
        .iter()
        .map(|board| board.iter().map(|n| (n, false)).collect())
        .collect();

    let (first_winner, first_winner_index) =
        find_first_winner(&initial_board_states, &input.numbers_drawn, 0);

    let result_a = calc_board_score(
        &first_winner,
        input.numbers_drawn[first_winner_index] as usize,
    );
    println!("Day4 A resull: {:?}", result_a);

    let (last_winner, last_winner_index) =
        find_last_winner(&initial_board_states, &input.numbers_drawn, 0);

    let result_b = calc_board_score(
        &last_winner,
        input.numbers_drawn[last_winner_index] as usize,
    );
    println!("Day4 B resull: {:?}", result_b);
}

fn calc_board_score(board_state: &[(&u32, bool)], last_number: usize) -> usize {
    board_state.iter().fold(
        0,
        |acc, (n, marked)| if *marked { acc } else { acc + (**n) as usize },
    ) * last_number
}

fn find_first_winner<'a>(
    board_states: &[Vec<(&'a u32, bool)>],
    numbers_drawn: &[u32],
    index: usize,
) -> (BoardState<'a>, usize) {
    let new_state = calc_new_states(board_states, numbers_drawn, index);

    if let Some(winner) = new_state.iter().find(|b| is_winner(b)) {
        (winner.to_vec(), index)
    } else {
        find_first_winner(&new_state, numbers_drawn, index + 1)
    }
}

fn find_last_winner<'a>(
    board_states: &[Vec<(&'a u32, bool)>],
    numbers_drawn: &[u32],
    index: usize,
) -> (BoardState<'a>, usize) {
    let new_states = calc_new_states(board_states, numbers_drawn, index);

    if new_states.len() == 1 && is_winner(&new_states[0]) {
        (new_states[0].to_vec(), index)
    } else {
        find_last_winner(
            &new_states
                .into_iter()
                .filter(|b| !is_winner(b))
                .collect::<Vec<_>>(),
            numbers_drawn,
            index + 1,
        )
    }
}

fn calc_new_states<'a>(
    board_states: &[Vec<(&'a u32, bool)>],
    numbers_drawn: &[u32],
    index: usize,
) -> Vec<BoardState<'a>> {
    board_states
        .iter()
        .map(|board_state| {
            board_state
                .iter()
                .map(|(n, marked)| (*n, *marked || **n == numbers_drawn[index]))
                .collect()
        })
        .collect()
}

fn is_winner(board_state: &[(&u32, bool)]) -> bool {
    let (row_marked, column_marked) =
        (0..5).fold((false, false), |(row_marked, column_marked), i| {
            let (current_row, current_column) =
                (0..5).fold((true, true), |(row_marked, column_marked), j| {
                    (
                        row_marked && board_state[(i * 5) + j].1,
                        column_marked && board_state[i + (j * 5)].1,
                    )
                });

            (row_marked || current_row, column_marked || current_column)
        });

    row_marked || column_marked
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_winner_should_returns_true_when_full_row_is_marked() {
        #[rustfmt::skip]
        let state: Vec<(&u32, bool)> = vec![
            (&21, false), (&23, true),  (&11, false), (&54, false), (&95, false),
            (&21, true),  (&23, true),  (&11, true),  (&54, true),  (&95, true),
            (&21, false), (&23, false), (&11, false), (&54, false), (&95, false),
            (&21, false), (&23, false), (&11, false), (&54, false), (&95, false),
            (&21, false), (&23, true),  (&11, false), (&54, false), (&95, false),
        ];

        assert!(is_winner(&state));
    }

    #[test]
    fn test_is_winner_should_returns_true_when_full_column_is_marked() {
        #[rustfmt::skip]
        let state: Vec<(&u32, bool)> = vec![
            (&21, false), (&23, true), (&11, false), (&54, false), (&95, false),
            (&21, false), (&23, true), (&11, false), (&54, false), (&95, false),
            (&21, false), (&23, true), (&11, false), (&54, false), (&95, false),
            (&21, false), (&23, true), (&11, false), (&54, false), (&95, false),
            (&21, false), (&23, true), (&11, false), (&54, false), (&95, false),
        ];

        assert!(is_winner(&state));
    }
}
