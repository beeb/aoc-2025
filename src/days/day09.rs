use itertools::Itertools;
use pathfinding::grid::Grid;
use winnow::{
    Parser as _, Result,
    ascii::{dec_uint, newline},
    combinator::{separated, separated_pair},
};

use crate::days::Day;

fn rectangle_size(a: (usize, usize), b: (usize, usize)) -> usize {
    let w = a.0.abs_diff(b.0) + 1;
    let h = a.1.abs_diff(b.1) + 1;
    w * h
}

pub struct Day09;

fn parse_point(input: &mut &str) -> Result<(usize, usize)> {
    separated_pair(dec_uint, ',', dec_uint).parse_next(input)
}

impl Day for Day09 {
    type Input = Grid;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        let points: Vec<_> = separated(1.., parse_point, newline).parse_next(input)?;
        Ok(points.into_iter().collect())
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
                rectangle_size(*a, *b)
            })
            .max()
            .unwrap()
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
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
        assert_eq!(Day09::part_2(&parsed), 0);
    }
}
