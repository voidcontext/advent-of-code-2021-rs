use crate::file_utils::read_and_parse;
use std::{
    cmp::{max, min},
    collections::HashMap,
    ops::Neg,
};

mod parser;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn is_diagonal(&self, other: &Self) -> bool {
        let (dx, dy) = self.diff(other);

        dx.abs() == dy.abs()
    }

    fn diff(&self, other: &Self) -> (isize, isize) {
        (
            self.x as isize - other.x as isize,
            self.y as isize - other.y as isize,
        )
    }
}

type Line = (Point, Point);

pub fn solve() {
    let input = read_and_parse("data/day5.input", parser::parse).unwrap();

    println!(
        "Day5 A result: {}",
        count_overlaps(&count_coverage(&input, false))
    );
    println!(
        "Day5 B result: {}",
        count_overlaps(&count_coverage(&input, true))
    );
}

fn line_to_points(line: &Line, count_diagonal: bool) -> Vec<Point> {
    match line {
        &(Point { x: x1, y: y1 }, Point { x: x2, y: y2 }) if x1 == x2 => (min(y1, y2)
            ..max(y1, y2) + 1)
            .map(|y| Point { x: x1, y })
            .collect(),
        &(Point { x: x1, y: y1 }, Point { x: x2, y: y2 }) if y1 == y2 => (min(x1, x2)
            ..max(x1, x2) + 1)
            .map(|x| Point { x, y: y1 })
            .collect(),
        (p1, p2) if p1.is_diagonal(p2) && count_diagonal => {
            let (_, dy) = p1.diff(p2);

            (min(p1.x, p2.x)..max(p1.x, p2.x) + 1)
                .map(|x| Point {
                    x,
                    y: (p1.y as isize + ((p1.x as isize - x as isize).abs() * dy.signum().neg()))
                        as usize,
                })
                .collect()
        }
        _ => vec![],
    }
}

fn count_coverage(lines: &[Line], count_diagonal: bool) -> HashMap<Point, usize> {
    let mut count = HashMap::new();

    for line in lines {
        for point in line_to_points(line, count_diagonal) {
            let c = count.entry(point).or_insert(0);
            *c += 1
        }
    }

    count
}

fn count_overlaps(count: &HashMap<Point, usize>) -> usize {
    count.iter().filter(|(_, &c)| c >= 2).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_to_points_horizontal() {
        let line = (Point { x: 1, y: 1 }, Point { x: 1, y: 3 });
        let expected = vec![
            Point { x: 1, y: 1 },
            Point { x: 1, y: 2 },
            Point { x: 1, y: 3 },
        ];

        assert_eq!(line_to_points(&line, false), expected);
    }

    #[test]
    fn test_line_to_points_vertical() {
        let line = (Point { x: 9, y: 7 }, Point { x: 7, y: 7 });
        let expected = vec![
            Point { x: 7, y: 7 },
            Point { x: 8, y: 7 },
            Point { x: 9, y: 7 },
        ];

        assert_eq!(line_to_points(&line, false), expected);
    }

    #[test]
    fn test_line_to_point_diagonal() {
        let line = (Point { x: 1, y: 1 }, Point { x: 3, y: 3 });
        let expected = vec![
            Point { x: 1, y: 1 },
            Point { x: 2, y: 2 },
            Point { x: 3, y: 3 },
        ];

        assert_eq!(line_to_points(&line, true), expected);
    }

    #[test]
    fn test_line_to_point_diagonal_2() {
        let line = (Point { x: 9, y: 7 }, Point { x: 7, y: 9 });
        let expected = vec![
            Point { x: 7, y: 9 },
            Point { x: 8, y: 8 },
            Point { x: 9, y: 7 },
        ];

        assert_eq!(line_to_points(&line, true), expected);
    }

    #[test]
    fn test_line_to_points_non_horizontal_vertical_or_diagonal() {
        assert_eq!(
            line_to_points(&(Point { x: 6, y: 4 }, Point { x: 2, y: 1 }), true),
            vec![]
        )
    }
}
