use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{char, digit1, line_ending};

use nom::combinator::map;
use nom::combinator::map_res;
use nom::multi::{count, separated_list1};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Day4Input {
    numbers_drawn: Vec<u32>,
    bingo_boards: Vec<Vec<u32>>,
}

#[derive(Debug, PartialEq)]
pub struct ParserError {
    message: String,
}

pub fn parse(input: &str) -> Result<Day4Input, ParserError> {
    match separated_pair(
        numbers_drawn_line,
        line_ending,
        separated_list1(line_ending, bingo_board),
    )(input)
    {
        Ok(("", (numbers_drawn, bingo_boards))) => Ok(Day4Input {
            numbers_drawn,
            bingo_boards,
        }),
        Ok((remaining, _)) => Err(ParserError {
            message: format!("Remaining input: '{}'", remaining),
        }),
        Err(err) => Err(ParserError {
            message: format!("{}", err),
        }),
    }
}

fn int_from_string(int: &str) -> Result<u32, std::num::ParseIntError> {
    int.parse()
}

fn u32_parser(input: &str) -> IResult<&str, u32> {
    map_res(digit1, int_from_string)(input)
}

fn check_bingo_number(number: u32) -> Result<u32, ParserError> {
    if number > 75 {
        Err(ParserError {
            message: format!("Not a bingo number, {} > 75", number),
        })
    } else {
        Ok(number)
    }
}

fn bingo_number(input: &str) -> IResult<&str, u32> {
    alt((
        preceded(char(' '), map_res(take(1usize), int_from_string)),
        map_res(map_res(take(2usize), int_from_string), check_bingo_number),
    ))(input)
}

fn numbers_drawn_line(input: &str) -> IResult<&str, Vec<u32>> {
    terminated(
        separated_list1(tag(","), map_res(u32_parser, check_bingo_number)),
        line_ending,
    )(input)
}

fn bingo_board(input: &str) -> IResult<&str, Vec<u32>> {
    map(
        count(
            count(terminated(bingo_number, alt((tag(" "), line_ending))), 5), // TOOD: make this more precise: e.g. incorrect when there's a new line after 3 numbers
            5,
        ),
        |lines| lines.iter().flatten().cloned().collect(),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_numbers_drawn_line() {
        let input = "12,32,43,72,38,60\n";

        assert_eq!(
            numbers_drawn_line(input),
            Ok(("", vec![12, 32, 43, 72, 38, 60]))
        )
    }

    #[test]
    fn test_bingo_board() {
        let board = "12 23  7 12 34
75 54  2 34 35
19 43 32 45 67
63 71  1 37 21
15 72 22 44 51
";

        assert_eq!(
            bingo_board(board),
            Ok((
                "",
                vec![
                    12, 23, 7, 12, 34, 75, 54, 2, 34, 35, 19, 43, 32, 45, 67, 63, 71, 1, 37, 21,
                    15, 72, 22, 44, 51
                ]
            ))
        )
    }

    #[test]
    fn test_parse_should_parse_the_input_format() {
        let input = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

        assert_eq!(
            parse(input),
            Ok(Day4Input {
                numbers_drawn: vec![
                    7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18,
                    20, 8, 19, 3, 26, 1
                ],
                bingo_boards: vec![
                    vec![
                        22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1,
                        12, 20, 15, 19
                    ],
                    vec![
                        3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14,
                        21, 16, 12, 6
                    ],
                    vec![
                        14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5,
                        2, 0, 12, 3, 7
                    ]
                ]
            })
        )
    }
}
