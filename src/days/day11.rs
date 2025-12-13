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

fn get_bridge_nodes(g: &Graph<String, ()>) -> Vec<(usize, NodeIndex)> {
    toposort(g, None)
        .unwrap()
        .into_iter()
        .enumerate()
        .filter(|(_, n)| g.neighbors_directed(*n, Direction::Incoming).count() > 6)
        .collect()
}

fn get_bridge_layers(
    bridge_nodes: Vec<(usize, NodeIndex)>,
    start: NodeIndex,
    end: NodeIndex,
) -> Vec<Vec<NodeIndex>> {
    let mut bridge_layers = Vec::<Vec<_>>::new();
    bridge_layers.push(vec![start]);
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
    bridge_layers.push(vec![end]);
    bridge_layers
}

fn count(
    g: &Graph<String, ()>,
    from: NodeIndex,
    to: NodeIndex,
    via: Option<NodeIndex>,
    counts: &mut HashMap<NodeIndex, usize>,
) {
    let c = if let Some(via) = via {
        let c1 = count_paths(
            from,
            |&n| g.neighbors_directed(n, Direction::Outgoing),
            |&n| n == via,
        );
        let c2 = count_paths(
            via,
            |&n| g.neighbors_directed(n, Direction::Outgoing),
            |&n| n == to,
        );
        // paths from `from` to `end` going through the via node
        c1 * c2
    } else {
        count_paths(
            from,
            |&n| g.neighbors_directed(n, Direction::Outgoing),
            |&n| n == to,
        )
    };
    // total paths reaching the `to` node
    let c = c * counts.get(&from).unwrap_or(&1);
    counts
        .entry(to)
        .and_modify(|num| {
            *num += c;
        })
        .or_insert(c);
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
        let svr = *input.nodes.get("svr").unwrap();
        let out = *input.nodes.get("out").unwrap();
        let dac = *input.nodes.get("dac").unwrap();
        let fft = *input.nodes.get("fft").unwrap();
        let bridge_nodes = get_bridge_nodes(&input.graph);
        let bridge_layers = get_bridge_layers(bridge_nodes, svr, out);
        let mut num_paths = HashMap::<NodeIndex, usize>::new();
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
                        count(&input.graph, *start, *end, Some(dac), &mut num_paths);
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
                        count(&input.graph, *start, *end, Some(fft), &mut num_paths);
                    }
                }
                continue;
            }
            for start in start_layer {
                for end in end_layer {
                    count(&input.graph, *start, *end, None, &mut num_paths);
                }
            }
        }
        *num_paths.get(&out).unwrap()
    }
}
