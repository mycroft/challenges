/*
 * AOC 2018 day 13
 */
use std::fs;

#[derive(Copy,Clone,Debug,PartialEq)]
enum Direction {
    West,
    East,
    North,
    South,
}

#[derive(Debug,Clone,Copy)]
struct Car {
    id: usize,
    x: usize,
    y: usize,
    d: Direction,
    t: usize,
}

fn step(map: &Vec<Vec<char>>, cars: &Vec<Car>) -> Vec<Car> {
    let mut next_cars_state = vec![];

    for car in cars {
        // retrieve next char according to direction
        let next_coord = match car.d {
            Direction::North => (car.x, car.y - 1),
            Direction::South => (car.x, car.y + 1),
            Direction::West => (car.x - 1, car.y),
            Direction::East => (car.x + 1, car.y),
        };

        let next_char = map[next_coord.1][next_coord.0];
        let mut next_t = car.t;

        let next_direction = match next_char {
            '|' | '-' => {
                car.d
            },
            '+' => {
                let next_direction = match next_t {
                    0 => {
                        // go left
                        match car.d {
                            Direction::North => Direction::West,
                            Direction::West => Direction::South,
                            Direction::South => Direction::East,
                            Direction::East => Direction::North
                        }
                    },
                    1 => {
                        // right ahead
                        car.d
                    }, 
                    2 => {
                        // go right
                        match car.d {
                            Direction::North => Direction::East,
                            Direction::East => Direction::South,
                            Direction::South => Direction::West,
                            Direction::West => Direction::North
                        }
                    },
                    _ => unreachable!()
                };

                next_t += 1;

                if next_t == 3 {
                    next_t = 0;
                }

                next_direction
            }
            '/' => match car.d {
                Direction::North => Direction::East,
                Direction::South => Direction::West,
                Direction::West => Direction::South,
                Direction::East => Direction::North
            }
            '\\' => match car.d {
                Direction::North => Direction::West,
                Direction::South => Direction::East,
                Direction::West => Direction::North,
                Direction::East => Direction::South
            },
            _ => unreachable!()
        };

        let next_car_state = Car{id: car.id, x: next_coord.0, y: next_coord.1, d: next_direction, t: next_t};

        next_cars_state.push(next_car_state);
    }

    next_cars_state
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut rows = vec![];
    let mut cars = vec![];

    let mut n = 0;

    for (lineno, line) in contents.lines().enumerate() {
        let mut row = vec![];

        for (charno, c) in line.chars().enumerate() {
            let c = match c {
                '|' | '/' | '\\' | '-' | '+' | ' ' => c,
                '>' => {
                    cars.push(Car{id: n, x: charno, y: lineno, d: Direction::East, t: 0});
                    n += 1;
                    '-'
                },
                'v' => {
                    cars.push(Car{id: n, x: charno, y: lineno, d: Direction::South, t: 0});
                    n += 1;
                    '|'
                },
                '^' => {
                    cars.push(Car{id: n, x: charno, y: lineno, d: Direction::North, t: 0});
                    n += 1;
                    '|'
                },
                '<' => {
                    cars.push(Car{id: n, x: charno, y: lineno, d: Direction::West, t: 0});
                    n += 1;
                    '-'
                }
                _ => {
                    unreachable!()
                }
            };

            row.push(c);
        }

        rows.push(row);
    }

    let mut first_crash = true;

    loop {
        let cars_next_state = step(&rows, &cars);
        let mut to_remove = vec![];

        for car in &cars_next_state {
            for sub_car in &cars {
                if car.x == sub_car.x && car.y == sub_car.y && car.id != sub_car.id {
                    // println!("1 Crash at {},{}", car.x, car.y);
                    // println!("{:?} & {:?}", car, sub_car);

                    to_remove.push(car);
                    to_remove.push(sub_car);
                    break;
                }
            }

            for sub_car in &cars_next_state {
                if car.x == sub_car.x && car.y == sub_car.y && car.id != sub_car.id {
                    // println!("2 Crash at {},{}", car.x, car.y);
                    // println!("{:?} & {:?}", car, sub_car);

                    to_remove.push(car);
                    to_remove.push(sub_car);
                    break;
                }
            }
        }

        // there might be a case of 2 cars at the same place but different directions. Remove that.
        if to_remove.len() != 4 {
            to_remove.clear();
        } else if first_crash {
            println!("Part #1: {},{}", to_remove[0].x, to_remove[0].y);
            first_crash = false;
        }

        cars = cars_next_state.iter().filter(|x| to_remove.iter().all(|&c| c.id != x.id)).cloned().collect::<Vec<Car>>();

        if cars.len() == 1 {
            println!("Part #2: {},{}", cars[0].x, cars[0].y);
            break;
        }
    }
}
