use intcode::{parse,Machine};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position(isize, isize);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Bot {
    position: Position,
    direction: Direction
}

fn build_map() -> (HashSet<Position>, Bot) {
    let mut hm = HashSet::new();
    let mut machine = Machine::new(&parse("input.txt"));
    machine.run();
    let output = machine.get_output();
    let mut bot = Bot{ position: Position(0, 0), direction: Direction::Up};

    let mut y = 0;
    let mut x = 0;
    for &o in &output {
        match o {
            10 => {
                x = 0;
                y += 1;
                continue;
            },
            35 => {
                hm.insert(Position(x, y));
            },
            46 => {},
            94 => {
                // to handle: all the possible chars.
                // my input reported 94 (up)
                bot = Bot{position: Position(x, y), direction: Direction::Up};
            },
            _ => {
                println!("Got: {}", o);
                unreachable!()
            }
        }
        x += 1;
    }

    (hm, bot)
}

fn find_intersections(points: &HashSet<Position>) -> HashSet<Position> {
    let mut hm = HashSet::new();
    let directions = [
        (0, 1), (0, -1), (1, 0), (-1, 0),
    ];

    for point in points {
        if 4 != directions.iter().filter(|x| points.contains(&Position(point.0 + x.0, point.1 + x.1))).count() {
            continue;
        }
        hm.insert(*point);
    }

    hm
}

fn display(points: &HashSet<Position>, bot: &Bot) {
    let max_x = points.iter().map(|p| p.0).max().unwrap() + 1;
    let max_y = points.iter().map(|p| p.1).max().unwrap() + 1;

    for y in 0..max_y {
        for x in 0..max_x {
            if points.contains(&Position(x, y)) {
                print!("#");
            } else if Position(x, y) == bot.position {
                print!("^");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn get_directions(current: Direction) -> Vec<Direction> {
    match current {
        Direction::Up => [Direction::Left, Direction::Right],
        Direction::Down => [Direction::Right, Direction::Left],
        Direction::Left => [Direction::Down, Direction::Up],
        Direction::Right => [Direction::Up, Direction::Down]
    }.to_vec()
}

fn get_coords(current: Direction) -> (isize, isize) {
    match current {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0)
    }
}

fn get_path(hm: &mut HashSet<Position>, bot: Bot) -> Vec<(Direction, usize)> {
    let mut current_direction = bot.direction;
    let mut current_position = bot.position;
    let mut result = Vec::new();

    hm.insert(bot.position);

    loop {
        // find direction
        let possible_directions = get_directions(current_direction);
        let dir_left = get_coords(possible_directions[0]);
        let dir_right = get_coords(possible_directions[1]);
        let pos_left = Position(current_position.0 + dir_left.0, current_position.1 + dir_left.1);
        let pos_right = Position(current_position.0 + dir_right.0, current_position.1 + dir_right.1);
        let mut direction_shift = dir_left;
        let mut local_result = (Direction::Down, 0);

        if hm.contains(&pos_left) {
            // println!("We're going left.");

            current_direction = possible_directions[0];
            local_result.0 = Direction::Left;
            direction_shift = dir_left;
        } else if hm.contains(&pos_right) {
            // println!("We're going right");

            current_direction = possible_directions[1];
            local_result.0 = Direction::Right;
            direction_shift = dir_right;
        } else {
            // can not move anymore.
            // println!("Can not move anymore... direction:{current_direction:?} position:{current_position:?}");
            break;
        }

        // then compute how many steps
        while hm.contains(&current_position) {
            current_position = Position(
                current_position.0 + direction_shift.0,
                current_position.1 + direction_shift.1,
            );
            local_result.1 += 1;
        }

        // return a position before
        current_position = Position(
            current_position.0 - direction_shift.0,
            current_position.1 - direction_shift.1,
        );

        // remove one to not go too far.
        local_result.1 -= 1;
        result.push(local_result);
    }

    result
}

/*
Running get_path allowed to get:
[
A    (Left, 5), (Right, 9), (Left, 7), (Left, 11),
B    (Left, 7), (Right, 9), (Right, 11), (Left, 7), (Left, 7), 
A    (Left, 5), (Right, 9), (Left, 7), (Left, 11),
B    (Left, 7), (Right, 9), (Right, 11), (Left, 7), (Left, 7), 
C    (Left, 5), (Left, 5), (Left, 11), 
C    (Left, 5), (Left, 5), (Left, 11), 
B    (Left, 7), (Right, 9), (Right, 11), (Left, 7), (Left, 7), 
A    (Left, 5), (Right, 9), (Left, 7), (Left, 11),
B    (Left, 7), (Right, 9), (Right, 11), (Left, 7), (Left, 7), 
C    (Left, 5), (Left, 5), (Left, 11)
]
First, you will be prompted for the main movement routine.
The main routine may only call the movement functions: A, B, or C.
Supply the movement functions to use as ASCII text, separating them with commas
(,, ASCII code 44), and ending the list with a newline (ASCII code 10).
For example, to call A twice, then alternate between B and C three times, provide the string A,A,B,C,B,C,B,C and then a newline.

*/
fn get_manual_path() -> Vec<isize> {
    let mut result = Vec::new();

    let main = String::from("A,B,A,B,C,C,B,A,B,C");
    let s_a = String::from("L,4,R,8,L,6,L,10");
    let s_b = String::from("L,6,R,8,R,10,L,6,L,6");
    let s_c = String::from("L,4,L,4,L,10");

    for s in [main, s_a, s_b, s_c] {
        for c in s.chars() {
            result.push(c as isize);
        }
        result.push(10);
    }

    // not showing the feed
    result.push('n' as isize);
    result.push(10);

    // println!("{result:?}");

    result
}

fn main() {
    let (mut hm, bot) = build_map();
    let intersections = find_intersections(&hm);

    // println!("{:?}", hm);
    // println!("{:?}", intersections);
    // display(&hm, &bot);

    println!("#1 {}", intersections.iter().map(|p| p.0 * p.1).sum::<isize>()); // 3192

    // let path = get_path(&mut hm, bot);
    // println!("{path:?}");

    let path = get_manual_path();
    // println!("{path:?}");

    let mut code = parse("input.txt");
    code[0] = 2;

    let mut machine = Machine::new(&code);
    for input in path {
        machine.add_input(input);
    }
    let ret = machine.run();
    // let output = machine.get_output();
    // for c in &output {
    //    print!("{}", (*c as u8) as char);
    // }
    // println!();

    // println!("#2 {}", output[output.len() - 1]);
    println!("#2 {ret}"); // 684691
}
