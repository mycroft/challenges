use intcode::{parse, Machine};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

#[derive(Debug)]
struct Robot {
    location: Point,
    direction: Direction
}

impl Robot {
    fn new() -> Self {
        Self {
            location: Point { x: 0, y: 0 },
            direction: Direction::Up,
        }
    }    
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Black,
    White
}

fn run_robot(robot: &mut Robot, machine: &mut Machine, initial: Color) -> HashMap<Point, Color> {
    let mut hm: HashMap<Point, Color> = HashMap::new();

    hm.insert(robot.location, initial);

    loop {
        // where the robot is?
        let current_color = *hm.get(&robot.location).unwrap_or(&Color::Black);

        let input = match current_color {
            Color::Black => 0,
            Color::White => 1,
        };

        machine.add_input(input);
        machine.run();

        let output = machine.get_output();
        machine.clean_output();

        // 1st output: 0 means to paint the panel black, and 1 means to paint the panel white.
        let new_color = match output[0] {
            0 => Color::Black,
            1 => Color::White,
            _ => unreachable!()
        };
        hm.insert(robot.location, new_color);

        // 2nd output: 0 means it should turn left 90 degrees, and 1 means it should turn right 90 degrees.
        let new_direction = match output[1] {
            0 => match robot.direction {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            },
            1 => match robot.direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
            _ => unreachable!()
        };

        let new_location = match new_direction {
            Direction::Up => Point { x: robot.location.x, y: robot.location.y - 1 },
            Direction::Left => Point { x: robot.location.x - 1, y: robot.location.y },
            Direction::Down => Point { x: robot.location.x, y: robot.location.y + 1 },
            Direction::Right => Point { x: robot.location.x + 1, y: robot.location.y },
        };

        // apply new location, direction
        robot.direction = new_direction;
        robot.location = new_location;

        if machine.is_finished() {
            break;
        }

    }

    hm
}

fn display(hm: &HashMap<Point, Color>) {
    let min_x = hm.iter().map(|(p, _)| p.x).min().unwrap();
    let min_y = hm.iter().map(|(p, _)| p.y).min().unwrap();
    let max_x = hm.iter().map(|(p, _)| p.x).max().unwrap();
    let max_y = hm.iter().map(|(p, _)| p.y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let color = *hm.get(&Point{x, y}).unwrap_or(&Color::Black);
            match color {
                Color::Black => print!(" "),
                Color::White => print!("#"),
            }
        }
        println!()
    }
}

fn main() {
    let code = parse("input.txt");
    let mut machine = Machine::new(&code);
    let mut robot = Robot::new();

    let hm = run_robot(&mut robot, &mut machine, Color::Black);
    println!("#1 {}", hm.len()); // 2428

    let mut machine = Machine::new(&code);
    let mut robot = Robot::new();
    let hm = run_robot(&mut robot, &mut machine, Color::White);

    println!("#2:");
    display(&hm); // RJLFBUCU
}

#[test]
fn test_input_step1() {
    let code = parse("input.txt");
    let mut machine = Machine::new(&code);
    let mut robot = Robot::new();

    let hm = run_robot(&mut robot, &mut machine, Color::Black);
    assert_eq!(
        2428,
        hm.len()
    );
}

#[test]
fn test_input_step2() {
    let code = parse("input.txt");
    let mut machine = Machine::new(&code);
    let mut robot = Robot::new();

    let hm = run_robot(&mut robot, &mut machine, Color::White);
    assert_eq!(
        249,
        hm.len()
    );
}