use std::ops::RangeInclusive;

use winnow::{
    Parser as _, Result,
    ascii::{dec_uint, newline},
    combinator::{separated, separated_pair},
};

use crate::days::Day;

pub struct Day05;

/// Parse a range from the input (two unsigned integers separated by a dash).
fn parse_range(input: &mut &str) -> Result<RangeInclusive<usize>> {
    let (start, end) = separated_pair(dec_uint, '-', dec_uint).parse_next(input)?;
    Ok(start..=end)
}

fn parse_ranges(input: &mut &str) -> Result<Vec<RangeInclusive<usize>>> {
    separated(1.., parse_range, newline).parse_next(input)
}

impl Day for Day05 {
    type Input = (Vec<RangeInclusive<usize>>, Vec<usize>);

    fn parser(input: &mut &str) -> Result<Self::Input> {
        separated_pair(
            parse_ranges,
            (newline, newline),
            separated(1.., dec_uint::<_, usize, _>, newline),
        )
        .parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let (ranges, ingredients) = input;
        ingredients
            .iter()
            .filter(|i| ranges.iter().any(|r| r.contains(i)))
            .count()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let (ranges, _) = input;
        let mut unmerged = ranges.clone();
        unmerged.sort_unstable_by_key(|r| *r.start());
        loop {
            let mut merged = Vec::<RangeInclusive<usize>>::new();
            for range in &unmerged {
                if let Some(into) = merged
                    .iter_mut()
                    .find(|r| r.contains(range.start()) || r.contains(range.end()))
                {
                    *into = *into.start().min(range.start())..=*into.end().max(range.end());
                } else {
                    merged.push(range.clone());
                }
            }
            if merged.len() == unmerged.len() {
                return merged.into_iter().map(Iterator::count).sum();
            }
            unmerged = merged;
        }
    }
}

#[cfg(test)]
#[expect(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        let parsed = Day05::parser(&mut INPUT).unwrap();
        assert_eq!(Day05::part_1(&parsed), 3);
    }

    #[test]
    fn test_part2() {
        let parsed = Day05::parser(&mut INPUT).unwrap();
        assert_eq!(Day05::part_2(&parsed), 14);
    }
}
