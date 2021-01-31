use std::fs;
use std::collections::{HashMap,HashSet};

fn common(a: &[u8], b: &[u8]) -> String {
    let out = a.iter().enumerate().filter(|(i, c)| **c == b[*i]).map(|(_i, c)| *c).collect::<Vec<u8>>();

    String::from_utf8(out).unwrap()
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines : Vec<String> = contents.lines().map(|x| x.to_string()).collect::<Vec<String>>();

    let mut final_result = HashMap::new();

    for line in &lines {
        let results = line
            .chars()
            .fold(HashMap::<char,usize>::new(), |mut h, c| { *h.entry(c).or_insert(0)+=1; h })
            .iter()
            .fold(HashSet::<usize>::new(), |mut h, (_c, v)| { if *v > 1 { h.insert(*v); }; h });

        for res in results {
            *final_result.entry(res).or_insert(0) += 1;
        }
    }

    let part1 = final_result.iter().fold(1, |mut res, (_k, v)| { res *= v; res});

    println!("Part #1: {}", part1);


    let lines_idx = lines.len();
    let mut max_str = String::from("");

    for id in 0..lines_idx {
        for id2 in id+1..lines_idx {
            let n = common(&lines[id].as_bytes(), &lines[id2].as_bytes());

            if n.len() > max_str.len() {
                max_str = n.clone();
            }
        }
    }

    println!("Part #2: {}", max_str);
}
