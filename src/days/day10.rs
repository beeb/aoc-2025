use std::ops::{Deref, DerefMut};

use winnow::{
    Parser, Result,
    ascii::{dec_uint, newline},
    combinator::{delimited, empty, repeat, separated, seq},
    token::one_of,
};

use crate::days::Day;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Lights(u16);

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
pub struct Button(u16);

#[derive(Debug, Clone, Hash)]
pub struct Machine {
    current: Lights,
    target: Lights,
    buttons: Vec<Button>,
    joltages: Vec<u16>,
}

impl From<&[bool]> for Lights {
    fn from(value: &[bool]) -> Self {
        let mut bits = 0;
        for (i, bit) in value.iter().enumerate() {
            if *bit {
                bits |= 1 << i;
            }
        }
        Self(bits)
    }
}

impl Deref for Lights {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Lights {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&[u8]> for Button {
    fn from(value: &[u8]) -> Self {
        let mut bits = 0;
        for bit in value {
            bits |= 1 << bit;
        }
        Self(bits)
    }
}

impl Deref for Button {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Button {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct Day10;

fn parse_lights(input: &mut &str) -> Result<Lights> {
    let lights: Vec<_> = delimited(
        '[',
        repeat(1.., one_of(('.', '#')).map(|c: char| c == '#')),
        ']',
    )
    .parse_next(input)?;
    Ok(lights.as_slice().into())
}

fn parse_button(input: &mut &str) -> Result<Button> {
    let indices: Vec<_> =
        delimited('(', separated(1.., dec_uint::<_, u8, _>, ','), ')').parse_next(input)?;
    Ok(indices.as_slice().into())
}

fn parse_buttons(input: &mut &str) -> Result<Vec<Button>> {
    separated(1.., parse_button, ' ').parse_next(input)
}

fn parse_joltages(input: &mut &str) -> Result<Vec<u16>> {
    delimited('{', separated(1.., dec_uint::<_, u16, _>, ','), '}').parse_next(input)
}

fn parse_machine(input: &mut &str) -> Result<Machine> {
    seq! {Machine {
        current: empty.value(Lights(0)),
        target: parse_lights,
        _: ' ',
        buttons: parse_buttons,
        _: ' ',
        joltages: parse_joltages
    }}
    .parse_next(input)
}

impl Day for Day10 {
    type Input = Vec<Machine>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        separated(1.., parse_machine, newline).parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        dbg!(input);
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
