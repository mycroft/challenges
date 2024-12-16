use std::fs;
use std::collections::HashSet;

use pathfinding::prelude::astar;
use pathfinding::directed::astar::astar_bag;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    coord: Coord,
    direction: Direction,
}


impl State {
    fn successors(&self, maze: &HashSet<Coord>) -> Vec<(State, usize)> {
        let mut succs = Vec::new();

        let mut new_state = *self;
        new_state.coord.x += 1;
        new_state.direction = Direction::East;
        if maze.contains(&new_state.coord) && self.direction != Direction::West {
            if self.direction != Direction::East {
                succs.push((new_state, 1001));
            } else {
                succs.push((new_state, 1));
            }
        }

        let mut new_state = *self;
        new_state.coord.x -= 1;
        new_state.direction = Direction::West;
        if maze.contains(&new_state.coord) && self.direction != Direction::East {
            if self.direction != Direction::West {
                succs.push((new_state, 1001));
            } else {
                succs.push((new_state, 1));
            }
        }

        let mut new_state = *self;
        new_state.coord.y += 1;
        new_state.direction = Direction::South;
        if maze.contains(&new_state.coord) && self.direction != Direction::North {
            if self.direction != Direction::South {
                succs.push((new_state, 1001));
            } else {
                succs.push((new_state, 1));
            }
        }

        let mut new_state = *self;
        new_state.coord.y -= 1;
        new_state.direction = Direction::North;
        if maze.contains(&new_state.coord) && self.direction != Direction::South {
            if self.direction != Direction::North {
                succs.push((new_state, 1001));
            } else {
                succs.push((new_state, 1));
            }
        }

        succs
    }
}

fn read_input(fp: &str) -> (HashSet<Coord>, State, Coord) {
    let contents = fs::read_to_string(fp)
        .expect("Something went wrong reading the file");

    let lines = contents.lines();

    let mut starting_coord : Coord = Coord { x: 0, y: 0 };
    let mut ending_coord : Coord = Coord { x: 0, y: 0 };
    let mut maze = HashSet::new();

    for (id_y, line) in lines.enumerate() {
        for (id_x, c) in line.chars().enumerate() {
            if c == '#' {
                continue;
            }

            match c {
                'S' => {
                    starting_coord = Coord { x: id_x as isize, y: id_y as isize };
                    maze.insert(starting_coord);
                },
                'E' => {
                    ending_coord = Coord { x: id_x as isize, y: id_y as isize };
                    maze.insert(ending_coord);
                },
                _ => {
                    maze.insert(Coord { x: id_x as isize, y: id_y as isize });
                },
            }
        }
    }

    (maze, State { coord: starting_coord, direction: Direction::East }, ending_coord)
}

fn solve_step1(maze: &HashSet<Coord>, initial: &State, end: &Coord) -> usize {
    let path = astar(
        initial,
        |s| s.successors(maze),
        |_s| 1,
        |s| s.coord == *end
    );

    path.unwrap().1 as usize
}

fn solve_step2(maze: &HashSet<Coord>, initial: &State, end: &Coord) -> usize {
    let mut seats: HashSet<Coord> = HashSet::new();

    let pathes = astar_bag(
        initial,
        |s| s.successors(maze),
        |_s| 1,
        |s| s.coord == *end
    );

    if pathes.is_none() {
        return 0;
    }

    let pathes = pathes.unwrap();

    for path in pathes.0 {
        for seat in path {
            seats.insert(seat.coord);
        }
    }

    seats.len()
}

fn main() {
    let (maze, s, e) = read_input("input.txt");

    println!("#1 {:?}", solve_step1(&maze, &s, &e));
    println!("#2 {:?}", solve_step2(&maze, &s, &e));
}

#[test]
fn test_sample() {
    let (maze, s, e) = read_input("input_test.txt");

    assert_eq!(7036, solve_step1(&maze, &s, &e));
    assert_eq!(45, solve_step2(&maze, &s, &e));

    let (maze, s, e) = read_input("input_test2.txt");

    assert_eq!(11048, solve_step1(&maze, &s, &e));
    assert_eq!(64, solve_step2(&maze, &s, &e));
}