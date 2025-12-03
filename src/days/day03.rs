use winnow::{Parser as _, Result, ascii::newline, combinator::separated, token::take_while};

use crate::days::Day;

#[derive(Debug, Clone)]
pub struct Bank(Vec<u8>);

impl Bank {
    #[expect(clippy::cast_possible_wrap)]
    fn max_joltage(&self, n: usize) -> usize {
        let bank_size = self.0.len() as isize;
        let mut bat = Vec::new();
        let mut idx = -1;
        while bat.len() < n {
            let got = bat.len();
            let rem = n - got;
            let (i, digit) = self
                .0
                .iter()
                .enumerate()
                .rev()
                .skip(rem - 1)
                .take((bank_size - idx - rem as isize) as usize)
                .max_by_key(|(_, d)| **d)
                .unwrap();
            idx = i as isize;
            bat.push(digit);
        }
        let mut res = 0;
        for (i, d) in bat.into_iter().rev().enumerate() {
            res += (*d as usize) * 10usize.pow(i as u32);
        }
        res
    }
}

pub struct Day03;

fn parse_bank(input: &mut &str) -> Result<Bank> {
    let bank_str = take_while(1.., |c: char| c.is_ascii_digit()).parse_next(input)?;
    Ok(Bank(bank_str.chars().map(|c| c as u8 - b'0').collect()))
}

impl Day for Day03 {
    type Input = Vec<Bank>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        separated(1.., parse_bank, newline).parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.iter().map(|b| b.max_joltage(2)).sum()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        input.iter().map(|b| b.max_joltage(12)).sum()
    }
}

#[cfg(test)]
#[expect(const_item_mutation)]
mod test {

    use super::*;

    const INPUT: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part1() {
        let parsed = Day03::parser(&mut INPUT).unwrap();
        assert_eq!(Day03::part_1(&parsed), 357);
    }

    #[test]
    fn test_part2() {
        let parsed = Day03::parser(&mut INPUT).unwrap();
        assert_eq!(Day03::part_2(&parsed), 3_121_910_778_619);
    }
}
