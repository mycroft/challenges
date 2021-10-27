use std::fs;

#[derive(Copy, Clone, Debug)]
struct Car {
    x: usize,
    y: usize,
    c: char,
    turn: usize,
}

fn get_next_coords(car: Car) -> (usize, usize) {
    let vector = match car.c {
        '>' => (1, 0),
        '<' => (-1, 0),
        'v' => (0, 1),
        '^' => (0, -1),
        _ => unreachable!(),
    };

    (
        (car.x as i32 + vector.0) as usize,
        (car.y as i32 + vector.1) as usize,
    )
}

fn display_roads(roads: &Vec<Vec<char>>) {
    for road in roads {
        println!("{}", road.iter().collect::<String>());
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut roads : Vec<Vec<char>> = vec![];
    let mut cars : Vec<Car> = vec![];

    for (y, line) in lines.iter().enumerate() {
        let line = line.chars().collect::<Vec<char>>();

        line.iter()
            .enumerate()
            .filter(|&(_, c)| *c != '|' && *c != '-' && *c != '/' && *c != '\\' && *c != '+' && *c != ' ')
            .map(|(x, c)| {
                cars.push(Car{x: x, y: y, c: *c, turn: 0})
            }).count();

        roads.push(line);
    }

    // in roads, replace the cars.
    for car in &cars {
        roads[car.y][car.x] = match car.c {
            '<' | '>' => '-',
            '^' | 'v' => '|',
            _ => unreachable!(),
        };
    }

    let mut first_crashed = false;
    let mut removed_cars : Vec<usize> = vec![];

    // display_roads(&roads);

    loop {
        for car_id in 0..cars.len() {
            // Move the car.
            let mut car = cars[car_id];
            if removed_cars.contains(&car_id) {
                continue;
            }

            let next_coords = get_next_coords(car);

            // Check any collision.
            let touched_cars = cars
                .iter()
                .enumerate()
                .filter(|(idx, _)| !removed_cars.contains(idx))
                .filter(|(_, car)| car.x == next_coords.0 && car.y == next_coords.1).map(|(idx, _)| idx)
                .collect::<Vec<usize>>();

            if touched_cars.len() != 0 {
                if !first_crashed {
                    println!("#1: {},{}", next_coords.0, next_coords.1);
                    first_crashed = true;
                }
                
                removed_cars.push(car_id);
                removed_cars.push(touched_cars[0]);

                continue;
            }

            if ['|', '-'].iter().any(|&x| x == roads[next_coords.1 as usize][next_coords.0 as usize]) {
                cars[car_id] = Car{x: next_coords.0, y: next_coords.1, c: car.c, turn: car.turn };
                continue;
            }

            // direction is changing.
            car.c =  match roads[next_coords.1 as usize][next_coords.0 as usize] {
                '/' => {
                    match car.c {
                        '^' => '>',
                        'v' => '<',
                        '<' => 'v',
                        '>' => '^',
                        _ => unreachable!(),
                    }
                },
                '\\' => {
                    match car.c {
                        '^' => '<',
                        'v' => '>',
                        '<' => '^',
                        '>' => 'v',
                        _ => unreachable!(),
                    }
                },
                '+' => {
                    match car.c {
                        '^' => match car.turn % 3 {
                            0 => '<',
                            1 => car.c,
                            2 => '>',
                            _ => unreachable!(),
                        },
                        '>' => match car.turn % 3 {
                            0 => '^',
                            1 => car.c,
                            2 => 'v',
                            _ => unreachable!(),
                        },
                        'v' => match car.turn % 3 {
                            0 => '>',
                            1 => car.c,
                            2 => '<',
                            _ => unreachable!(),
                        },
                        '<' => match car.turn % 3 {
                            0 => 'v',
                            1 => car.c,
                            2 => '^',
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    }
                },
                _ => unreachable!(),
            };

            if roads[next_coords.1 as usize][next_coords.0 as usize] == '+' {
                car.turn += 1;
            }

            cars[car_id] = Car{x: next_coords.0, y: next_coords.1, c: car.c, turn: car.turn };
        }

        cars = cars.iter().enumerate().filter(|(id, _)| !removed_cars.contains(id)).map(|(_, car)| *car).collect();
        removed_cars.clear();

        if cars.len() < 2 {
            break;
        }
    }
    
    let remaining_cars = cars
        .iter()
        .enumerate()
        .filter(|(idx, _)| !removed_cars.contains(idx))
        .map(|(_, car)| car)
        .collect::<Vec<&Car>>();

    println!("#2: {},{}", remaining_cars[0].x, remaining_cars[0].y);
}
