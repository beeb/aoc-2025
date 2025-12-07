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
    /// Gets the position directly above self
    fn above(self) -> Point {
        Point {
            y: self.y - 1,
            x: self.x,
        }
    }

    /// Gets the position directly below self
    fn below(self) -> Point {
        Point {
            y: self.y + 1,
            x: self.x,
        }
    }

    /// Gets the position directly to the left of self
    fn left(self) -> Point {
        Point {
            y: self.y,
            x: self.x - 1,
        }
    }

    /// Gets the position directly to the right of self
    fn right(self) -> Point {
        Point {
            y: self.y,
            x: self.x + 1,
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

    /// Project the source beam into the manifold, keeping track of how many possible paths go through each position
    /// and recording the total number of splits (the return value).
    fn project_beams(&mut self) -> usize {
        self.beams.insert(
            Point {
                y: 1,
                x: self.start.x,
            },
            1, // 1 path goes through the initial beam
        );
        let mut n_splits = 0; // counter for total splits
        for y in 2..LINES {
            let range = Point { y, x: 0 }..Point { y, x: COLUMNS };
            if y % 2 == 0 {
                // check all splitters on this line (only even lines have splitters)
                for p in self.splitters.range(range.clone()) {
                    let Some(n) = self.receives_beam(*p) else {
                        // the splitter doesn't receive any beam as input
                        continue;
                    };
                    // the splitter receives `n` possible beams as input
                    n_splits += 1; // count the splitter
                    // split the beam, indicating that `n` paths are propagated to either side
                    // and potentially summing with any existing paths count at that location
                    let n_left = *self
                        .beams
                        .entry(p.left())
                        .and_modify(|p| *p += n)
                        .or_insert(n);
                    let n_right = *self
                        .beams
                        .entry(p.right())
                        .and_modify(|p| *p += n)
                        .or_insert(n);
                    // propagate the beams down to the next line which doesn't contain splitters
                    // keeping the same number of paths
                    self.beams.insert(p.left().below(), n_left);
                    self.beams.insert(p.right().below(), n_right);
                }
            }
            // check all beams on this line (including the ones we just split)
            for (p, n) in self
                .beams
                .range(range)
                .map(|(p, n)| (*p, *n))
                .collect::<Vec<_>>()
            {
                // propagate beams down keeping the same count
                // but only if there isn't a splitter there
                let below = p.below();
                if !self.splitters.contains(&below) {
                    self.beams.insert(below, n);
                }
            }
        }
        n_splits
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
        // sum up all the paths reaching the last line
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
