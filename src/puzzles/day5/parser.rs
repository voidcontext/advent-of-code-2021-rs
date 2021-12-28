use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map, map_res},
    multi::many0,
    sequence::{separated_pair, terminated},
    IResult,
};

use super::{Line, Point};

#[derive(Debug, PartialEq)]
pub struct ParserError {
    message: String,
}

impl ParserError {
    fn new(message: String) -> ParserError {
        ParserError { message }
    }
}

pub fn parse(input: &str) -> Result<Vec<Line>, ParserError> {
    match many0(terminated(line_parser, newline))(input) {
        Ok(("", lines)) => Ok(lines),
        Ok((remaining, _)) => Err(ParserError::new(format!("Remaining input: {}", remaining))),
        Err(err) => Err(ParserError::new(format!("Unexpected error: {:?}", err))),
    }
}

fn usize_parser(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |uint: &str| uint.parse())(input)
}

fn point_parser(input: &str) -> IResult<&str, Point> {
    map(
        separated_pair(usize_parser, tag(","), usize_parser),
        |(x, y)| Point { x, y },
    )(input)
}

fn line_parser(input: &str) -> IResult<&str, Line> {
    separated_pair(point_parser, tag(" -> "), point_parser)(input)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_point_parser() {
        assert_eq!(
            point_parser("233,3540"),
            Ok(("", Point { x: 233, y: 3540 }))
        )
    }

    #[test]
    fn test_line_parser() {
        assert_eq!(
            line_parser("123,342 -> 563,239"),
            Ok(("", (Point { x: 123, y: 342 }, Point { x: 563, y: 239 })))
        )
    }

    #[test]
    fn test_parse() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

        #[rustfmt::skip]
        let expected = vec![
            (Point {x: 0, y: 9}, Point {x: 5, y: 9}),
            (Point {x: 8, y: 0}, Point {x: 0, y: 8}),
            (Point {x: 9, y: 4}, Point {x: 3, y: 4}),
            (Point {x: 2, y: 2}, Point {x: 2, y: 1}),
            (Point {x: 7, y: 0}, Point {x: 7, y: 4}),
            (Point {x: 6, y: 4}, Point {x: 2, y: 0}),
            (Point {x: 0, y: 9}, Point {x: 2, y: 9}),
            (Point {x: 3, y: 4}, Point {x: 1, y: 4}),
            (Point {x: 0, y: 0}, Point {x: 8, y: 8}),
            (Point {x: 5, y: 5}, Point {x: 8, y: 2}),
        ];

        assert_eq!(parse(input), Ok(expected))
    }
}
