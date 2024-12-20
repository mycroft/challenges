use std::fs;
use std::collections::{HashMap,HashSet};
use pathfinding::prelude::astar;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn neighbours(&self, walls: &HashSet<Coord>, map_size: Coord, end_wall: Option<Coord>) -> Vec<Coord> {
        let mut neighbours = Vec::new();
        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let neighbour = Coord { x: self.x + dx, y: self.y + dy };
            if neighbour.x < 0 || neighbour.y < 0 || neighbour.x > map_size.x || neighbour.y > map_size.y {
                continue;
            }
            if !walls.contains(&neighbour) || end_wall == Some(neighbour) {
                neighbours.push(neighbour);
            }
        }
        neighbours
    }

    fn all_neighbour_at_distance(&self, walls: &HashSet<Coord>, distance: isize) -> Vec<Coord> {
        let mut neighbours = Vec::new();

        for x in -distance as isize..=distance as isize {
            for y in -distance as isize..=distance as isize {
                if self.manhattan_distance(&Coord { x: self.x + x, y: self.y + y }) > distance as usize {
                    continue;
                }

                let neighbour = Coord { x: self.x + x, y: self.y + y };
                if !walls.contains(&neighbour) {
                    neighbours.push(neighbour);
                }
            }
        }
        neighbours
    }

    fn manhattan_distance(&self, other: &Coord) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }
}

#[derive(Debug)]
struct Race {
    walls: HashSet<Coord>,
    start: Coord,
    end: Coord,
    size: Coord,
}

fn read_input(fp: &str) -> Race {
    let mut walls = HashSet::new();
    let mut start = Coord { x: 0, y: 0 };
    let mut end = Coord { x: 0, y: 0 };

    let contents = fs::read_to_string(fp).expect("Could not read file");
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let coord = Coord { x: x as isize, y: y as isize };
            match c {
                '#' => { walls.insert(coord); },
                'S' => { start = coord; },
                'E' => { end = coord; },
                _ => {}
            }
        }
    }
    
    let map_size = walls.iter().fold(Coord { x: 0, y: 0 }, |acc, c| {
        Coord { x: acc.x.max(c.x), y: acc.y.max(c.y) }
    });

    Race { walls, size: map_size, start, end }
}

fn solve(race: &Race, start: Coord, end: Coord) -> (Vec<Coord>, usize) {
    let path = astar(
        &start,
        |p| p.neighbours(&race.walls, race.size, None).into_iter().map(|p| (p, 1)),
        |p| p.manhattan_distance(&race.end),
        |p| *p == end,
    );

    let coords = path.clone().unwrap().0.iter().cloned().collect();

    (coords, path.unwrap().1)
}

fn solve0(race: &Race, distance: usize, threshold: usize) -> usize {
    let mut total = 0;
    let mut legit_steps = HashMap::new();
    let (path, legit_time) = solve(&race, race.start, race.end);

    for (idx, step) in path.iter().enumerate() {
        legit_steps.insert(step, legit_time - idx);
    }

    for (current_time, step) in path.iter().enumerate() {
        let all_neighbours = step.all_neighbour_at_distance(&race.walls, distance as isize);

        for neighbour in all_neighbours {
            if neighbour == *step {
                continue;
            }

            if legit_steps.contains_key(&neighbour) {
                let cheating_time = *legit_steps.get(&neighbour).unwrap();
                let after_time = current_time + cheating_time + step.manhattan_distance(&neighbour) as usize;

                if after_time < legit_time {
                    let gain = legit_time - after_time;

                    if gain >= threshold {
                        // println!("Cheating: {:?}->{:?} current:{} cheating_time:{} before:{} after:{} Gain:{}",
                        //    *step, neighbour, current_time, cheating_time, legit_time, after_time, gain);

                        total += 1;
                    }
                }   
            }

        }

    }

    total  
}

fn main() {
    let race = read_input("input.txt");
    let step1 = solve0(&race, 2, 100);
    let step2 = solve0(&race, 20, 100);

    println!("#1: {}", step1);
    println!("#2: {}", step2);

    // 1018373 is too high
}

#[test]
fn sample() {
    let race = read_input("input_test.txt");
    let result = solve0(&race, 2, 0);
    assert_eq!(44, result);

    let result = solve0(&race, 20, 50);
    assert_eq!(285, result);

}