use std::collections::{HashMap,HashSet};

#[macro_use] extern crate scan_fmt;

mod parse;
use parse::{parse,Valve};

fn part1_search(valves: &HashMap<String, Valve>, opened: &HashSet<String>, flowed: isize, current_room: &String, depth_to_go: isize, best: &mut isize) {
    // println!("opened{opened:?} {flowed} {current_room} depth:{depth_to_go}");
    if flowed > *best {
        *best = flowed
    }

    if depth_to_go <= 0 {
        return;
    }

    if !opened.contains(current_room) {
        let mut opened = opened.clone();
        opened.insert(current_room.clone());

        part1_search(
            valves, 
            &opened,
            flowed + valves.get(current_room).unwrap().rate * depth_to_go,
            current_room,
            depth_to_go - 1,
            best
        );
    } else {
        //let destinations = valves.get(current_room).unwrap().destinations.clone();
        for v in &valves.get(current_room).unwrap().destinations {
            if opened.contains(v.0) {
                continue;
            }

            // if depth_to_go - v.1 < 0 {
            //    continue;
            //}

            part1_search(
                valves, 
                &opened,
                flowed,
                &v.0,
                depth_to_go - v.1,
                best
            );    
        }
    }
}

fn part1(valves: &HashMap<String, Valve>) -> isize {
    let mut best = 0;

    let mut hs = HashSet::new();
    hs.insert("AA".to_string());

    part1_search(&valves, &hs, 0, &"AA".to_string(), 29, &mut best);

    best
}

fn part2_search(
    valves: &HashMap<String, Valve>,
    opened: &HashSet<String>,
    flowed: isize,
    current_room: &String,
    depth_to_go: isize,
    elephants_turn: bool,
    best: &mut isize
) {
    if flowed > *best {
        *best = flowed
    }

    if depth_to_go <= 0 {
        return;
    }

    if !opened.contains(current_room) {
        let mut opened = opened.clone();
        opened.insert(current_room.clone());

        part2_search(
            valves, 
            &opened,
            flowed + valves.get(current_room).unwrap().rate * depth_to_go,
            current_room,
            depth_to_go - 1,
            elephants_turn,
            best
        );
        if !elephants_turn {
            let mut opened = opened.clone();
            opened.insert(current_room.clone());

            part2_search(
                valves,
                &opened,
                flowed + valves.get(current_room).unwrap().rate * depth_to_go,
                &"AA".to_string(),
                25,
                true,
                best,
            );
        }
    } else {
        let destinations = valves.get(current_room).unwrap().destinations.clone();
        for v in destinations {
            if opened.contains(&v.0) {
                continue;
            }

            part2_search(
                valves, 
                &opened,
                flowed,
                &v.0,
                depth_to_go - v.1,
                elephants_turn,
                best
            );    
        }
    }
}

fn part2(valves: &HashMap<String, Valve>) -> isize {
    let mut best = 0;

    let mut hs = HashSet::new();
    hs.insert("AA".to_string());

    part2_search(&valves, &hs, 0, &"AA".to_string(), 25, false, &mut best);

    best
}

fn main() {
    let valves = parse("input.txt");
    println!("#1 {}", part1(&valves)); // 1741
    println!("#2 {}", part2(&valves)); // 2316
}

#[test]
fn test_sample_part1() {
    let valves = parse("input.txt_test");
    assert_eq!(
        1651,
        part1(&valves)
    );
}

#[test]
fn test_sample_part2() {
    let valves = parse("input.txt_test");
    assert_eq!(
        1707,
        part2(&valves)
    );
}
