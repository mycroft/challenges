use std::fs::read_to_string;
use std::collections::HashMap;

use pathfinding::prelude::bfs;

#[derive(Debug, PartialEq)]
pub struct Valve {
    name: String,
    pub rate: isize,
    next: Vec<String>,
    pub destinations: HashMap<String, isize>,
}

fn successors(valves: &HashMap<String,Valve>, current: &String) -> Vec<String> {
    for valve in valves {
        if &valve.1.name == current {
            return valve.1.next.clone();
        }
    }

    Vec::new()
}

pub fn parse(fp: &str) -> HashMap<String, Valve> {
    let contents = read_to_string(fp).unwrap();
    let mut valves = HashMap::new();

    let lines = contents.lines().collect::<Vec<&str>>();
    let mut pipes = Vec::new();

    for line in lines {
        if line == "" {
            break;
        }

        let (name, rate, _, _, _, next) = scan_fmt!(
            line,
            "Valve {} has flow rate={}; {} {} to {} {[A-Z, ]}",
            String, isize, String, String, String, String
        ).unwrap();

        pipes.push(name.clone());

        valves.insert(
            name.clone(),
            Valve{
                name: name.clone(),
                rate: rate,
                next: next.split(", ").map(|x| x.to_string()).collect::<Vec<String>>(),
                destinations: HashMap::new(),
            }
        );
    }

    for v in 0..pipes.len() {
        let current_pipe = &pipes[v];
        for o in 0..pipes.len() {
            let other_pipe = &pipes[o];
            if v == o {
                continue;
            }

            if valves.get(other_pipe).unwrap().rate == 0 {
                continue;
            }

            let path = bfs(
                current_pipe,
                |p| successors(&valves, p),
                |p| p == other_pipe
            ).unwrap();

            valves.get_mut(current_pipe).unwrap().destinations.insert(
                other_pipe.clone(),
                path.len() as isize - 1
            );
        }
    }

    valves
}
