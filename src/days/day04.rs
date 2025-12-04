use std::collections::HashSet;

use winnow::{
    Parser as _, Result,
    ascii::line_ending,
    combinator::{repeat, separated},
    token::one_of,
};

use crate::days::Day;

const DIRS: [(i16, i16); 8] = [
    (0, -1),  // up
    (1, -1),  // top right
    (1, 0),   // right
    (1, 1),   // bottom right
    (0, 1),   // down
    (-1, 1),  // bottom left
    (-1, 0),  // left
    (-1, -1), // top left
];

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Point {
    x: i16,
    y: i16,
}

impl Point {
    #[must_use]
    fn offset(self, dx: i16, dy: i16) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Grid(HashSet<Point>);

impl Grid {
    fn count_neighbours(&self, at: Point) -> usize {
        DIRS.iter()
            .filter(|(dx, dy)| self.0.contains(&at.offset(*dx, *dy)))
            .count()
    }
}

/// Get the positions of all accessible paper rolls
fn get_accessible(grid: &Grid) -> Option<Vec<Point>> {
    let accessible: Vec<_> = grid
        .0
        .iter()
        .filter(|p| grid.count_neighbours(**p) < 4)
        .copied()
        .collect();
    if accessible.is_empty() {
        None
    } else {
        Some(accessible)
    }
}

pub struct Day04;

/// Parse a row of the grid, returning a boolean for each cell in the row to indicate whether a paper roll is present.
fn parse_line(input: &mut &str) -> Result<Vec<bool>> {
    let cells: Vec<_> = repeat(1.., one_of(('.', '@'))).parse_next(input)?;
    Ok(cells.into_iter().map(|c| c == '@').collect())
}

impl Day for Day04 {
    type Input = Grid;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        let lines: Vec<_> = separated(1.., parse_line, line_ending).parse_next(input)?;
        let mut grid = HashSet::new();
        for (y, line) in lines.into_iter().enumerate() {
            for (x, cell) in line.into_iter().enumerate() {
                if !cell {
                    continue;
                }
                grid.insert(Point {
                    x: x as i16,
                    y: y as i16,
                });
            }
        }
        Ok(Grid(grid))
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        get_accessible(input).unwrap().len()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut grid = input.clone();
        let init_rolls = grid.0.len();
        while let Some(accessible) = get_accessible(&grid) {
            for rem in accessible {
                grid.0.remove(&rem);
            }
        }
        init_rolls - grid.0.len()
    }
}
