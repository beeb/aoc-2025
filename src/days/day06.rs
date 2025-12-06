use itertools::Itertools;
use winnow::{
    Parser as _, Result,
    ascii::{newline, space0, space1},
    combinator::{delimited, repeat, separated},
    token::one_of,
};

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
    type Input = (Vec<Problem>, Vec<Problem>); // part 1, part 2

    fn parser(input: &mut &str) -> Result<Self::Input> {
        let digits: Vec<Vec<char>> = separated(
            1..,
            repeat::<_, _, Vec<_>, _, _>(1.., one_of(('0'..='9', ' '))),
            newline,
        )
        .parse_next(input)?;
        newline.take().parse_next(input)?;
        let ops = parse_operators_line.parse_next(input)?;
        let part1_numbers: Vec<Vec<usize>> = digits
            .iter()
            .map(|line| {
                line.iter()
                    .chunk_by(|c| c != &&' ')
                    .into_iter()
                    .filter_map(|(is_digit, digits)| {
                        is_digit.then_some({
                            let digits: Vec<_> = digits
                                .filter_map(|c: &char| {
                                    if c == &' ' {
                                        None
                                    } else {
                                        Some(*c as u8 - b'0')
                                    }
                                })
                                .collect();
                            digits
                                .into_iter()
                                .rev()
                                .enumerate()
                                .map(|(i, d)| d as usize * 10usize.pow(i as u32))
                                .sum()
                        })
                    })
                    .collect()
            })
            .collect();
        let mut part2_numbers: Vec<Vec<usize>> = vec![vec![]];
        for i in (0..digits[0].len()).rev() {
            let pos_digits: Vec<_> = digits.iter().map(|line| *line.get(i).unwrap()).collect();
            if pos_digits.iter().all(|c| c == &' ') {
                part2_numbers.push(vec![]);
            } else {
                let last_problem = part2_numbers.last_mut().unwrap();
                last_problem.push(
                    pos_digits
                        .into_iter()
                        .rev()
                        .filter_map(|c| if c == ' ' { None } else { Some(c as u8 - b'0') })
                        .enumerate()
                        .map(|(i, d)| d as usize * 10usize.pow(i as u32))
                        .sum(),
                );
            }
        }
        Ok((
            ops.iter()
                .enumerate()
                .map(|(i, op)| Problem {
                    numbers: part1_numbers.iter().map(|n| *n.get(i).unwrap()).collect(),
                    op: *op,
                })
                .collect(),
            ops.iter()
                .zip(part2_numbers.iter().rev())
                .map(|(op, numbers)| Problem {
                    numbers: numbers.clone(),
                    op: *op,
                })
                .collect(),
        ))
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let (input, _) = input;
        input
            .iter()
            .map(|p| match p.op {
                Operator::Add => p.numbers.iter().sum::<usize>(),
                Operator::Mul => p.numbers.iter().product(),
            })
            .sum()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let (_, input) = input;
        input
            .iter()
            .map(|p| match p.op {
                Operator::Add => p.numbers.iter().sum::<usize>(),
                Operator::Mul => p.numbers.iter().product(),
            })
            .sum()
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
        assert_eq!(Day06::part_2(&parsed), 3_263_827);
    }
}
