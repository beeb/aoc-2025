use std::ops::{AddAssign, SubAssign};

use winnow::{
    Parser as _, Result,
    ascii::{dec_uint, line_ending},
    combinator::{separated, seq},
    token::one_of,
};

use crate::days::Day;

/// The number of positions on the safe's dial
const DIAL_SIZE: i32 = 100;

/// The possible directions to turn the safe's dial
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

/// A move (turn) of the dial, in either direction for a given number of clicks.
#[derive(Debug, Clone)]
pub struct Move {
    dir: Direction,
    clicks: u16,
}

/// The dial state
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
    /// Turn the dial according to the provided move.
    ///
    /// The returned value is the number of times the dial reaches the zero position during the move.
    fn turn(&mut self, mov: &Move) -> usize {
        let prev = self.pos; // previous value
        match mov.dir {
            Direction::Left => self.pos.sub_assign(i32::from(mov.clicks)),
            Direction::Right => self.pos.add_assign(i32::from(mov.clicks)),
        }
        let temp = self.pos; // temporary value before clamping to the dial's range
        let xings = (temp / DIAL_SIZE).unsigned_abs() as usize; // calculate the number of zero crossings
        self.pos = self.pos.rem_euclid(DIAL_SIZE);
        // if going from positive to negative, we need to add 1 to account for the first zero crossing
        // not needed if we start from exactly zero
        if prev > 0 && temp <= 0 {
            xings + 1
        } else {
            xings
        }
    }
}

pub struct Day01;

/// Parse a direction character
fn parse_dir(input: &mut &str) -> Result<Direction> {
    one_of(['L', 'R'])
        .map(|c: char| match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => unreachable!(),
        })
        .parse_next(input)
}

/// Parse a move (input line)
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
        input.iter().fold(0, |acc, mov| {
            dial.turn(mov);
            if dial.pos == 0 { acc + 1 } else { acc }
        })
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut dial = Dial::default();
        input.iter().map(|m| dial.turn(m)).sum()
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

    #[test]
    fn test_part2() {
        let parsed = Day01::parser(&mut INPUT).unwrap();
        assert_eq!(Day01::part_2(&parsed), 6);
    }
}
