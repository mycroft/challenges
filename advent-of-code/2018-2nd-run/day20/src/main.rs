/*
 * 2018 - day 20
 */
use std::fs;
use std::cmp::min;
use std::collections::{HashMap,HashSet};

fn get_max_distance(regex: String) -> (i32, usize) {
    let mut directions : HashMap<char, (i32, i32)> = HashMap::new();
    directions.insert('N', (0, -1));
    directions.insert('S', (0, 1));
    directions.insert('W', (-1, 0));
    directions.insert('E', (1, 0));

    let mut positions : Vec<(i32, i32)> = vec![];
    let mut m : HashMap<(i32, i32), HashSet<(i32, i32)>> = HashMap::new();
    let mut distances : HashMap<(i32, i32), i32> = HashMap::new();

    let mut x = 10000;
    let mut y = 10000;

    let mut prev_x = x;
    let mut prev_y = y;

    for c in regex.chars() {
        if c == '^' || c == '$' {
            continue;
        }

        if c == '(' {
            positions.push((x, y));
        } else if c == ')' {
            let res = positions.pop().unwrap();
            x = res.0;
            y = res.1;
        } else if c == '|' {
            let res = positions.last().unwrap();
            x = res.0;
            y = res.1;
        } else {
            let (dx, dy) = directions.get(&c).unwrap();
            x += dx;
            y += dy;

            m.entry((x, y)).or_insert(HashSet::new()).insert((prev_x, prev_y));

            let current_distance = match distances.get(&(x, y)) {
                Some(x) => *x,
                None => 0
            };

            if current_distance == 0 {
                let old_distance = match distances.get(&(prev_x, prev_y)) {
                    Some(x) => *x,
                    None => 0,
                };
                distances.insert((x, y), old_distance + 1);
            } else {
                distances.insert((x, y), min(distances[&(prev_x, prev_y)] + 1, distances[&(x, y)]));
            }
        }

        prev_x = x;
        prev_y = y;
    }
    
    (
        *distances.iter().map(|(_, k)| k).max().unwrap(),
        distances.iter().map(|(_, k)| k).filter(|&v| *v >= 1000).count()
    )
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let contents = contents.trim_end().to_string();

    let res = get_max_distance(contents);

    println!("#1: {}", res.0);
    println!("#2: {}", res.1);
}

#[test]
fn basic_test() {
    assert_eq!((3, 0), get_max_distance("^WNE$".to_string()));
    assert_eq!((10, 0), get_max_distance("^ENWWW(NEEE|SSE(EE|N))$".to_string()));
    assert_eq!((18, 0), get_max_distance("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$".to_string()));
}