use std::ops::{Deref, DerefMut};

use itertools::Itertools;
use winnow::{
    Parser as _, Result,
    ascii::{dec_uint, newline},
    combinator::{delimited, repeat, separated, seq},
    token::one_of,
};

use crate::days::Day;

const SHAPE_SIZE: usize = 3;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Shape([[bool; SHAPE_SIZE]; SHAPE_SIZE]);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    width: u8,
    height: u8,
    counts: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Puzzle {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

fn parse_shape(input: &mut &str) -> Result<Shape> {
    let tiles: Vec<Vec<bool>> = separated(
        SHAPE_SIZE,
        repeat::<_, _, Vec<_>, _, _>(SHAPE_SIZE, one_of(('#', '.')).map(|c: char| c == '#')),
        newline,
    )
    .parse_next(input)?;
    Ok(Shape(
        tiles
            .into_iter()
            .map(|row| row.try_into().unwrap())
            .collect_vec()
            .try_into()
            .unwrap(),
    ))
}

fn parse_shape_definition(input: &mut &str) -> Result<Shape> {
    delimited((dec_uint::<_, u8, _>, ':', newline), parse_shape, newline).parse_next(input)
}

fn parse_all_shapes(input: &mut &str) -> Result<Vec<Shape>> {
    separated(1.., parse_shape_definition, newline).parse_next(input)
}

fn parse_dimensions(input: &mut &str) -> Result<(u8, u8)> {
    let (width, _, height) = (dec_uint, 'x', dec_uint).parse_next(input)?;
    Ok((width, height))
}

fn parse_region(input: &mut &str) -> Result<Region> {
    let ((width, height), _, counts) = (
        parse_dimensions,
        ": ",
        separated(1.., dec_uint::<_, u8, _>, ' '),
    )
        .parse_next(input)?;
    Ok(Region {
        width,
        height,
        counts,
    })
}

fn parse_all_regions(input: &mut &str) -> Result<Vec<Region>> {
    separated(1.., parse_region, newline).parse_next(input)
}

pub struct Day12;

impl Day for Day12 {
    type Input = Puzzle;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        seq! { Puzzle{
            shapes: parse_all_shapes,
            _: newline,
            regions: parse_all_regions
        }}
        .parse_next(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        dbg!(&input.shapes);
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}

impl Deref for Shape {
    type Target = [[bool; SHAPE_SIZE]; SHAPE_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Shape {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
