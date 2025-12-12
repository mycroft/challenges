use std::hash::Hash;
use std::{collections::HashSet, fs::read_to_string};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point{
    x: isize,
    y: isize,
    z: isize,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

fn parse(fp: &str) -> HashSet<Point> {
    let contents = read_to_string(fp).expect("failed to open and read file");
    let mut points = HashSet::new();

    for line in contents.lines() {
        let mut coords = line.split(',');
        points.insert(Point {
            x: coords.next().unwrap().parse::<isize>().unwrap(),
            y: coords.next().unwrap().parse::<isize>().unwrap(),
            z: coords.next().unwrap().parse::<isize>().unwrap(),
        });
    }

    points
}

fn get_all_distances(points: &HashSet<Point>) -> HashMap<(Point, Point), f64> {
    let mut distances = HashMap::new();

    for p0 in points {
        for p1 in points {
            if p0 == p1 {
                continue;
            }
            if distances.contains_key(&(*p1, *p0)) {
                continue;
            }
            distances.insert((*p0, *p1), p0.distance(p1));
        }
    }

    distances
}

fn merge(circuits: &Vec<HashSet<Point>>, p0: Point, p1: Point) -> Vec<HashSet<Point>> {
    // check all circuits where p0 or p1 are in
    let mut known = Vec::new();
    let mut final_circuits = Vec::new();

    for circuit in circuits {
        if circuit.contains(&p0) || circuit.contains(&p1) {
            known.push(circuit);
        } else {
            final_circuits.push(circuit.clone());
        }
    }

    let mut new_circuit: HashSet<Point> = HashSet::new();
    if known.is_empty() {
        new_circuit.insert(p0);
        new_circuit.insert(p1);
    } else {
        for circuit in known {
            for point in circuit {
                if !new_circuit.contains(point) {
                    new_circuit.insert(*point);
                }
            }
        }
    }

    if !new_circuit.contains(&p0) {
        new_circuit.insert(p0);
    }
    if !new_circuit.contains(&p1) {
        new_circuit.insert(p1);
    }

    final_circuits.push(new_circuit);

    final_circuits
}

fn main() {
    let points = parse("input.txt");

    let distances = get_all_distances(&points);

    let mut sorted_distances: Vec<_> = distances.iter().collect();
    sorted_distances.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

    let mut circuits = Vec::new();

    let mut count_it = 0;

    for (couple, _) in sorted_distances {
        circuits = merge(&circuits, couple.0, couple.1);

        count_it += 1;
        if count_it == 1000 {
            let mut sizes: Vec<usize> = circuits.iter().map(|c| c.len()).collect();
            sizes.sort();
            let top3 = sizes.iter().rev().take(3).collect::<Vec<_>>();
            let s = top3.iter().fold(1, |acc, &x| acc * x);
            println!("#1: {}", s);
        }

        if circuits.len() == 1 && circuits.get(0).unwrap().len() == points.len() {
            println!("#2: {}", couple.0.x * couple.1.x);
            break;
        }
    }
}
