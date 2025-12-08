use std::{cmp::Reverse, collections::HashSet};

use itertools::Itertools;
use petgraph::{
    algo::connected_components,
    graph::{NodeIndex, UnGraph},
    visit::DfsPostOrder,
};
use winnow::{
    Parser as _, Result,
    ascii::{dec_uint, newline},
    combinator::separated,
};

use crate::days::Day;

const NUM_CONNECTIONS: usize = if cfg!(test) { 10 } else { 1000 };

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn dist_squared(&self, other: &Point) -> usize {
        let x_dist = self.x.abs_diff(other.x);
        let y_dist = self.y.abs_diff(other.y);
        let z_dist = self.z.abs_diff(other.z);
        x_dist * x_dist + y_dist * y_dist + z_dist * z_dist
    }
}

fn get_all_dist(graph: &UnGraph<Point, usize>) -> Vec<(usize, NodeIndex, NodeIndex)> {
    let mut dist = Vec::new();
    for nodes in graph.node_indices().combinations_with_replacement(2) {
        let [a, b, ..] = nodes.as_slice() else {
            unreachable!();
        };
        if a == b {
            continue;
        }
        let pa = graph.node_weight(*a).unwrap();
        let pb = graph.node_weight(*b).unwrap();
        dist.push((pa.dist_squared(pb), *a, *b));
    }
    dist.sort_unstable_by_key(|(d, _, _)| *d);
    dist
}

pub struct Day08;

fn parse_point(input: &mut &str) -> Result<Point> {
    let (x, _, y, _, z) = (dec_uint, ',', dec_uint, ',', dec_uint).parse_next(input)?;
    Ok(Point { x, y, z })
}

impl Day for Day08 {
    type Input = UnGraph<Point, usize>;

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
        // record all pairs' distances
        let dist = get_all_dist(&graph);
        for (d, a, b) in dist.into_iter().take(NUM_CONNECTIONS) {
            graph.add_edge(a, b, d);
        }
        let mut nets_sizes = Vec::new();
        let mut visited = HashSet::new();
        for node in graph.node_indices() {
            if graph.neighbors(node).count() == 0 {
                continue;
            }
            if visited.contains(&node) {
                continue;
            }
            let mut visitor = DfsPostOrder::new(&graph, node);
            let mut nodes = Vec::new();
            while let Some(node) = visitor.next(&graph) {
                visited.insert(node);
                nodes.push(node);
            }
            nets_sizes.push(Reverse(nodes.len()));
        }
        nets_sizes
            .into_iter()
            .sorted_unstable()
            .take(3)
            .map(|l| l.0)
            .product()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut graph = input.clone();
        // record all pairs' distances
        let dist = get_all_dist(&graph);
        for (d, a, b) in dist {
            graph.add_edge(a, b, d);
            if connected_components(&graph) == 1 {
                return graph.node_weight(a).unwrap().x * graph.node_weight(b).unwrap().x;
            }
        }
        0
    }
}

#[cfg(test)]
#[expect(const_item_mutation)]
mod tests {
    use super::*;

    const INPUT: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part1() {
        let parsed = Day08::parser(&mut INPUT).unwrap();
        assert_eq!(Day08::part_1(&parsed), 40);
    }

    #[test]
    fn test_part2() {
        let parsed = Day08::parser(&mut INPUT).unwrap();
        assert_eq!(Day08::part_2(&parsed), 25_272);
    }
}
