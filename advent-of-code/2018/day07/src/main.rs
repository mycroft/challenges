use std::fs;
use regex::Regex;
use std::collections::HashSet;

fn step(elements: &mut HashSet<(char, char)>) -> char {
    let mut startings = HashSet::new();
    let mut endings = HashSet::new();

    let mut candidates = vec![];
    let c : char;

    elements
        .iter()
        .map(|x| {
            if !startings.contains(&x.0) {
                startings.insert(x.0);
            }
            if !endings.contains(&x.1) {
                endings.insert(x.1);
            }
        })
        .count();

    for starting in &startings {
        if endings.contains(&starting) {
            continue;
        }

        candidates.push(starting);
    }

    candidates.sort();

    c = **candidates.get(0).unwrap();


    let to_remove : Vec<(char, char)> = elements.iter().filter(|x| x.0 == c).cloned().collect();

    for el in &to_remove {
        elements.remove(el);
    }

    c
}

fn z_step(elements: &mut HashSet<(char, char)>, done: &mut Vec<char>, wip: &mut HashSet<(char, usize)>, ts: usize) {
    // find finished tasks
    // println!("Current TS: {}", ts);
    // println!("wip: {:?} // done: {:?}", wip, done);

    let mut to_remove = vec![];

    for task in wip.iter() {
        if task.1 == ts {
            to_remove.push(task.clone());

            done.push(task.0);

            // remove all tasks where task.0 is destination.
            let to_remove_el : Vec<(char, char)> = elements.iter().filter(|x| x.1 == task.0).cloned().collect();

            for el in &to_remove_el {
                elements.remove(&el);
            }
        }
    }

    for el in &to_remove {
        wip.remove(el);
    }

    // find new tasks to do
    let mut all_possibles = HashSet::new();

    if done.len() == 0 {
        all_possibles = elements
            .iter()
            .filter(|x| {
                elements.iter().all(|y| x.0 != y.1 && !done.contains(&x.0)) && !wip.iter().any(|y| x.0 == y.0)
            })
            .map(|x| x.0)
            .collect();
    } else {
        all_possibles = elements
            .iter()
            .filter(|x| {
                let requirements : HashSet<char> = elements.iter().filter(|y| y.1 == x .1).map(|y| y.0).collect();

                requirements.iter().all(|req| done.contains(req)) && !done.contains(&x.1)
            }).map(|x| x.1)
            .collect();
    }

    let mut all_possibles : Vec<char> = all_possibles.into_iter().collect();
    all_possibles.sort();

    for possible in &all_possibles {
        if wip.len() == 5 { // 5 workers.
            continue;
        }

        if wip.iter().any(|x| x.0 == *possible) {
            continue;
        }

        let finishes_at = ts + (60 + 1 + (*possible as usize) - 'A' as usize);
        // println!("{}: Adding task: {}. Finishes at {},", ts, *possible, finishes_at);

        wip.insert((*possible, finishes_at));
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let re = Regex::new(r"^Step (?P<from>.) must be finished before step (?P<to>.) can begin.$").unwrap();

    let mut h = HashSet::new();

    for line in lines {
        let caps = re.captures(line).unwrap();

        let from = caps.name("from").unwrap().as_str().chars().nth(0).unwrap();
        let to = caps.name("to").unwrap().as_str().chars().nth(0).unwrap();

        h.insert((from, to));
    }

    let h_copy = h.clone();

    let mut res = String::from("");

    let mut last_element : char = 'a';

    while h.len() > 0 {
        for el in h.iter() {
            last_element = el.1;
        }

        let r = step(&mut h);

        res.push(r);
    }

    res.push(last_element);

    println!("Part #1: {}", res);

    let mut elements = h_copy;
    let mut done = Vec::new();
    let mut wip = HashSet::new();

    let mut ts = 0;

    loop {
        z_step(&mut elements, &mut done, &mut wip, ts);

        if done.len() == res.len() {
            break;
        }

        ts += 1;
    }    

    println!("Part #2: {}", ts);
}
