use std::{collections::HashSet, ops::RangeInclusive};

use winnow::{
    Parser as _, Result,
    ascii::dec_uint,
    combinator::{separated, separated_pair},
};

use crate::days::Day;

const MAX_DIGITS: u32 = 10;
const MAX_SEED: usize = 10usize.pow(MAX_DIGITS / 2); // pattern needs to repeat at least twice

trait IntoParts {
    fn into_parts(self) -> Option<(usize, usize)>;
}

impl IntoParts for usize {
    fn into_parts(self) -> Option<(usize, usize)> {
        let digits = self.ilog10() + 1;
        if !digits.is_multiple_of(2) {
            return None; // number must have an even number of digits
        }
        let half_digits = digits >> 1;
        let divisor = 10usize.pow(half_digits);
        let first = self / divisor;
        let second = self - (first * divisor);
        Some((first, second))
    }
}

trait RepeatDigits {
    fn repeat_digits(self, repeat: u32) -> Option<usize>;
}

impl RepeatDigits for usize {
    fn repeat_digits(self, repeat: u32) -> Option<usize> {
        let digits = self.ilog10() + 1;
        if digits * repeat > MAX_DIGITS {
            return None;
        }
        let mut res = self;
        for i in 1..repeat {
            res += self * 10usize.pow(digits * i);
        }
        Some(res)
    }
}

pub struct Day02;

fn parse_range(input: &mut &str) -> Result<RangeInclusive<usize>> {
    let (start, end) = separated_pair(dec_uint, '-', dec_uint).parse_next(input)?;
    Ok(start..=end)
}

impl Day for Day02 {
    type Input = Vec<RangeInclusive<usize>>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        separated(1.., parse_range, ',').parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .flat_map(Clone::clone)
            .filter(|id| {
                id.into_parts()
                    .is_some_and(|(first, second)| first == second)
            })
            .sum()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut res = 0;
        let mut set = HashSet::new();
        for seed in 1..MAX_SEED {
            for repeat in 2..MAX_DIGITS {
                let Some(id) = seed.repeat_digits(repeat) else {
                    continue;
                };
                if set.contains(&id) {
                    continue;
                }
                set.insert(id);
                for range in input {
                    if range.contains(&id) {
                        res += id;
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        let parsed = Day02::parser(&mut INPUT).unwrap();
        assert_eq!(Day02::part_1(&parsed), 1_227_775_554);
    }

    #[test]
    fn test_part2() {
        let parsed = Day02::parser(&mut INPUT).unwrap();
        assert_eq!(Day02::part_2(&parsed), 4_174_379_265);
    }

    #[test]
    fn test_repeat() {
        assert_eq!(123.repeat_digits(3), Some(123_123_123));
    }
}
