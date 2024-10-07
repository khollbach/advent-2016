use std::collections::HashMap;

use crate::{Edge, Graph, Input, Node};

impl Graph {
    pub fn from_lines(lines: impl Iterator<Item = String>) -> Self {
        let mut this = Self {
            inputs: vec![],
            nodes: HashMap::new(),
        };

        for l in lines {
            if l.starts_with("value") {
                let (value, node) = l
                    .strip_prefix("value ")
                    .unwrap()
                    .split_once(" goes to bot ")
                    .unwrap();
                let value = value.parse().unwrap();
                let node = node.parse().unwrap();
                this.inputs.push(Input { node, value });
            } else {
                let (node, rest) = l
                    .strip_prefix("bot ")
                    .unwrap()
                    .split_once(" gives low to ")
                    .unwrap();
                let (e1, e2) = rest.split_once(" and high to ").unwrap();

                let node = node.parse().unwrap();
                let e1 = parse_edge(e1);
                let e2 = parse_edge(e2);

                let old = this.nodes.insert(node, Node { edges: [e1, e2] });
                assert!(old.is_none());
            }
        }

        this
    }
}

fn parse_edge(s: &str) -> Edge {
    let (type_, index) = s.split_once(' ').unwrap();
    let index = index.parse().unwrap();
    match type_ {
        "bot" => Edge::Node(index),
        "output" => Edge::Output(index),
        _ => panic!("bad edge type: {type_}"),
    }
}
