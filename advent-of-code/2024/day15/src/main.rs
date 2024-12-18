use std::collections::HashSet;

mod read;
use crate::read::read_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Box {
    coord: Coord, // left coordonate
}

#[derive(Debug)]
struct Map {
    walls: HashSet<Coord>,
    boxes: HashSet<Box>,
    robot: Coord,
    is_wide: bool,
}

impl Direction {
    fn shift(&self, coord: Coord) -> Coord {
        match self {
            Direction::Up => Coord { x: coord.x, y: coord.y - 1 },
            Direction::Down => Coord { x: coord.x, y: coord.y + 1 },
            Direction::Left => Coord { x: coord.x - 1, y: coord.y },
            Direction::Right => Coord { x: coord.x + 1, y: coord.y },
        }
    }

    fn mega_shift(&self , coord: Coord, n: usize) -> Coord {
        let mut new_coord = coord;
        for _ in 0..n {
            new_coord = self.shift(new_coord);
        }

        new_coord
    }
}

impl Map {
    #[allow(dead_code)]
    fn print(&self) {
        let mut max_x = 0;
        let mut max_y = 0;

        for coord in self.walls.iter().chain(self.boxes.iter().map(|b| &b.coord)).chain([self.robot].iter()) {
            if coord.x > max_x { max_x = coord.x; }
            if coord.y > max_y { max_y = coord.y; }
        }

        for y in 0..=max_y {
            for x in 0..=max_x {
                let coord = Coord { x, y };
                if self.walls.contains(&coord) {
                    print!("#");
                } else if self.has_box(coord) {
                    print!("O");
                } else if self.robot == coord {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn has_box(&self, coord: Coord) -> bool {
        if self.is_wide {
            self.boxes.contains(&Box { coord }) || self.boxes.contains(&Box { coord: Coord { x: coord.x - 1, y: coord.y } })
        } else {
            self.boxes.contains(&Box { coord })
        }
    }

    fn get_box(&self, coord: Coord) -> Option<Box> {
        if self.is_wide {
            if self.boxes.contains(&Box { coord }) {
                Some(Box { coord })
            } else if self.boxes.contains(&Box { coord: Coord { x: coord.x - 1, y: coord.y } }) {
                Some(Box { coord: Coord { x: coord.x - 1, y: coord.y } })
            } else {
                None
            }
        } else {
            Some(Box { coord })
        }
    }

    fn can_move(&self, coord: Coord, direction: Direction) -> Option<usize> {
        let mut current_coord = coord;
        let mut box_number = 0;

        if self.is_wide {
            if !self.can_move_wide(coord, direction) {
                return None;
            } else {
                return Some(0);
            }
        }

        loop {
            current_coord = direction.shift(current_coord);
            if self.walls.contains(&current_coord) {
                return None;
            }
            if self.boxes.contains(&Box { coord: current_coord }) {
                box_number += 1;
                continue;
            }

            return Some(box_number);
        }
    }

    fn can_move_wide(&self, coord: Coord, direction: Direction) -> bool {
        // let mut current_coord = coord;
        // let mut box_number = 0;

        // println!("Checking if can move wide from {:?} (direction: {:?})", coord, direction);

        if self.robot == coord {
            // println!("Robot is at {:?}", coord);
            let next_coord = direction.shift(coord);
            if self.walls.contains(&next_coord) {
                return false;
            }

            let b = self.get_box(next_coord);
            if b.is_none() {
                return true;
            }

            let b = b.unwrap();
            self.can_move_wide(b.coord, direction)
        } else {
            let b = self.get_box(coord);
            if b.is_none() {
                return true;
            }

            let b = b.unwrap();

            let next_coord1 = direction.shift(b.coord);
            let next_coord2 = direction.shift(Coord { x: b.coord.x + 1, y: b.coord.y });

            if self.walls.contains(&next_coord1) || self.walls.contains(&next_coord2) {
                return false;
            }

            match direction {
                Direction::Left => self.can_move_wide(next_coord1, direction),
                Direction::Right => self.can_move_wide(next_coord2, direction),
                _ => self.can_move_wide(next_coord1, direction) && self.can_move_wide(next_coord2, direction),
            }
        }
    }

    fn move_boxes_wide(&mut self, coord: Coord, direction: Direction) {
        let coord = direction.shift(coord);

        let b = self.get_box(coord);
        if b.is_none() {
            return;
        }

        let b = b.unwrap();

        self.boxes.remove(&b);

        self.move_boxes_wide(b.coord, direction);
        self.move_boxes_wide(Coord{x: b.coord.x + 1, y: b.coord.y}, direction);

        self.boxes.insert(Box { coord: direction.shift(b.coord) });
    }

    fn move_boxes(&mut self, coord: Coord, direction: Direction, box_number: usize) {
        if box_number == 0 {
            return;
        }

        let current_coord = direction.shift(coord);
        let next_coord = direction.mega_shift(coord, box_number + 1);

        // println!("Moving boxes from {:?} to {:?}", current_coord, next_coord);

        self.boxes.remove(&Box { coord: current_coord });
        self.boxes.insert(Box { coord: next_coord });
    }

    fn compute_score(&self) -> usize {
        self.boxes.iter().map(|b| b.coord.x as usize + 100 * b.coord.y as usize).sum()
    }
}

fn play(map: &mut Map, moves: &[Direction]) {
    for direction in moves {
        let new_robot = direction.shift(map.robot);
        let box_number = map.can_move(map.robot, *direction);

        if box_number.is_some() {
            if !map.is_wide {
                map.move_boxes(map.robot, *direction, box_number.unwrap());
            } else {
                map.move_boxes_wide(map.robot, *direction);
            }

            map.robot = new_robot;
        }
    }
}

fn main() {
    // part 1
    let (mut map, moves) = read_input("input.txt", false);
    play(&mut map, &moves);
    println!("#1: {}", map.compute_score()); // 1527563

    let (mut map, moves) = read_input("input.txt", true);
    play(&mut map, &moves);
    println!("#2: {}", map.compute_score()); // 1521635
    // 1517582 is too low


    // world.calculate()=1505648
}

#[test]
fn test_sample() {
    let (mut map, moves) = read_input("input_test.txt", false);
    play(&mut map, &moves);
    assert_eq!(2028, map.compute_score());

    let (mut map, moves) = read_input("input_test2.txt", false);
    play(&mut map, &moves);
    assert_eq!(10092, map.compute_score());

    let (mut map, moves) = read_input("input_test2.txt", true);
    play(&mut map, &moves);
    assert_eq!(9021, map.compute_score());

}