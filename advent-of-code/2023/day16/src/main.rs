use std::fs;
use std::collections::{HashMap, VecDeque, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cursor {
    location: Location,
    direction: Direction,
}

fn readmap(filepath: &str) -> HashMap<Location, char> {
    let mut result = HashMap::new();

    let contents = fs::read_to_string(filepath).expect("a file to open");
    let lines = contents.lines().collect::<Vec<&str>>();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            result.insert(Location{x: x as isize, y: y as isize}, c);
        }
    }

    result
}

fn draw(hm: &HashMap<Location, char>, visited: &HashSet<Location>) {
    let mut max_x = 0;
    let mut max_y = 0;

    for loc in hm.keys() {
        if loc.x > max_x {
            max_x = loc.x;
        }
        if loc.y > max_y {
            max_y = loc.y;
        }
    }

    for y in 0..=max_y {
        for x in 0..=max_x {
            if visited.contains(&Location{x, y}) {
                print!("#");
            } else {
                let c = hm.get(&Location { x, y }).unwrap();
                print!("{}", c);
            }
        }
        println!();
    }
}

fn solve(hm: &HashMap<Location, char>, starting_cursor: Cursor) -> usize {
    let mut cursors = VecDeque::new();
    cursors.push_back(starting_cursor);

    let mut visited = HashSet::new();
    let mut known_cursors = HashSet::new();

    loop {
        if cursors.is_empty() {
            break;
        }

        let cursor = cursors.pop_front().unwrap();

        let map_element = hm.get(&cursor.location);
        if map_element.is_none() {
            // cursor is out of the map: leaving
            continue;
        }

        if !visited.contains(&cursor.location) {
            visited.insert(cursor.location);
        }


        // check if we already saw this cursor
        if known_cursors.contains(&cursor) {
            continue;
        } else {
            known_cursors.insert(cursor);
        }

        // println!("{:?} {} cursors:{:?}", cursor, visited.len(), cursors);
        // draw(hm, &visited);

        if *map_element.unwrap() == '\\' {
            let cursor = match cursor.direction {
                Direction::North => {
                    Cursor{
                        location: Location{x: cursor.location.x-1, y: cursor.location.y},
                        direction: Direction::West,
                    }
                },
                Direction::East => {
                    Cursor{
                        location: Location{x: cursor.location.x, y: cursor.location.y + 1},
                        direction: Direction::South,
                    }
                },
                Direction::South => {
                    Cursor{
                        location: Location{x: cursor.location.x+1, y: cursor.location.y},
                        direction: Direction::East,
                    }
                },
                Direction::West => {
                    Cursor{
                        location: Location{x: cursor.location.x, y: cursor.location.y - 1},
                        direction: Direction::North,
                    }
                },
            };

            cursors.push_back(cursor);

            continue;

        } else if *map_element.unwrap() == '/' {
            let cursor = match cursor.direction {
                Direction::North => {
                    Cursor{
                        location: Location{x: cursor.location.x+1, y: cursor.location.y},
                        direction: Direction::East,
                    }
                },
                Direction::East => {
                    Cursor{
                        location: Location{x: cursor.location.x, y: cursor.location.y - 1},
                        direction: Direction::North,
                    }
                },
                Direction::South => {
                    Cursor{
                        location: Location{x: cursor.location.x-1, y: cursor.location.y},
                        direction: Direction::West,
                    }
                },
                Direction::West => {
                    Cursor{
                        location: Location{x: cursor.location.x, y: cursor.location.y + 1},
                        direction: Direction::South,
                    }
                },
            };

            cursors.push_back(cursor);

            continue;

        } else if *map_element.unwrap() == '|' {
            if cursor.direction == Direction::East || cursor.direction == Direction::West {
                // Adding 2 cursors
                cursors.push_back(Cursor{
                    location: Location{x: cursor.location.x, y: cursor.location.y - 1},
                    direction: Direction::North,
                });

                cursors.push_back(Cursor{
                    location: Location{x: cursor.location.x, y: cursor.location.y + 1},
                    direction: Direction::South,
                });

                continue;
            }
        } else if *map_element.unwrap() == '-' && (cursor.direction == Direction::North || cursor.direction == Direction::South) {
            // Adding 2 cursors
            cursors.push_back(Cursor{
                location: Location{x: cursor.location.x - 1, y: cursor.location.y},
                direction: Direction::West,
            });

            cursors.push_back(Cursor{
                location: Location{x: cursor.location.x + 1, y: cursor.location.y},
                direction: Direction::East,
            });

            continue;
        }

        // Move ahead
        let cursor = match cursor.direction {
            Direction::North => Cursor{
                location: Location { x: cursor.location.x, y: cursor.location.y - 1 },
                direction: Direction::North,
            },
            Direction::East => Cursor{
                location: Location { x: cursor.location.x + 1, y: cursor.location.y },
                direction: Direction::East,
            },
            Direction::West => Cursor{
                location: Location { x: cursor.location.x - 1, y: cursor.location.y },
                direction: Direction::West,
            },
            Direction::South => Cursor{
                location: Location { x: cursor.location.x, y: cursor.location.y + 1 },
                direction: Direction::South,
            }
        };

        cursors.push_back(cursor);
    }

    visited.len()
}

fn main() {
    let hm = readmap("input.txt");
    let mut max_x = 0;
    let mut max_y = 0;
    for loc in hm.keys() {
        if loc.x > max_x {
            max_x = loc.x;
        }
        if loc.y > max_y {
            max_y = loc.y;
        }
    }

    println!("#1 {}", solve(&hm, Cursor{location: Location{x: 0, y: 0}, direction: Direction::East}));

    let mut max_energy = 0;

    for x in 0..=max_x {
        let energy = solve(&hm, Cursor{location: Location{x, y: 0}, direction: Direction::South});
        if energy > max_energy {
            max_energy = energy;
        }

        let energy = solve(&hm, Cursor{location: Location{x, y: max_y}, direction: Direction::North});
        if energy > max_energy {
            max_energy = energy;
        }
    }

    for y in 0..=max_y {
        let energy = solve(&hm, Cursor{location: Location{x: 0, y}, direction: Direction::East});
        if energy > max_energy {
            max_energy = energy;
        }

        let energy = solve(&hm, Cursor{location: Location{x: max_x, y}, direction: Direction::West});
        if energy > max_energy {
            max_energy = energy;
        }
    }

    println!("#2 {}", max_energy);
}
