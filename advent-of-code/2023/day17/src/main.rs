use std::{fs, collections::HashMap};
use pathfinding::prelude::dijkstra;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State{
    location: Location,
    previous_location: Option<Location>,
}

impl State {
    fn successors(&self, hm: &HashMap<Location, isize>, min_size: isize, max_size: isize) -> Vec<(State, isize)> {
        let mut results = Vec::new();

        let mut dirs = [(0isize, 1isize), (1, 0)];

        if self.previous_location.is_some() {
            if self.previous_location.unwrap().x != self.location.x {
                dirs = [(0, -1), (0, 1)];
            } else {
                dirs = [(-1, 0), (1, 0)];
            }
        }

        for dir in dirs {
            let mut score = 0;
            for n in 1..=max_size {
                let mut new_location = self.location;
                new_location.x += dir.0 * n;
                new_location.y += dir.1 * n;
            
                if !hm.contains_key(&new_location) {
                    continue;
                }

                score += hm.get(&new_location).unwrap();

                if n < min_size {
                    continue;
                }
            

                results.push(
                    (State{location: new_location, previous_location: Some(self.location)}, score),
                )
            }
        }

        results
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("a file to open");
    let lines = contents.lines();

    let mut hs = HashMap::new();

    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            hs.insert(Location{x: x as isize, y: y as isize}, c.to_digit(10).unwrap() as isize);

            max_x = x;
        }
        max_y = y;
    }

    let initial_state = State{
        location: Location{
            x: 0,
            y: 0,
        },
        previous_location: None,
    };

    let result = dijkstra(
        &initial_state,
        |s| s.successors(&hs, 1, 3),
        |s| s.location == Location{x: max_x as isize, y: max_y as isize},
    );

    println!("#1 {:?}", result.unwrap().1);

    let result = dijkstra(
        &initial_state,
        |s| s.successors(&hs, 4, 10),
        |s| s.location == Location{x: max_x as isize, y: max_y as isize},
    );

    let result = result.unwrap();

    println!("#2 {:?}", result.1);
}
