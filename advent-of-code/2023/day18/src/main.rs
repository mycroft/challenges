use std::fs;
use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

fn get_square_size(hs: &HashSet<Position>) -> (isize, isize, isize, isize) {
    let mut min_x = None;
    let mut min_y = None;
    let mut max_x = None;
    let mut max_y = None;

    for item in hs {
        if min_x.is_none() {
            min_x = Some(item.x);
            min_y = Some(item.y);
            max_x = Some(item.x);
            max_y = Some(item.y);
            continue;
        }

        if item.x < min_x.unwrap() {
            min_x = Some(item.x);
        }
        if item.x > max_x.unwrap() {
            max_x = Some(item.x);
        }
        if item.y < min_y.unwrap() {
            min_y = Some(item.y);
        }
        if item.y > max_y.unwrap() {
            max_y = Some(item.y);
        }
    }

    (min_x.unwrap(), max_x.unwrap(), min_y.unwrap(), max_y.unwrap())
}

fn fill(hs: &HashSet<Position>) -> HashSet<Position> {
    let mut filled: HashSet<Position> = HashSet::new();

    let (min_x, max_x, min_y, _) = get_square_size(hs);
    let mut initial_position = Position{x: 0, y: min_y + 1};
    let mut to_visit = VecDeque::new();

    for x in min_x..=max_x {
        initial_position.x = x;

        if hs.contains(&initial_position) {
            initial_position.x += 1;
            break;
        }
    }

    to_visit.push_back(initial_position);

    let dirs = [(1isize, 0isize), (-1, 0), (0, -1), (0, 1)];

    loop {
        if to_visit.is_empty() {
            break;
        }

        let current_position = to_visit.pop_front().unwrap();
        filled.insert(current_position);

        for dir in dirs {
            let mut new_position = current_position;
            new_position.x += dir.0;
            new_position.y += dir.1;

            if hs.contains(&new_position) || to_visit.contains(&new_position) || filled.contains(&new_position) {
                continue;
            }

            to_visit.push_back(new_position);

        }
    }

    filled
}

fn draw(hs: &HashSet<Position>, visited: &HashSet<Position>) {
    let (min_x, max_x, min_y, max_y) = get_square_size(hs);

    println!("min_x:{} max_x:{} min_y:{} max_y:{}", min_x, max_x, min_y, max_y);
    println!("{:?}", hs);

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if hs.contains(&Position{x, y}) {
                print!("#");
            } else if visited.contains(&Position{x, y}) {
                print!("o");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_values_by_dir(c: &str) -> (isize, isize) {
    match c {
        "1" | "D" => (0, 1),
        "0" | "R" => (1, 0),
        "2" | "L" => (-1, 0),
        "3" | "U" => (0, -1),
        _ => unreachable!(),
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("a file to open");
    let lines = contents.lines();
    let mut hs = HashSet::new();
    let mut hs2_polygons = VecDeque::new();

    let mut current_position = Position{
        x: 0,
        y: 0,
    };

    let mut p2_current_position = Position{
        x: 0,
        y: 0,
    };

    for line in lines {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let step = parts[1].parse::<isize>().unwrap();
        let dir = get_values_by_dir(parts[0]);

        for _ in 0..step {
            current_position.x += dir.0;
            current_position.y += dir.1;

            hs.insert(current_position);
        }

        let hex = parts[2];
        let hex_steps = &hex[2..7];
        let p2_steps = isize::from_str_radix(hex_steps, 16).unwrap();
        let hex_dirs = &hex[7..8];
        let p2_dir = get_values_by_dir(hex_dirs);

        p2_current_position.x += p2_dir.0 * p2_steps;
        p2_current_position.y += p2_dir.1 * p2_steps;

        hs2_polygons.push_back(p2_current_position);
    }

    let filled = fill(&hs);

    println!("#1 {}", filled.len() + hs.len());

    println!("#2 {}", calculate_area(&hs2_polygons));

    // draw(&hs, &filled);

}

fn calculate_area(polygon: &VecDeque<Position>) -> usize {
    let mut area: i128 = 0;

    for n in 0..polygon.len() {
        let pos_from = polygon[n];
        let pos_to = polygon[(n+1)%polygon.len()];

        let r = (pos_from.x - pos_to.x) + (pos_from.y - pos_to.y);

        area += (pos_from.x * pos_to.y - pos_from.y * pos_to.x + r.abs()) as i128;
    }

    (area.unsigned_abs() / 2 + 1) as usize
}
