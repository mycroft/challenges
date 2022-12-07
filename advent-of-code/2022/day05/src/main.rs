use std::fs::read_to_string;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
struct Move {
    num: usize,
    from: usize,
    to: usize
}

#[derive(Debug, Clone)]
struct Error;

fn parse_initial_state(fp: &str) -> Result<(Vec<VecDeque<char>>, Vec<Move>), Error> {
    let contents = read_to_string(fp).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    let cols = (lines[0].len() + 1) / 4;

    for _ in 0..cols {
        stacks.push(VecDeque::new());
    }

    for line in &lines {
        if line.chars().nth(1).unwrap() == '1' {
            break;
        }

        for idx in 0..cols {
            let c = line.chars().nth(1 + (idx * 4)).unwrap();
            if c == ' ' {
                continue;
            }
            stacks[idx].push_front(c);
        }
    }

    let moves: Vec<Move> = lines.into_iter()
        .filter(|x| x.starts_with("move"))
        .map(|x| {
            let parts: Vec<&str> = x.split(' ').collect();
            Move {
                num: parts[1].parse::<usize>().unwrap(),
                from: parts[3].parse::<usize>().unwrap(),
                to: parts[5].parse::<usize>().unwrap(),
            }
        })
        .collect();

    Ok((stacks, moves))
}


fn main() {
    let state = parse_initial_state("input.txt").unwrap();

    let mut stacks = state.0.clone();
    let moves = state.1;

    for mov in &moves {
        for _ in 0..mov.num {
            let el = stacks[mov.from-1].pop_back().unwrap();
            stacks[mov.to-1].push_back(el);
        }
    }

    let res0: String = stacks.iter()
        .map(|x| x[x.len() - 1])
        .collect();


    let mut stacks = state.0;

    for mov in &moves {
        let mut smallstack: VecDeque<char> = VecDeque::new();
        for _ in 0..mov.num {
            let el =  stacks[mov.from-1].pop_back().unwrap();
            smallstack.push_front(el);
        }
        stacks[mov.to-1].append(&mut smallstack);
    }

    let res1: String = stacks.iter()
        .map(|x| x[x.len() - 1])
        .collect();

    println!("#1 {res0}");
    println!("#2 {res1}");
}
