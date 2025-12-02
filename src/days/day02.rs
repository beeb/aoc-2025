use std::ops::RangeInclusive;

use winnow::{
    Parser as _, Result,
    ascii::dec_uint,
    combinator::{separated, separated_pair},
};

use crate::days::Day;

const MAX_DIGITS: usize = 10;

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

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
