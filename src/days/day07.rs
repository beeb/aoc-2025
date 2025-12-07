use std::collections::{BTreeMap, BTreeSet};

use winnow::{
    Parser as _, Result,
    ascii::newline,
    combinator::{repeat, separated},
    token::one_of,
};

use crate::days::Day;

const LINES: i32 = 142;
const COLUMNS: i32 = 141;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    y: i32,
    x: i32,
}

impl Point {
    fn above(self) -> Point {
        Point {
            y: self.y - 1,
            x: self.x,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Manifold {
    /// Start point
    start: Point,
    /// Positions of all splitters
    splitters: BTreeSet<Point>,
    /// For each beam position, how many paths go through here
    beams: BTreeMap<Point, usize>,
}

impl Manifold {
    /// Return whether a beam enter the splitter, and how many paths go through that position
    fn receives_beam(&self, splitter: Point) -> Option<usize> {
        self.beams.get(&splitter.above()).copied()
    }

    fn project_beams(&mut self) -> usize {
        self.beams.insert(
            Point {
                y: 1,
                x: self.start.x,
            },
            1, // 1 path goes through the initial beam
        );
        let mut count_splits = 0;
        for y in 2..LINES {
            for x in 0..COLUMNS {
                let p = Point { y, x };
                if self.splitters.contains(&p)
                    && let Some(paths) = self.receives_beam(p)
                {
                    count_splits += 1;
                    let left = *self
                        .beams
                        .entry(Point { y: p.y, x: p.x - 1 })
                        .and_modify(|p| *p += paths)
                        .or_insert(paths);
                    let right = *self
                        .beams
                        .entry(Point { y: p.y, x: p.x + 1 })
                        .and_modify(|p| *p += paths)
                        .or_insert(paths);
                    self.beams.insert(
                        Point {
                            y: p.y + 1,
                            x: p.x - 1,
                        },
                        left,
                    );
                    self.beams.insert(
                        Point {
                            y: p.y + 1,
                            x: p.x + 1,
                        },
                        right,
                    );
                } else if let Some(paths) = self.beams.get(&p) {
                    self.beams.insert(Point { y: p.y + 1, x: p.x }, *paths);
                }
            }
        }
        count_splits
    }
}

impl Manifold {}

pub struct Day07;

fn parse_line(input: &mut &str) -> Result<Vec<char>> {
    repeat(1.., one_of(('.', 'S', '^'))).parse_next(input)
}

fn parse_grid(input: &mut &str) -> Result<Vec<Vec<char>>> {
    separated(1.., parse_line, newline).parse_next(input)
}

impl Day for Day07 {
    type Input = Manifold;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        let grid = parse_grid.parse_next(input)?;
        let start_x = grid
            .first()
            .unwrap()
            .iter()
            .position(|c| c == &'S')
            .unwrap();
        let mut splitters = BTreeSet::new();
        for (y, line) in grid.iter().enumerate().skip(1) {
            for (x, cell) in line.iter().enumerate() {
                if cell == &'^' {
                    splitters.insert(Point {
                        y: y as i32,
                        x: x as i32,
                    });
                }
            }
        }
        Ok(Manifold {
            start: Point {
                y: 0,
                x: start_x as i32,
            },
            splitters,
            beams: BTreeMap::new(),
        })
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut manifold = input.clone();
        manifold.project_beams()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut manifold = input.clone();
        manifold.project_beams();
        let last_line = Point { y: LINES - 1, x: 0 }..Point {
            y: LINES - 1,
            x: COLUMNS,
        };
        manifold
            .beams
            .range(last_line)
            .map(|(_, paths)| *paths)
            .sum()
    }
}

#[cfg(test)]
#[expect(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        let parsed = Day07::parser(&mut INPUT).unwrap();
        assert_eq!(Day07::part_1(&parsed), 21);
    }

    #[test]
    fn test_part2() {
        let parsed = Day07::parser(&mut INPUT).unwrap();
        assert_eq!(Day07::part_2(&parsed), 40);
    }
}
