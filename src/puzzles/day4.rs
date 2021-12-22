/// In this puzzle I am going to use parser combinators to parse the
/// numbers drawn and the bingo boards. While the puzzle would
/// necessarily require this, it is a good excercise and
/// demonstration.
mod parser;

use crate::file_utils::read_and_parse;

type BoardState = Vec<(u32, bool)>;

// TODO: memory consumption seems to be high, this needs to be investigated
pub fn solve() {
    let input = read_and_parse("data/day4.input", parser::parse).unwrap();

    let (first_winner, first_winner_index) =
        find_first_winner(&input.bingo_boards, &input.numbers_drawn);

    let result_a = calc_board_score(
        &first_winner,
        input.numbers_drawn[first_winner_index] as usize,
    );
    println!("Day4 A resull: {:?}", result_a);

    let (last_winner, last_winner_index) =
        find_last_winner(&input.bingo_boards, &input.numbers_drawn);

    let result_b = calc_board_score(
        &last_winner,
        input.numbers_drawn[last_winner_index] as usize,
    );
    println!("Day4 B resull: {:?}", result_b);
}

fn initial_state(boards: &[Vec<u32>]) -> Vec<BoardState> {
    boards
        .iter()
        .map(|board| board.iter().map(|&n| (n, false)).collect())
        .collect()
}

fn calc_board_score(board_state: &[(u32, bool)], last_number: usize) -> usize {
    board_state.iter().fold(
        0,
        |acc, &(n, marked)| if marked { acc } else { acc + n as usize },
    ) * last_number
}

fn find_first_winner(boards: &[Vec<u32>], numbers_drawn: &[u32]) -> (BoardState, usize) {
    let mut state = initial_state(boards);
    let mut index = 0;

    loop {
        calc_new_states(&mut state, numbers_drawn, index);

        if let Some(w) = state.iter().find(|b| is_winner(b)) {
            break (w.clone(), index);
        }

        index += 1;
    }
}

fn find_last_winner(boards: &[Vec<u32>], numbers_drawn: &[u32]) -> (BoardState, usize) {
    let mut state = initial_state(boards);
    let mut index = 0;

    loop {
        calc_new_states(&mut state, numbers_drawn, index);

        if state.len() == 1 && is_winner(&state[0]) {
            break (state[0].clone(), index);
        }

        state.retain(|b| !is_winner(b));

        index += 1;
    }
}

fn calc_new_states(board_states: &mut [Vec<(u32, bool)>], numbers_drawn: &[u32], index: usize) {
    for board in board_states {
        for cell in board {
            if cell.0 == numbers_drawn[index] {
                cell.1 = true;
            }
        }
    }
}

fn is_winner(board_state: &[(u32, bool)]) -> bool {
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
        let state: Vec<(u32, bool)> = vec![
            (21, false), (23, true),  (11, false), (54, false), (95, false),
            (21, true),  (23, true),  (11, true),  (54, true),  (95, true),
            (21, false), (23, false), (11, false), (54, false), (95, false),
            (21, false), (23, false), (11, false), (54, false), (95, false),
            (21, false), (23, true),  (11, false), (54, false), (95, false),
        ];

        assert!(is_winner(&state));
    }

    #[test]
    fn test_is_winner_should_returns_true_when_full_column_is_marked() {
        #[rustfmt::skip]
        let state: Vec<(u32, bool)> = vec![
            (21, false), (23, true), (11, false), (54, false), (95, false),
            (21, false), (23, true), (11, false), (54, false), (95, false),
            (21, false), (23, true), (11, false), (54, false), (95, false),
            (21, false), (23, true), (11, false), (54, false), (95, false),
            (21, false), (23, true), (11, false), (54, false), (95, false),
        ];

        assert!(is_winner(&state));
    }
}
