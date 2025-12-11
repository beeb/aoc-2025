use std::collections::HashMap;

use petgraph::{Graph, graph::NodeIndex};
use winnow::{
    Parser as _, Result,
    ascii::{alpha1, newline},
    combinator::separated,
};

use crate::days::Day;

#[derive(Debug, Clone)]
pub struct Server {
    nodes: HashMap<String, NodeIndex>,
    graph: Graph<(), ()>,
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
            let idx = graph.add_node(());
            nodes.insert((*node).to_string(), idx);
        }
        let out = graph.add_node(());
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
        dbg!(input);
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
