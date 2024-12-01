use std::fs;
use graphrs::{algorithms::community::louvain, Edge, Graph, GraphSpecs};

fn solve(fp: &str) -> usize {
    let contents = fs::read_to_string(fp).expect("a file to open");

    let mut graph = Graph::<&str, ()>::new(GraphSpecs::undirected_create_missing());
    for line in contents.lines() {
        let mut s = line.split_whitespace();
        let name = s.next().unwrap().strip_suffix(':').unwrap();
        for n in s {
            graph.add_edge(Edge::new(name, n)).unwrap();
        }
    }
    let res = louvain::louvain_partitions(&graph, false, Some(0f64), Some(4f64), None).unwrap();
    assert_eq!(res.len(), 1);
    assert_eq!(res[0].len(), 2);
    res[0].iter().map(|n| n.len()).product::<usize>()
}

fn main() {
    let partitions = solve("input.txt");
    println!("{:?}", partitions);
}
