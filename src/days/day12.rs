use itertools::{Itertools, repeat_n};
use winnow::{
    Parser as _, Result,
    ascii::{dec_uint, newline},
    combinator::{delimited, repeat, separated, seq},
    token::one_of,
};

use crate::days::Day;

const SHAPE_SIZE: usize = 3;

/// A shape rotation
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Rotation {
    QuaterTurn,
    HalfTurn,
    ThreeQuarter,
}

/// A shape to place in the region
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Shape([[bool; SHAPE_SIZE]; SHAPE_SIZE]);

impl Shape {
    /// Get the area of a shape (how many tiles are occupied by it)
    fn area(&self) -> usize {
        self.0.iter().flatten().filter(|k| **k).count()
    }

    /// Check whether we can place this shape at the (x, y) coordinate in the grid
    fn can_place(&self, grid: &[Vec<bool>], x: usize, y: usize) -> bool {
        for dy in 0..SHAPE_SIZE {
            for dx in 0..SHAPE_SIZE {
                if !self.0[dy][dx] {
                    continue;
                }
                let grid_x = x + dx;
                let grid_y = y + dy;
                if grid_y >= grid.len() || grid_x >= grid[0].len() {
                    return false;
                }
                if grid[grid_y][grid_x] {
                    return false;
                }
            }
        }
        true
    }

    /// Place this shape on the grid at (x, y)
    fn place(self, grid: &mut [Vec<bool>], x: usize, y: usize) {
        for dy in 0..SHAPE_SIZE {
            for dx in 0..SHAPE_SIZE {
                if self.0[dy][dx] {
                    grid[y + dy][x + dx] = true;
                }
            }
        }
    }

    /// Remove this shape from (x, y)
    fn remove(self, grid: &mut [Vec<bool>], x: usize, y: usize) {
        for dy in 0..SHAPE_SIZE {
            for dx in 0..SHAPE_SIZE {
                if self.0[dy][dx] {
                    grid[y + dy][x + dx] = false;
                }
            }
        }
    }

    /// Rotate 90 degrees
    fn rotate(self) -> Self {
        let mut out = [[false; SHAPE_SIZE]; SHAPE_SIZE];
        for y in 0..SHAPE_SIZE {
            for (x, row) in out.iter_mut().enumerate() {
                row[SHAPE_SIZE - 1 - y] = self.0[y][x];
            }
        }
        Shape(out)
    }

    /// Rotate by the required amount
    fn rotate_to(self, rot: Rotation) -> Self {
        match rot {
            Rotation::QuaterTurn => self.rotate(),
            Rotation::HalfTurn => self.rotate().rotate(),
            Rotation::ThreeQuarter => self.rotate().rotate().rotate(),
        }
    }

    /// Mirror the shape along its x-axis
    fn mirror(mut self) -> Self {
        for row in &mut self.0 {
            row.reverse();
        }
        self
    }
}

impl Ord for Shape {
    /// Order shapes by their area
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.area().cmp(&other.area())
    }
}

impl PartialOrd for Shape {
    /// Order shapes by their area
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// A region where to place presents under the tree
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    width: usize,
    height: usize,
    counts: Vec<usize>,
}

/// Today's puzzle input
#[derive(Debug, Clone)]
pub struct Puzzle {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

/// Recursively try to pack all shapes into the region's grid
fn pack(mut shapes_to_place: Vec<Shape>, mut grid: Vec<Vec<bool>>) -> bool {
    let Some(shape) = shapes_to_place.pop() else {
        // all shapes are placed, success!
        return true;
    };
    // all possible orientations
    let mut orientations = vec![
        shape,
        shape.rotate_to(Rotation::QuaterTurn),
        shape.rotate_to(Rotation::HalfTurn),
        shape.rotate_to(Rotation::ThreeQuarter),
        shape.mirror(),
        shape.rotate_to(Rotation::QuaterTurn).mirror(),
        shape.rotate_to(Rotation::HalfTurn).mirror(),
        shape.rotate_to(Rotation::ThreeQuarter).mirror(),
    ];
    // some might be duplicates (rotational symmetric or axisymmetrical parts), we should remove them
    orientations.sort_unstable_by_key(|s| s.0);
    orientations.dedup();
    for s in orientations {
        for x in 0..grid[0].len() {
            for y in 0..grid.len() {
                // try to place on the grid, at the first available spot
                if !s.can_place(&grid, x, y) {
                    continue;
                }
                s.place(&mut grid, x, y);
                // try to place all other parts recursively
                if pack(shapes_to_place.clone(), grid.clone()) {
                    // all shapes are placed, success!
                    return true;
                }
                // we couldn't fit all the other parts, we have to continue trying
                s.remove(&mut grid, x, y); // remove the shape we just placed to try other orientations
            }
        }
    }
    false
}

pub struct Day12;

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

fn parse_dimensions(input: &mut &str) -> Result<(usize, usize)> {
    let (width, _, height) = (dec_uint, 'x', dec_uint).parse_next(input)?;
    Ok((width, height))
}

fn parse_region(input: &mut &str) -> Result<Region> {
    let ((width, height), _, counts) = (
        parse_dimensions,
        ": ",
        separated(1.., dec_uint::<_, usize, _>, ' '),
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
        let mut count = 0;
        for region in &input.regions {
            let mut shapes_to_place = Vec::new();
            // search with largest shapes first (= put them at the end of the list)
            for (count, shape) in region
                .counts
                .iter()
                .zip(input.shapes.iter())
                .sorted_unstable_by(|(_, a), (_, b)| a.cmp(b))
            {
                shapes_to_place.extend(repeat_n(*shape, *count));
            }

            // necessary optimization, some regions are too small to fit all shapes
            let total_area: usize = shapes_to_place.iter().map(Shape::area).sum();
            if total_area > region.width * region.height {
                continue; // impossible to pack, not enough space
            }

            if pack(
                shapes_to_place,
                vec![vec![false; region.width]; region.height],
            ) {
                count += 1;
            }
        }
        count
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        0
    }
}
