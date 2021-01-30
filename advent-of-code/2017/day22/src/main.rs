use std::fs;
use std::collections::HashMap;

#[derive(Debug,PartialEq,Eq,Hash,Clone,Copy)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug,Clone,Eq,PartialEq)]
enum State {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();
    let lines : Vec<&str> = lines.collect::<Vec<&str>>();

    let mut lines_n : i32 = 0 - (lines.len() / 2) as i32;
    let x_n : i32 = 0 - (lines[0].len() / 2) as i32;

    let mut points = HashMap::new();

    for line in lines {
        line
            .chars()
            .enumerate()
            .map(|(i, x)| if x == '#' { points.insert(Pos { x: x_n + i as i32, y: lines_n }, State::Infected); }
        ).count();

        lines_n += 1;
    }

    let points_orig = points.clone();


    let mut current_dir : i8 = 3;
    let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut current_pos = Pos{ x: 0, y: 0 };
    let mut infections = 0;

    for _i in 0..10000 {
        if points.contains_key(&current_pos) {
            // turns right
            current_dir -= 1;
            if current_dir < 0 {
                current_dir = 3;
            }

            points.remove(&current_pos);

        } else {
            // turns left
            current_dir += 1;
            if current_dir > 3 {
                 current_dir = 0;
            }

            points.insert(current_pos.clone(), State::Flagged);
            infections += 1;
        }

        current_pos = Pos {
            x: current_pos.x + dirs[current_dir as usize].0,
            y: current_pos.y + dirs[current_dir as usize].1,
        };
    }

    println!("Part #1: {}", infections);


    let mut current_dir : i8 = 3;
    let mut current_pos = Pos{ x: 0, y: 0 };
    let mut points = points_orig;

    let mut infections = 0;

    for _i in 0..10000000 {
        if let Some(state) = points.get(&current_pos) {
            match state {
                State::Clean => unreachable!(),
                State::Weakened => {
                    points.insert(current_pos, State::Infected);
                    infections += 1;
                },
                State::Infected => {
                    points.insert(current_pos, State::Flagged);

                    // turns right
                    current_dir -= 1;
                    if current_dir < 0 { current_dir = 3; }
                },
                State::Flagged => {
                    points.remove(&current_pos);

                    // reverse
                    for _i in 0..2 {
                        current_dir += 1;
                        if current_dir > 3 { current_dir = 0; }
                    }

                }
            };
        } else {
            // clean: turns left
            current_dir += 1;
            if current_dir > 3 { current_dir = 0; }

            points.insert(current_pos, State::Weakened);
        }

        current_pos = Pos {
            x: current_pos.x + dirs[current_dir as usize].0,
            y: current_pos.y + dirs[current_dir as usize].1,
        };

        // println!("New pos: {:?} - {:?}", current_pos, points);
    }

    // let infected = points.iter().filter(|(_, v)| **v == State::Infected).count();

    println!("Part #2: {}", infections);
}

