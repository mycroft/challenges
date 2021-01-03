use std::fs;
use std::collections::HashSet;
use regex::Regex;
use std::cmp::{min,max};

fn main() {
    let mut current_direction = 0;
    let mut i : i16 = 0;
    let mut j : i16 = 0;
    let contents = fs::read_to_string("input.txt").unwrap();
    let _directions : Vec<&str> = contents.trim().split(", ").collect();

    let mut locations : HashSet<(i16, i16)> = HashSet::new();
    let mut already_visited = None;

    let re = Regex::new(r"(.)(\d+)").unwrap();

    for direction in _directions {
        let cap = re.captures(direction).unwrap();

        match cap.get(1).unwrap().as_str() {
            "R" => {
                current_direction += 90;
            },
            "L" => {
                current_direction -= 90;
            },
            _ => {
                unimplemented!();
            }
        }

        if current_direction < 0 {
            current_direction += 360;
        } else if current_direction >= 360 {
            current_direction -= 360;
        }

        let val = cap.get(2).unwrap().as_str().parse::<i16>().unwrap();

        let mut visited : Vec<(i16, i16)> = vec![];

        let before = (i, j);

        match current_direction {
            0 => {
                j -= val;
            },
            90 => {
                i += val;
            },
            180 => {
                j += val;
            },
            270 => {
                i -= val;
            }
            _ => {
                unimplemented!();
            }
        }

        let after = (i, j);

        for z1 in min(before.0, after.0)..=max(before.0, after.0) {
            for z2 in min(before.1, after.1)..=max(before.1, after.1) {
                if (z1, z2) == before {
                    continue;
                }
                visited.push((z1, z2));
            }
        }

        for visit in &visited {
            if locations.contains(&visit) {
                match already_visited {
                    None => {
                        already_visited = Some(visit.clone());
                    }
                _ => {}
                }
            } else {
                locations.insert(*visit);
            } 
        }
    }

    println!("Part #1: {:?} {:?} total: {}", i, j, (i + j).abs());

    let visited_location = already_visited.unwrap();
    println!("Part #2: {:?} {:?} total: {}",
        visited_location.0,
        visited_location.1,
        (visited_location.0 + visited_location.1).abs()
    );
}
