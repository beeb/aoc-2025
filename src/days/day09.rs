use std::{iter::once, ops::RangeInclusive};

use itertools::Itertools;
use winnow::{
    Parser as _, Result,
    ascii::{dec_uint, newline},
    combinator::{separated, separated_pair},
};

use crate::days::Day;

#[derive(Debug, Copy, Clone, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

/// Calculate the area of a rectangle
fn rectangle_area(a: Point, b: Point) -> usize {
    let w = a.x.abs_diff(b.x) + 1;
    let h = a.y.abs_diff(b.y) + 1;
    w * h
}

/// Check if two ranges overlap
fn range_overlap(a: RangeInclusive<usize>, b: RangeInclusive<usize>) -> bool {
    if a.start() <= b.start() {
        a.end() > b.start()
    } else {
        b.end() > a.start()
    }
}

/// Create a range from two numbers (inclusive)
fn range(a: usize, b: usize) -> RangeInclusive<usize> {
    if a < b { a..=b } else { b..=a }
}

pub struct Day09;

fn parse_point(input: &mut &str) -> Result<Point> {
    separated_pair(dec_uint, ',', dec_uint)
        .map(|(x, y)| Point { x, y })
        .parse_next(input)
}

impl Day for Day09 {
    type Input = Vec<Point>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        separated(1.., parse_point, newline).parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .combinations_with_replacement(2)
            .map(|points| {
                let [a, b, ..] = points.as_slice() else {
                    unreachable!();
                };
                rectangle_area(**a, **b)
            })
            .max()
            .unwrap()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let first_point = input.iter().next().unwrap();
        let edges: Vec<(Point, Point)> = input
            .iter()
            .chain(once(first_point))
            .copied()
            .tuple_windows()
            .collect();
        let mut max_area = 0;
        for corners in input.iter().combinations_with_replacement(2) {
            let [a, b, ..] = corners.as_slice() else {
                unreachable!();
            };
            if a == b {
                continue; // not a rectangle
            }
            let area = rectangle_area(**a, **b);
            // only needed to check the edges if the rectangle is actually a better candidate that previously seen
            if area <= max_area {
                continue;
            }
            // check if any edge crosses rectangle (one end of the edge is fully inside the rectangle)
            if edges.iter().any(|(e, f)| {
                if e.x == f.x {
                    // vertical edge
                    e.x > a.x.min(b.x)
                        && e.x < a.x.max(b.x)
                        && range_overlap(range(e.y, f.y), range(a.y, b.y))
                } else {
                    // horizontal edge
                    e.y > a.y.min(b.y)
                        && e.y < a.y.max(b.y)
                        && range_overlap(range(e.x, f.x), range(a.x, b.x))
                }
            }) {
                continue;
            }
            max_area = area;
        }
        max_area
    }
}

#[cfg(test)]
#[expect(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part1() {
        let parsed = Day09::parser(&mut INPUT).unwrap();
        assert_eq!(Day09::part_1(&parsed), 50);
    }

    #[test]
    fn test_part2() {
        let parsed = Day09::parser(&mut INPUT).unwrap();
        assert_eq!(Day09::part_2(&parsed), 24);
    }
}
