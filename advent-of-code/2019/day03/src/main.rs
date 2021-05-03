/*
 * AOC 2019-03
 */
use std::fs;

use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut points : Vec<HashMap<(i32, i32), usize>> = Vec::new();
    
    for (_lineidx, line) in lines.iter().enumerate() {
        let moves = line.split(",").collect::<Vec<&str>>();

        let mut points_row : HashMap<(i32, i32), usize> = HashMap::new();

        let mut x : i32 = 0;
        let mut y : i32 = 0;

        let mut idx = 0;

        for (_movidx, mov) in moves.iter().enumerate() {
            let dir = mov.chars().nth(0).unwrap();
            let steps = &mov[1..mov.len()];

            let dir = match dir {
                'R' => (1, 0),
                'L' => (-1, 0),
                'U' => (0, -1),
                'D' => (0, 1),
                _ => unimplemented!()
            };

            for _n in 0..steps.parse::<i32>().unwrap() {
                x += dir.0;
                y += dir.1;

                idx += 1;

                points_row.insert((x, y), idx);
            }
        }        

        points.push(points_row);
    }

    let p0 = points.get(0).unwrap();
    let p1 = points.get(1).unwrap();

    let mut min_distance = 0;
    let mut min_distance2 = 0;

    for p in p0 {
        let i = p1.get(p.0);

        if i == None {
            continue;
        }

        let manhattan = p.0.0.abs() + p.0.1.abs();

        if min_distance == 0 || min_distance > manhattan {
            min_distance = manhattan;
        }

        if min_distance2 == 0 || min_distance2 > p.1 + i.unwrap() {
            min_distance2 = p.1 + i.unwrap();
        }
    }

    println!("Part #1: {}", min_distance);
    println!("Part #2: {}", min_distance2);
}
