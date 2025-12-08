use itertools::Itertools;
use petgraph::graph::UnGraph;
use winnow::{
    Parser as _, Result,
    ascii::{dec_uint, newline},
    combinator::separated,
};

use crate::days::Day;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn dist_squared(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

pub struct Day08;

fn parse_point(input: &mut &str) -> Result<Point> {
    let (x, _, y, _, z) = (dec_uint, ',', dec_uint, ',', dec_uint).parse_next(input)?;
    Ok(Point { x, y, z })
}

impl Day for Day08 {
    type Input = UnGraph<Point, ()>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        let points: Vec<_> = separated(1.., parse_point, newline).parse_next(input)?;
        let mut graph = UnGraph::with_capacity(points.len(), 1000);
        for p in points {
            graph.add_node(p);
        }
        Ok(graph)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut graph = input.clone();
        for nodes in graph.node_indices().combinations(2) {
            let [a, b, ..] = nodes.as_slice() else {
                unreachable!();
            };
        }
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
