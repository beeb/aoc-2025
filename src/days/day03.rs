use winnow::{Parser as _, Result, ascii::newline, combinator::separated, token::take_while};

use crate::days::Day;

#[derive(Debug, Clone)]
pub struct Bank(Vec<u8>);

impl Bank {
    fn max_joltage(&self) -> u8 {
        let (idx, first_digit) = self
            .0
            .iter()
            .enumerate()
            .rev()
            .skip(1)
            .max_by_key(|(_, d)| **d)
            .unwrap();
        let second_digit = self.0.iter().skip(idx + 1).max().unwrap();
        first_digit * 10 + second_digit
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
        input.iter().map(|b| usize::from(b.max_joltage())).sum()
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
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
}
