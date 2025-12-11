use std::collections::HashMap;

use itertools::Itertools;
use pathfinding::prelude::count_paths;
use petgraph::{
    Direction, Graph,
    algo::{has_path_connecting, toposort},
    graph::NodeIndex,
    visit::EdgeRef as _,
};
use winnow::{
    Parser as _, Result,
    ascii::{alpha1, newline},
    combinator::separated,
};

use crate::days::Day;

#[derive(Debug, Clone)]
pub struct Server {
    nodes: HashMap<String, NodeIndex>,
    graph: Graph<String, ()>,
}

pub struct Day11;

fn parse_outputs<'a>(input: &mut &'a str) -> Result<Vec<&'a str>> {
    separated(1.., alpha1, ' ').parse_next(input)
}

fn parse_device<'a>(input: &mut &'a str) -> Result<(&'a str, Vec<&'a str>)> {
    let (node, _, outputs) = (alpha1, ": ", parse_outputs).parse_next(input)?;
    Ok((node, outputs))
}

impl Day for Day11 {
    type Input = Server;

    fn parser(input: &mut &str) -> Result<Self::Input> {
        let devices: Vec<_> = separated(1.., parse_device, newline).parse_next(input)?;
        let mut graph = Graph::new();
        let mut nodes = HashMap::new();
        for (node, _) in &devices {
            let idx = graph.add_node((*node).to_string());
            nodes.insert((*node).to_string(), idx);
        }
        let out = graph.add_node("out".to_string());
        nodes.insert("out".to_string(), out);
        for (node, outputs) in &devices {
            for output in outputs {
                graph.add_edge(*nodes.get(*node).unwrap(), *nodes.get(*output).unwrap(), ());
            }
        }
        Ok(Server { nodes, graph })
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let end = input.nodes.get("out").unwrap();
        count_paths(
            *input.nodes.get("you").unwrap(),
            |&n| input.graph.neighbors_directed(n, Direction::Outgoing),
            |n| n == end,
        )
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let out = *input.nodes.get("out").unwrap();
        let bridge_nodes = toposort(&input.graph, None)
            .unwrap()
            .into_iter()
            .enumerate()
            .filter(|(_, n)| {
                input
                    .graph
                    .neighbors_directed(*n, Direction::Incoming)
                    .count()
                    > 6
            })
            .collect_vec();
        let mut bridge_layers = Vec::<Vec<_>>::new();
        bridge_layers.push(vec![*input.nodes.get("svr").unwrap()]);
        for (_, chunk) in &bridge_nodes
            .into_iter()
            .tuple_windows()
            .chunk_by(|((i, _), (j, _))| (*i).abs_diff(*j) <= 20)
        {
            let nodes = chunk.into_iter().map(|((_, n), _)| n).collect_vec();
            if nodes.len() == 1 {
                bridge_layers
                    .last_mut()
                    .unwrap()
                    .push(*nodes.first().unwrap());
            } else {
                bridge_layers.push(nodes);
            }
        }
        bridge_layers.push(vec![out]);
        let mut num_paths = HashMap::<NodeIndex, usize>::new();
        let dac = *input.nodes.get("dac").unwrap();
        let fft = *input.nodes.get("fft").unwrap();
        for (start_layer, end_layer) in bridge_layers.iter().tuple_windows() {
            // disconnect output
            let mut g = input.graph.clone();
            for n in end_layer {
                let edges = g
                    .edges_directed(*n, Direction::Outgoing)
                    .map(|e| e.id())
                    .collect_vec();
                for edge in edges {
                    g.remove_edge(edge);
                }
            }
            if start_layer
                .iter()
                .any(|n| has_path_connecting(&g, *n, dac, None))
            {
                // special group with dac, only count paths going through dac
                for start in start_layer {
                    for end in end_layer {
                        let c1 = count_paths(
                            *start,
                            |&n| input.graph.neighbors_directed(n, Direction::Outgoing),
                            |&n| n == dac,
                        );
                        let c2 = count_paths(
                            dac,
                            |&n| input.graph.neighbors_directed(n, Direction::Outgoing),
                            |n| n == end,
                        );
                        // paths from start to end going through dac
                        let c = c1 * c2;
                        // total paths reaching end through dac
                        let c = c * num_paths.get(start).unwrap_or(&1);
                        num_paths
                            .entry(*end)
                            .and_modify(|num| {
                                *num += c;
                            })
                            .or_insert(c);
                    }
                }
                continue;
            } else if start_layer
                .iter()
                .any(|n| has_path_connecting(&g, *n, fft, None))
            {
                // special group with fft, only count paths going through dac
                for start in start_layer {
                    for end in end_layer {
                        let c1 = count_paths(
                            *start,
                            |&n| input.graph.neighbors_directed(n, Direction::Outgoing),
                            |&n| n == fft,
                        );
                        let c2 = count_paths(
                            fft,
                            |&n| input.graph.neighbors_directed(n, Direction::Outgoing),
                            |n| n == end,
                        );
                        // paths from start to end going through fft
                        let c = c1 * c2;
                        // total paths reaching end through fft
                        let c = c * num_paths.get(start).unwrap_or(&1);
                        num_paths
                            .entry(*end)
                            .and_modify(|num| {
                                *num += c;
                            })
                            .or_insert(c);
                    }
                }
                continue;
            }
            for start in start_layer {
                for end in end_layer {
                    // paths from start to end
                    let c = count_paths(
                        *start,
                        |&n| input.graph.neighbors_directed(n, Direction::Outgoing),
                        |n| n == end,
                    );
                    // total paths reaching end
                    let c = c * num_paths.get(start).unwrap_or(&1);
                    num_paths
                        .entry(*end)
                        .and_modify(|num| {
                            *num += c;
                        })
                        .or_insert(c);
                }
            }
        }
        *num_paths.get(&out).unwrap()
    }
}
