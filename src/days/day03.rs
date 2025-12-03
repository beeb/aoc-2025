use winnow::{Parser as _, Result, ascii::newline, combinator::separated, token::take_while};

use crate::days::Day;

#[derive(Debug, Clone)]
pub struct Bank(Vec<u8>);

impl Bank {
    /// Select `n` batteries from the bank to form the highest joltage when combining them, and return the joltage.
    ///
    /// The process to select batteries goes as follows:
    ///
    /// 1. Starting from the end of the bank, skip as many items as required to have enough to complete the selection
    ///    after selecting the current battery. For example, if we still have 5 batteries left to pick to complete the
    ///    selection, we skip the last 4. This would leave us with 4 additional batteries to pick from in case the
    ///    5th-to-last is picked next, making the selection feasible;
    /// 2. In the rest of the batteries to the left of that position, and up until the last battery that we previously
    ///    picked (or the start), select the maximum value;
    /// 3. Repeat until we have enough batteries.
    fn max_joltage(&self, n: usize) -> usize {
        let bank_size = self.0.len();
        let mut bat = Vec::new();
        // how many batteries must we ignore to the left of the last selected one (included)
        let mut ignore_left = 0;
        while bat.len() < n {
            // remaining batteries left to pick
            let rem = n - bat.len();
            // iterate in reverse order, skipping (rem - 1) items (to leave enough for subsequent selection)
            // and up until the last battery that was selected
            let (i, digit) = self
                .0
                .iter()
                .enumerate()
                .rev()
                .skip(rem - 1)
                .take(bank_size - ignore_left - rem + 1) // bank_size - ignore_left - (rem - 1)
                .max_by_key(|(_, d)| **d)
                .unwrap();
            ignore_left = i + 1; // record how many batteries to ignore on the left side
            bat.push(digit); // save the selected battery
        }
        combine_batteries(&bat)
    }
}

/// Calculate the combined joltage for a set of batteries.
fn combine_batteries(bat: &[&u8]) -> usize {
    let mut res = 0;
    for (i, d) in bat.iter().rev().enumerate() {
        res += (**d as usize) * 10usize.pow(i as u32);
    }
    res
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
