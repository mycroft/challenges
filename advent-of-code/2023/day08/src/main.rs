use std::fs;
use std::collections::HashMap;

#[macro_use] extern crate scan_fmt;

pub fn gcd(mut n: u128, mut m: u128) -> u128 {
    assert!(n != 0 && m != 0);
    while m != 0 {
      if m < n {
        std::mem::swap(&mut m, &mut n);
      }
      m %= n;
    }
    n
  }
  
fn find_number_of_moves(moves: &String, nodes: &HashMap<String, (String, String)>, starting_node: String, step2: bool) -> usize {
    let mut moves_number = 0;
    let mut current_node = starting_node;

    loop {
        if current_node == *"ZZZ" {
            break;
        }

        if current_node.ends_with('Z') && step2 {
            break;
        }

        let direction = moves.chars().nth(moves_number % moves.len()).unwrap();

        match direction {
            'L' => {
                current_node = nodes.get(&current_node).unwrap().0.clone();
            },
            'R' => {
                current_node = nodes.get(&current_node).unwrap().1.clone();
            },
            _ => unreachable!(),
        }

        moves_number += 1;
    }

    moves_number
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("a file to open");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut moves = String::new();
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        if moves.is_empty() {
            moves = line.to_string();
            continue;
        } else if line.is_empty() {
            continue;
        }

        let (from, left, right) = scan_fmt!(line, "{} = ({}, {})", String, String, String).unwrap();

        nodes.insert(from, (left, right));
    }

    let moves_number = find_number_of_moves(&moves, &nodes, "AAA".to_string(), false);

    println!("#1 {}", moves_number); // 14257

    let starting_nodes : Vec<&str> = [
        "AAA", "TTA", "KJA", "BGA", "LTA", "NJA",
    ].into();

    let mut required_moves: Vec<u128> = Vec::new();

    for starting_node in starting_nodes {
        let moves_number = find_number_of_moves(&moves, &nodes, starting_node.to_string(), true) as u128;
        required_moves.push(moves_number);
    }

    let mut p2 : u128 = 1;

    for rq in required_moves {
        p2 *= rq / gcd(p2, rq);
    }

    println!("#2 {}", p2);
}

