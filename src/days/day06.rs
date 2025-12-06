use itertools::Itertools;
use winnow::{
    Parser as _, Result,
    ascii::{newline, space0, space1},
    combinator::{delimited, repeat, separated},
    token::one_of,
};

/// The operator for a problem
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Add,
    Mul,
}

/// A cephalopod maths problem
#[derive(Debug, Clone)]
pub struct Problem {
    /// A list of numbers to combine with the operator
    numbers: Vec<usize>,
    /// The operator with which to combine the numbers
    op: Operator,
}

use crate::days::Day;

pub struct Day06;

/// Parse a single operator
fn parse_operator(input: &mut &str) -> Result<Operator> {
    one_of(('*', '+'))
        .map(|c: char| match c {
            '*' => Operator::Mul,
            '+' => Operator::Add,
            _ => unreachable!(),
        })
        .parse_next(input)
}

/// Parse the line with all operators
fn parse_operators_line(input: &mut &str) -> Result<Vec<Operator>> {
    delimited(space0, separated(1.., parse_operator, space1), space0).parse_next(input)
}

impl Day for Day06 {
    type Input = (Vec<Problem>, Vec<Problem>); // part 1, part 2

    fn parser(input: &mut &str) -> Result<Self::Input> {
        // parse the characters of a digits line, including the spaces
        let digits: Vec<Vec<char>> = separated(
            1..,
            repeat::<_, _, Vec<_>, _, _>(1.., one_of(('0'..='9', ' '))),
            newline,
        )
        .parse_next(input)?;
        // consume the last newline before the operators line
        newline.parse_next(input)?;
        // parse the operators line
        let ops = parse_operators_line.parse_next(input)?;

        // digits interpretation for part 1 (row-wise for now)
        let part1_numbers: Vec<Vec<usize>> = digits
            .iter()
            .map(|line| {
                // group digits by grouping them according to the spaces
                line.iter()
                    .chunk_by(|c| c != &&' ') // the key is true for groups of spaces
                    .into_iter()
                    .filter_map(|(is_digit, digits)| {
                        is_digit.then(|| {
                            // digits is a group of digits for a single number
                            // first convert the chars to actual numbers
                            // we have to accumulate the intermediary result because the `Group` operator doesn't
                            // support rev (not double-ended)
                            let digits: Vec<_> = digits.map(|c: &char| *c as u8 - b'0').collect();
                            // iterate over the digits in reverse order (least significant one first)
                            // and convert to the actual number
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

        // digits interpretation for part 2 (column-wise already, right to left)
        let mut part2_numbers: Vec<Vec<usize>> = vec![vec![]];
        // iterate over the columns in reverse order, simultaneously (starting at the end of each line)
        for i in (0..digits[0].len()).rev() {
            // gather the digits in the column
            let pos_digits: Vec<_> = digits.iter().map(|line| *line.get(i).unwrap()).collect();
            // if all the characters are spaces, this is a separator column which means we need to move onto the next
            // problem
            if pos_digits.iter().all(|c| c == &' ') {
                // push a new item in the list, which will hold the numbers for the next problem
                part2_numbers.push(vec![]);
            } else {
                // the digits in this column form a number for the current problem (last_problem)
                let last_problem = part2_numbers.last_mut().unwrap();
                // convert digits to number, bottom to top
                last_problem.push(
                    pos_digits
                        .into_iter()
                        .rev()
                        .filter_map(|c| if c == ' ' { None } else { Some(c as u8 - b'0') }) // ignore spaces
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
                    // get all the numbers for a given column (problem)
                    numbers: part1_numbers.iter().map(|n| *n.get(i).unwrap()).collect(),
                    op: *op,
                })
                .collect(),
            ops.iter()
                .zip(part2_numbers.iter().rev()) // need to reverse because operators are left to right
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
