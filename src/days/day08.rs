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
    /// Calculate the squared distance between two points
    fn dist_squared(&self, other: &Point) -> usize {
        let x_dist = self.x.abs_diff(other.x);
        let y_dist = self.y.abs_diff(other.y);
        let z_dist = self.z.abs_diff(other.z);
        x_dist * x_dist + y_dist * y_dist + z_dist * z_dist
    }
}

/// Calculate the distances between all pairs of junction boxes and return them in ascending distance order
fn get_all_dist_sorted(graph: &UnGraph<Point, ()>) -> Vec<(usize, NodeIndex, NodeIndex)> {
    let mut dist = Vec::new();
    for pair in graph.node_indices().combinations_with_replacement(2) {
        let [a, b, ..] = pair.as_slice() else {
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

/// Parse a point from its coordinates list
fn parse_point(input: &mut &str) -> Result<Point> {
    let (x, _, y, _, z) = (dec_uint, ',', dec_uint, ',', dec_uint).parse_next(input)?;
    Ok(Point { x, y, z })
}

impl Day for Day08 {
    type Input = UnGraph<Point, ()>;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        let points: Vec<_> = separated(1.., parse_point, newline).parse_next(input)?;
        // construct graph with all unconnected nodes
        let mut graph = UnGraph::with_capacity(points.len(), 1000);
        for p in points {
            graph.add_node(p);
        }
        Ok(graph)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut graph = input.clone();
        // compute all pairs' distances and iterate over them in ascending order up to the number
        // of connections required
        let dist = get_all_dist_sorted(&graph);
        for (_, a, b) in dist.into_iter().take(NUM_CONNECTIONS) {
            // link the two junctions boxes together
            graph.add_edge(a, b, ());
        }
        // find all the nets and record their cardinality
        let mut nets_sizes = Vec::new();
        let mut visited = HashSet::new(); // we want to only visit each node once
        for node in graph.node_indices() {
            if graph.neighbors(node).count() == 0 {
                // unconnected nodes can be ignored
                continue;
            }
            if visited.contains(&node) {
                // already visited nodes must be ignored
                continue;
            }
            // visit all connected nodes and record the size of the net
            let mut visitor = DfsPostOrder::new(&graph, node);
            let mut cardinality = 0;
            while let Some(node) = visitor.next(&graph) {
                visited.insert(node);
                cardinality += 1;
            }
            nets_sizes.push(Reverse(cardinality)); // we want to sort in descending order hence the `Reverse`
        }
        // multiply the size of the 3 largest nets
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
        // compute all pairs' distances and iterate over them in ascending order
        // until all nodes are part of a single net
        let dist = get_all_dist_sorted(&graph);
        for (_, a, b) in dist {
            graph.add_edge(a, b, ());
            // small shortcut here, but we could have done it the same way as part 1
            if connected_components(&graph) == 1 {
                // by connecting the last pair, all nodes are connected to each other, we're done!
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
