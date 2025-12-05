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

/// Parse a list of ranges (one per line).
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
        // sorting the ranges by start ID so that we can merge them easily in one pass
        let mut ranges = ranges.clone();
        ranges.sort_unstable_by_key(|r| *r.start());
        // accumulate the merged ranges into a new list
        let mut merged = Vec::<RangeInclusive<usize>>::new();
        for range in &ranges {
            // check if the current range intersects with the last one that was pushed to the merged list
            if let Some(into) = merged.last_mut()
                && into.contains(range.start())
            {
                // update the `end` of the last range in the merged list
                *into = *into.start()..=*into.end().max(range.end());
            } else {
                // this range does not intersect, no merging needed
                merged.push(range.clone());
            }
        }
        merged.into_iter().map(Iterator::count).sum()
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
