use std::fs;
use std::collections::{HashMap, HashSet};

fn read_input(fp: &str) -> HashSet<(String, String)> {
    let mut set = HashSet::new();
    let contents = fs::read_to_string(fp).expect("Error reading the file");
    for line in contents.lines() {
        let mut iter = line.split("-");
        let a = iter.next().unwrap().to_string();
        let b = iter.next().unwrap().to_string();
        set.insert((a, b));
    }
    set
}

fn find_connected(links: &HashSet<(String, String)>, a: &str) -> HashSet<String> {
    let mut set = HashSet::new();
    for link in links {
        if link.0 == a {
            set.insert(link.1.clone());
        }
        if link.1 == a {
            set.insert(link.0.clone());
        }
    }
    set
}

fn identity_3sets(links: &HashSet<(String, String)>) -> HashSet<Vec<String>> {
    let mut sets = HashSet::new();
    for link in links {
        let connected_left = find_connected(links, &link.0);
        let connected_right = find_connected(links, &link.1);

        let intersection = connected_left.intersection(&connected_right);
        let intersection: HashSet<String> = intersection.cloned().collect();
        if intersection.is_empty() {
            continue;
        }
        
        for i in intersection {
            let mut vec = vec![
                i.clone(),
                link.0.clone(),
                link.1.clone(),
            ];

            vec.sort();

            if sets.contains(&vec) {
                continue;
            }
            sets.insert(vec);
        }
    }

    sets
}

fn solve_step1(links: &HashSet<(String, String)>) -> (usize, HashSet<Vec<String>>) {
    let mut result = 0;
    let sets = identity_3sets(links);
    for set in &sets {
        if set.iter().filter(|&x| x.starts_with("t")).count() > 0 {
            result += 1;
        };
    }
    (result, sets)
}

fn solve_step2(links: &HashSet<(String, String)>, sets: &HashSet<Vec<String>>) -> String {
    let mut hash: HashMap<String, HashSet<String>> = HashMap::new();
    let mut max_size = 0;
    let mut largest_set: HashSet<String> = HashSet::new();

    for link in links {
        hash.entry(link.0.clone()).or_default().insert(link.0.clone());
        hash.entry(link.0.clone()).or_default().insert(link.1.clone());
        hash.entry(link.1.clone()).or_default().insert(link.0.clone());
        hash.entry(link.1.clone()).or_default().insert(link.1.clone());
    }
    
    for set in sets {
        let mut current_set: HashSet<String> = set.clone().into_iter().collect();
        loop {
            let mut sibblings : Option<HashSet<String>> = None;

            let mut added = false;

            for k in set.iter() {
                if sibblings.is_none() {
                    sibblings = Some(hash.get(k).unwrap().clone());
                } else {
                    sibblings = Some(sibblings.unwrap().intersection(hash.get(k).unwrap()).cloned().collect());
                }
            }

            for s in sibblings.clone().unwrap() {
                if current_set.contains(&s) {
                    continue;
                }
                let intersection: HashSet<String> = hash.get(&s).unwrap().intersection(&current_set).cloned().collect();
                if intersection.len() == current_set.len() {
                    current_set.insert(s.clone());
                    added = true;
                }
            }

            if !added {
                break;
            }
        }

        if current_set.len() > max_size {
            max_size = current_set.len();

            largest_set = current_set.clone();
        }
    }

    let mut v = largest_set.iter().collect::<Vec<&String>>();
    v.sort();

    v.into_iter().cloned().collect::<Vec<String>>().join(",")
}

fn main() {
    let links = read_input("input.txt");

    // get current time
    let start_at = std::time::Instant::now();

    let (result_step1, sets) = solve_step1(&links);
    println!("#1: {}", result_step1);
    println!("Elapsed: {:?}", start_at.elapsed());
    let result_step2 = solve_step2(&links, &sets);
    println!("#2: {}", result_step2);
    println!("Elapsed: {:?}", start_at.elapsed());

}

#[test]
fn sample1() {
    let links = read_input("input_test.txt");
    let result = solve_step1(&links);
    assert_eq!(7, result);
}
