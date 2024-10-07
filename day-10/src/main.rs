use std::{collections::HashMap, io};

mod parse;

fn main() {
    let lines = io::stdin().lines().map(Result::unwrap);
    let g = Graph::from_lines(lines);

    let ans = g.part_1();
    println!("{ans}");

    let ans = g.part_2();
    println!("{ans}");
}

struct Graph {
    inputs: Vec<Input>,
    nodes: HashMap<usize, Node>,
}

struct Node {
    /// [low, high]
    edges: [Edge; 2],
}

enum Edge {
    Node(usize),
    Output(usize),
}

#[derive(Clone)]
struct Input {
    node: usize,
    value: u32,
}

impl Graph {
    /// Return the index of the node that receives values 17 and 61.
    fn part_1(&self) -> usize {
        let ret = self.simulate();
        for (node, mut inputs) in ret.inputs_received {
            inputs.sort();
            if inputs == [17, 61] {
                return node;
            }
        }

        panic!("(17, 61) not found");
    }

    /// Return the product of outputs 0, 1, and 2.
    fn part_2(&self) -> u32 {
        let ret = self.simulate();
        ret.outputs[&0] * ret.outputs[&1] * ret.outputs[&2]
    }

    fn simulate(&self) -> SimulateRet {
        let mut inputs_received = HashMap::<usize, Vec<u32>>::new();
        let mut outputs = HashMap::new();

        let mut triggered_edges = self.inputs.clone();

        while let Some(e) = triggered_edges.pop() {
            let recv = inputs_received.entry(e.node).or_default();
            recv.push(e.value);

            if recv.len() >= 2 {
                assert_eq!(recv.len(), 2);
                recv.sort();

                for i in 0..2 {
                    let value = recv[i];
                    match self.nodes[&e.node].edges[i] {
                        Edge::Node(node) => {
                            triggered_edges.push(Input { node, value });
                        }
                        Edge::Output(index) => {
                            outputs.insert(index, value);
                        }
                    }
                }
            }
        }

        SimulateRet {
            inputs_received,
            outputs,
        }
    }
}

struct SimulateRet {
    /// The input values to each node.
    inputs_received: HashMap<usize, Vec<u32>>,
    outputs: HashMap<usize, u32>,
}
