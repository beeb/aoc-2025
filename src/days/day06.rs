use itertools::izip;
use winnow::{
    Parser as _, Result,
    ascii::{dec_uint, newline, space0, space1},
    combinator::{delimited, separated, separated_pair},
    token::one_of,
};

const NUMBER_LINES: usize = if cfg!(test) { 3 } else { 4 };

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
pub struct Problem {
    numbers: Vec<usize>,
    op: Operator,
}

use crate::days::Day;

pub struct Day06;

fn parse_numbers_line(input: &mut &str) -> Result<Vec<usize>> {
    delimited(
        space0,
        separated(1.., dec_uint::<_, usize, _>, space1),
        space0,
    )
    .parse_next(input)
}

fn parse_numbers(input: &mut &str) -> Result<Vec<Vec<usize>>> {
    separated(NUMBER_LINES, parse_numbers_line, newline).parse_next(input)
}

fn parse_operator(input: &mut &str) -> Result<Operator> {
    one_of(('*', '+'))
        .map(|c: char| match c {
            '*' => Operator::Mul,
            '+' => Operator::Add,
            _ => unreachable!(),
        })
        .parse_next(input)
}

fn parse_operators_line(input: &mut &str) -> Result<Vec<Operator>> {
    delimited(space0, separated(1.., parse_operator, space1), space0).parse_next(input)
}

impl Day for Day06 {
    type Input = Vec<Problem>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        let (numbers, operators) =
            separated_pair(parse_numbers, newline, parse_operators_line).parse_next(input)?;
        Ok(operators
            .into_iter()
            .enumerate()
            .map(|(i, op)| Problem {
                numbers: numbers.iter().map(|n| *n.get(i).unwrap()).collect(),
                op,
            })
            .collect())
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|p| match p.op {
                Operator::Add => p.numbers.iter().sum::<usize>(),
                Operator::Mul => p.numbers.iter().product(),
            })
            .sum()
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

    const INPUT: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        let parsed = Day06::parser(&mut INPUT).unwrap();
        assert_eq!(Day06::part_1(&parsed), 4_277_556);
    }

    #[test]
    fn test_part2() {
        let parsed = Day06::parser(&mut INPUT).unwrap();
        assert_eq!(Day06::part_2(&parsed), 14);
    }
}
