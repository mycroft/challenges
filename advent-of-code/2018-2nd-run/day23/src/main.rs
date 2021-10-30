/* AOC 2018 - 23 */
use std::{collections::BTreeMap, fs};
use std::cmp::max;
use regex::Regex;

#[derive(Clone, Copy, Debug)]
struct Pos(i64, i64, i64);

#[derive(Clone, Copy, Debug)]
struct Drone {
    pos: Pos,
    r: i64,
}

fn manhattan_distance(p0: Pos, p1: Pos) -> i64 {
    (p0.0 - p1.0).abs() + (p0.1 - p1.1).abs() + (p0.2 - p1.2).abs()
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let re = Regex::new(r"^pos=<(.*),(.*),(.*)>, r=(.*)$").unwrap();

    let mut drones : Vec<Drone> = vec![];
    let mut max_range = 0;
    let mut max_range_idx = 0;

    for (idx, line) in lines.iter().enumerate() {
        let caps = re.captures(line).unwrap();
        let drone = Drone {
            pos: Pos(
                caps.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                caps.get(2).unwrap().as_str().parse::<i64>().unwrap(),
                caps.get(3).unwrap().as_str().parse::<i64>().unwrap(),
            ),
            r: caps.get(4).unwrap().as_str().parse::<i64>().unwrap(),
        };

        if drone.r > max_range {
            max_range = drone.r;
            max_range_idx = idx;
        }

        drones.push(drone);
    }

    let mut detected_drones = 0;

    for drone in &drones {
        if manhattan_distance(drone.pos, drones[max_range_idx].pos) <= max_range {
            detected_drones += 1;
        }
    }

    println!("#1: {}", detected_drones);

    let mut hm: BTreeMap<i64, i64> = BTreeMap::new();

    for drone in &drones {
        let d = drone.pos.0.abs() + drone.pos.1.abs() + drone.pos.2.abs();

        hm.insert(max(d - drone.r, 0), 1);
        hm.insert(d + drone.r + 1, -1);
    }

    let mut num = 0;
    let mut max_count = 0;
    let mut distance = 0;

    for (k, v) in hm {
        num += v;

        if num > max_count {
            max_count = num;
            distance = k;
        }
    }

    println!("#2: {}", distance);
}

