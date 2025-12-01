use std::ops::{AddAssign, RemAssign, SubAssign};

use winnow::{
    Parser as _, Result,
    ascii::{dec_uint, line_ending},
    combinator::{separated, seq},
    token::one_of,
};

use crate::days::Day;

const DIAL_SIZE: i32 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Move {
    dir: Direction,
    clicks: u16,
}

#[derive(Debug, Clone)]
struct Dial {
    pos: i32,
}

impl Default for Dial {
    fn default() -> Self {
        Self { pos: 50 }
    }
}

impl Dial {
    fn turn(&mut self, mov: &Move) {
        match mov.dir {
            Direction::Left => self.pos.sub_assign(mov.clicks as i32),
            Direction::Right => self.pos.add_assign(mov.clicks as i32),
        }
        self.pos.rem_assign(DIAL_SIZE);
    }
}

pub struct Day01;

fn parse_dir(input: &mut &str) -> Result<Direction> {
    one_of(['L', 'R'])
        .map(|c: char| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .parse_next(input)
}

fn parse_move(input: &mut &str) -> Result<Move> {
    seq! { Move {
        dir: parse_dir,
        clicks: dec_uint
    }}
    .parse_next(input)
}

impl Day for Day01 {
    type Input = Vec<Move>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        separated(1.., parse_move, line_ending).parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut dial = Dial::default();
        let mut count = 0;
        for mov in input {
            dial.turn(mov);
            if dial.pos == 0 {
                count += 1;
            }
        }
        count
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}

#[cfg(test)]
#[allow(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part1() {
        let parsed = Day01::parser(&mut INPUT).unwrap();
        assert_eq!(Day01::part_1(&parsed), 3);
    }
}
