use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Coord,
    velocity: Coord,
}

fn read_input(fp: &str) -> Vec<Robot> {
    let contents = fs::read_to_string(fp).expect("Error reading the file");

    let mut robots = Vec::new();

    for line in contents.lines() {
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let left_parts = parts[0].split('=').collect::<Vec<&str>>();
        let coords_left = left_parts[1].split(',').collect::<Vec<&str>>();

        let right_parts = parts[1].split('=').collect::<Vec<&str>>();
        let coords_right = right_parts[1].split(',').collect::<Vec<&str>>();

        let position = Coord {
            x: coords_left[0].parse().unwrap(),
            y: coords_left[1].parse().unwrap(),
        };

        let velocity = Coord {
            x: coords_right[0].parse().unwrap(),
            y: coords_right[1].parse().unwrap(),
        };

        let robot = Robot {
            position,
            velocity,
        };

        robots.push(robot);
    }

    robots
}

fn dump(robots: &Vec<Robot>, space_size: Coord) {
    for y in 0..space_size.y {
        for x in 0..space_size.x {
            let mut found = false;
            for robot in robots {
                if robot.position == (Coord {x, y}) {
                    print!("#");
                    found = true;
                    break;
                }
            }

            if !found {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn dump_into_file(robots: &Vec<Robot>, space_size: Coord, fp: &str) {
    let mut contents = String::new();

    for y in 0..space_size.y {
        for x in 0..space_size.x {
            let mut found = false;
            for robot in robots {
                if robot.position == (Coord {x, y}) {
                    contents.push('#');
                    found = true;
                    break;
                }
            }

            if !found {
                contents.push('.');
            }
        }
        contents.push('\n');
    }

    fs::write(fp, contents).expect("Error writing to file");
}

fn next(robot: &mut Robot, space_size: Coord) {
    let mut new_robot = Robot {
        position: Coord {
            x: (robot.position.x + robot.velocity.x) % space_size.x,
            y: (robot.position.y + robot.velocity.y) % space_size.y,
        },
        velocity: robot.velocity,
    };

    if new_robot.position.x < 0 {
        new_robot.position.x += space_size.x;
    }

    if new_robot.position.y < 0 {
        new_robot.position.y += space_size.y;
    }

    // println!("{:?} -> {:?}", robot, new_robot);
    *robot = new_robot;
}

fn play(robots: &mut [Robot], space_size: Coord) {
    for robot in robots.iter_mut() {
        next(robot, space_size);
    }
}

fn count_quadrants(robots: &Vec<Robot>, space_size: Coord) -> [isize; 4] {
    let mut quadrants = [0; 4];

    for robot in robots {
        match (
            robot.position.x.cmp(&(space_size.x / 2)),
            robot.position.y.cmp(&(space_size.y / 2)),
        ) {
            (std::cmp::Ordering::Less, std::cmp::Ordering::Less) => quadrants[0] += 1,
            (std::cmp::Ordering::Less, std::cmp::Ordering::Greater) => quadrants[2] += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => quadrants[1] += 1,
            (std::cmp::Ordering::Greater, std::cmp::Ordering::Greater) => quadrants[3] += 1,
            _ => (),
        }
        /*
        if robot.position.x < space_size.x / 2 {
            if robot.position.y < space_size.y / 2 {
                quadrants[0] += 1;
            } else if robot.position.y > space_size.y / 2 {
                quadrants[2] += 1;
            }
        } else if robot.position.x > space_size.x / 2 {
            if robot.position.y < space_size.y / 2 {
                quadrants[1] += 1;
            } else if robot.position.y > space_size.y / 2 {
                quadrants[3] += 1;
            }
        };
         */
    }

    quadrants
}


fn main() {
    let mut robots = read_input("input.txt");
    let space_size = Coord {x: 101, y: 103};

    for _ in 0..100 {
        play(&mut robots, space_size);
    }

    dump(&robots, space_size);
    println!("#1 {}", count_quadrants(&robots, space_size).iter().product::<isize>());

    let mut robots = read_input("input.txt");
    let space_size = Coord {x: 101, y: 103};

    for n in 1..=10000 {
        play(&mut robots, space_size);
        dump_into_file(&robots, space_size, &format!("output/{:04}.txt", n));
    }
}
