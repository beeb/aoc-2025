use std::collections::BTreeSet;

use winnow::{
    Parser as _, Result,
    ascii::newline,
    combinator::{repeat, separated},
    token::one_of,
};

use crate::days::Day;

const LINES: i32 = 142;

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
    start: Point,
    splitters: BTreeSet<Point>,
    beams: BTreeSet<Point>,
}

impl Manifold {
    fn receives_beam(&self, splitter: Point) -> bool {
        self.beams.contains(&splitter.above())
    }
}

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
            beams: BTreeSet::new(),
        })
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut manifold = input.clone();
        manifold.beams.insert(Point {
            y: 1,
            x: manifold.start.x,
        });
        let mut count_splits = 0;
        for y in 2..LINES {
            for x in 0..LINES {
                let p = Point { y, x };
                if manifold.splitters.contains(&p) && manifold.receives_beam(p) {
                    count_splits += 1;
                    manifold.beams.insert(Point { y: p.y, x: p.x - 1 });
                    manifold.beams.insert(Point { y: p.y, x: p.x + 1 });
                    manifold.beams.insert(Point {
                        y: p.y + 1,
                        x: p.x - 1,
                    });
                    manifold.beams.insert(Point {
                        y: p.y + 1,
                        x: p.x + 1,
                    });
                } else if manifold.beams.contains(&p) {
                    manifold.beams.insert(Point { y: p.y + 1, x: p.x });
                }
            }
        }
        // for y in 0..LINES {
        //     for x in 0..LINES {
        //         let p = Point { y, x };
        //         if p == manifold.start {
        //             print!("S");
        //         } else if manifold.splitters.contains(&p) {
        //             print!("^");
        //         } else if manifold.beams.contains(&p) {
        //             print!("|");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        count_splits
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
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
        assert_eq!(Day07::part_2(&parsed), 14);
    }
}
