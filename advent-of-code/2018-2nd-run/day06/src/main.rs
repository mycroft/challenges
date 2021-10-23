use std::fs;
use std::collections::HashMap;

#[derive(Copy,Clone,Debug)]
struct Coord {
    idx: usize,
    x: usize,
    y: usize,
}

fn create_matrix(coords: &Vec<Coord>) -> (Coord, Vec<Vec<usize>>) {
    let mut min_x = None;
    let mut min_y = None;
    let mut max_x = None;
    let mut max_y = None;

    for coord in coords {
        if min_x == None {
            min_x = Some(coord.x);
            max_x = Some(coord.x);
            min_y = Some(coord.y);
            max_y = Some(coord.y);

            continue;
        }

        if min_x.unwrap() > coord.x {
            min_x = Some(coord.x);
        }

        if max_x.unwrap() < coord.x {
            max_x = Some(coord.x);
        }

        if min_y.unwrap() > coord.y {
            min_y = Some(coord.y);
        }

        if max_y.unwrap() < coord.y {
            max_y = Some(coord.y);
        }
    }

    // println!("{}, {} = {}, {}", min_x.unwrap(), max_x.unwrap(), min_y.unwrap(), max_y.unwrap());

    (
        Coord{
            idx: 0,
            x: min_x.unwrap(),
            y: min_y.unwrap()
        },
        vec![vec![0; max_x.unwrap() - min_x.unwrap() + 2]; max_y.unwrap() - min_y.unwrap() + 2]
    )
}

fn manhattan(a: Coord, b: Coord) -> usize {
    ((a.x as i32- b.x as i32).abs() + (a.y as i32 - b.y as i32).abs()) as usize
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut coords: Vec<Coord> = vec![];

    for (idx, line) in lines.iter().enumerate() {
        let parts : Vec<&str> = line.split(", ").collect();

        let coord = Coord {
            idx: idx + 1,
            x: parts[0].parse::<usize>().unwrap(),
            y: parts[1].parse::<usize>().unwrap(),
        };

        coords.push(coord);
    }

    let (root_coord, mut mx) = create_matrix(&coords);

    for x in 0..mx.len() {
        for y in 0..mx[0].len() {
            let real_x = root_coord.x + x;
            let real_y = root_coord.y + y;

            let mut match_idx = None;
            let mut min_distance = None;

            for coord in &coords {
                let current_distance = manhattan(*coord, Coord{idx:0, x:real_x, y:real_y});
                //if current_distance != 0 { continue; }
                if min_distance == None || min_distance.unwrap() >= current_distance {
                    if min_distance == None {
                        min_distance = Some(current_distance);
                        match_idx = Some(coord.idx);
                        continue;
                    }
                    if min_distance.unwrap() == current_distance {
                        match_idx = Some(0);
                    } else {
                        min_distance = Some(current_distance);
                        match_idx = Some(coord.idx);
                    }
                }
            }

            if match_idx != None {
                mx[x][y] = match_idx.unwrap();
            }
        }
    }

    let mut excluded : Vec<usize> = vec![];

    for x in 0..mx.len() {
        for y in 0..mx[0].len() {
            if x != 0 && x != mx.len() - 1 && y != 0 && y != mx[0].len() - 1 { continue };
            if !excluded.contains(&mx[x][y]) {
                excluded.push(mx[x][y]);
            }
        }
    }

    let mut counts : HashMap<usize, usize> = HashMap::new();

    for x in 0..mx.len() {
        for y in 0..mx[0].len() {
            if excluded.contains(&mx[x][y]) {
                continue;
            }
            *counts.entry(mx[x][y]).or_insert(0) += 1;
        }
    }

    let prefix_size = 200;
    let mx2 = vec![vec![0; prefix_size * 2]; prefix_size * 2];
    let mut count = 0;

    for x in 0..mx2.len() {
        for y in 0..mx2[x].len() {
            let coord = Coord{idx: 0, x: x + prefix_size, y: y + prefix_size};
            let sum_distance : usize = coords
                .iter()
                .map(|c| manhattan(
                    coord,
                    Coord{idx: 0, x: c.x + prefix_size, y:c.y + prefix_size}
                ))
                .sum();

            if sum_distance >= 10000 {
                continue;
            }

            count += 1;
        }
    }

    println!("#1: {}", counts.iter().map(|(_k, v)| v).max().unwrap());
    println!("#2: {}", count);
}
