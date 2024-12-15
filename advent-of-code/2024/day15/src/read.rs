use std::fs;
use std::collections::HashSet;

use crate::{Box, Coord, Direction, Map};

pub fn read_input(fp: &str, duplicate: bool) -> (Map, Vec<Direction>) {
    let mut walls = HashSet::new();
    let mut boxes = HashSet::new();
    let mut robot = Coord { x: 0, y: 0 };
    let mut moves = Vec::new();
    let mut parsing_map = true;

    let contents = fs::read_to_string(fp).unwrap();
    for (y, line) in contents.lines().enumerate() {
        if line.is_empty() {
            parsing_map = false;
            continue;
        }

        if parsing_map {
            for (x, c) in line.chars().enumerate() {
                let mut coord = Coord { x: x as isize, y: y as isize };
                if duplicate {
                    coord = Coord { x: x as isize * 2, y: y as isize };
                }
                match c {
                    '#' => { 
                        walls.insert(coord);
                        if duplicate {
                            walls.insert(Coord { x: coord.x + 1, y: coord.y });
                        }
                    },
                    'O' => {
                        boxes.insert(Box{coord});
                    },
                    '@' => { robot = coord; },
                    _ => {},
                }
            }
        } else {
            let mut directions = line.chars().map(|c| match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("Invalid direction"),
            }).collect();

            moves.append(&mut directions);
        }
    }

    (Map { walls, boxes, robot, is_wide: duplicate }, moves)
}
