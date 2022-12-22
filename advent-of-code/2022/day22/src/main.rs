use std::fs::read_to_string;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Ground,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    TurnLeft,
    TurnRight,
    Move(isize)
}

fn parse(fp: &str) -> (HashMap<Pos, Tile>, Vec<Instruction>) {
    let mut result = (HashMap::new(), Vec::new());

    let contents = read_to_string(fp).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut current_line_idx = 1;

    // parse map
    for line in &lines {
        if line.is_empty() {
            break;
        }

        line.chars().enumerate().map(|(idx, c)| {
            if c != ' ' {
                result.0.insert(
                    Pos{x: idx as isize + 1, y: current_line_idx},
                    match c {
                        '.' => Tile::Ground,
                        '#' => Tile::Wall,
                        _ => unreachable!()
                    }
                );
            }
        }).count();

        current_line_idx += 1;
    }

    // parse instructions
    let instr_line = lines.last().unwrap();
    let mut token = String::new();

    for c in instr_line.chars() {
        match c {
            'L' => {
                result.1.push(Instruction::Move(token.parse::<isize>().unwrap()));
                token = String::new();
                result.1.push(Instruction::TurnLeft);
            },
            'R' => {
                result.1.push(Instruction::Move(token.parse::<isize>().unwrap()));
                token = String::new();
                result.1.push(Instruction::TurnRight);
            },
            _ => {
                token.push(c);
            }
        };
    }

    if token != "" {
        result.1.push(Instruction::Move(token.parse::<isize>().unwrap()));
    }

    result
}

fn step1(m: &HashMap<Pos, Tile>, instructions: &Vec<Instruction>) -> isize {
    // find starting position
    let mut pos = Pos{x: 1, y: 1};
    let mut current_direction = (1, 0); // going to right.

    // find map size.
    let mut max_x = 1;
    let mut max_y = 1;

    for c in m {
        if c.0.x > max_x { max_x = c.0.x };
        if c.0.y > max_y { max_y = c.0.y };
    }

    while !m.contains_key(&pos) {
        pos = Pos{x: pos.x + current_direction.0, y: pos.y + current_direction.1};
    }

    for instr in instructions {
        // println!("current position: {pos:?} current direction: {current_direction:?} instruction: {instr:?}");

        match instr { 
            Instruction::TurnLeft => {
                current_direction = match current_direction {
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    _ => unreachable!()
                }
            },
            Instruction::TurnRight => {
                current_direction = match current_direction {
                    (1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    (0, -1) => (1, 0),
                    _ => unreachable!()
                }
            },
            Instruction::Move(x) => {
                for _ in 0..*x {
                    let mut new_pos = Pos{x: pos.x + current_direction.0, y: pos.y + current_direction.1};
                    // if new_pos does not exist, find next new_pos.
                    if !m.contains_key(&new_pos) {
                        // -1 => 99
                        // 1 => 0
                        new_pos = Pos{
                            x: match current_direction.0 { -1 => max_x, 0 => pos.x, 1 => 1, _ => unreachable!() },
                            y: match current_direction.1 { -1 => max_y, 0 => pos.y, 1 => 1, _ => unreachable!() },
                        };

                        // continue until something exists.
                        while !m.contains_key(&new_pos) {
                            new_pos = Pos{x: new_pos.x + current_direction.0, y: new_pos.y + current_direction.1};
                        }                    
                    }

                    // check if new_pos is a wall or not. If so, just do not move
                    if *m.get(&new_pos).unwrap() == Tile::Wall {
                        break;
                    } else {
                        // else, could move
                        pos = new_pos;
                    }
                }
            }
        }
    }

    1000 * pos.y + 4 * pos.x + match current_direction {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!()
    }
}

// Our cube is like the following:
//
//  12
//  3
// 45
// 6
//
// a face is 50x50.

// which_face returns the we're one, based on our own input.
fn which_face(x: isize, y: isize) -> isize {
    if y >= 1 && y <= 50 {
        if x >= 51 && x <= 100 {
            1
        } else if x >= 101 && x <= 150 {
            2
        } else {
            unreachable!()
        }
    } else if y >= 51 && y <= 100 {
        if x >= 51 && x <= 100 {
            3
        } else {
            unreachable!()
        }
    } else if y >= 101 && y <= 150 {
        if x >= 1 && x <= 50 {
            4
        } else if x >= 51 && x <= 100 {
            5
        } else {
            unreachable!()
        }
    } else if y >= 151 && y <= 200 {
        if x >= 1 && x <= 50 {
            6
        } else {
            unreachable!()
        }
    } else {
        println!("x: {} y: {}", x, y);
        unreachable!()
    }
}

// change_face will determine the new x, y and direction to follow after a face change. 
fn change_face(x: isize, y: isize, direction: (isize, isize)) -> (isize, isize, (isize, isize)) {
    let current_face = which_face(x, y);
    let new_face ;
    let new_x;
    let new_y;
    let new_direction;

    match current_face {
        1 => {
            match direction {
                (0, -1) => { new_face = 6; new_direction = (1, 0); new_x = 1; new_y = 150 + x - 50; }
                (-1, 0) => { new_face = 4; new_direction = (1, 0); new_x = 1; new_y = 100 + (50 - y + 1); }
                _ => unreachable!()
            }
        },
        2 => {
            match direction {
                (0, -1) => { new_face = 6; new_direction = (0, -1); new_x = x - 100; new_y = 200; }
                (1, 0) => { new_face = 5; new_direction = (-1, 0); new_x = 100; new_y = 100 + (50 - y + 1); }
                (0, 1) => { new_face = 3; new_direction = (-1, 0); new_x = 100; new_y = x - 50; }
                _ => unreachable!()
            }
        },
        3 => {
            match direction {
                (1, 0) => { new_face = 2; new_direction = (0, -1); new_x = 100 + (y - 50); new_y = 50; }
                (-1, 0) => { new_face = 4; new_direction = (0, 1); new_x = y - 50; new_y = 101; }
                _ => unreachable!()
            }
        },
        4 => {
            match direction {
                (-1, 0) => { new_face = 1; new_direction = (1, 0); new_x = 51; new_y = 50 - (y - 100) + 1; }
                (0, -1) => { new_face = 3; new_direction = (1, 0); new_x = 51; new_y = 50 + x; }
                _ => unreachable!()
            }
        },
        5 => {
            match direction {
                (1, 0) => { new_face = 2; new_direction = (-1, 0); new_x = 150; new_y = 150 - y + 1; }
                (0, 1) => { new_face = 6; new_direction = (-1, 0); new_x = 50; new_y = 150 + (x - 50); }
                _ => unreachable!()
            }
        },
        6 => {
            match direction {
                (1, 0) => { new_face = 5; new_direction = (0, -1); new_x = y - 150 + 50; new_y = 150; }
                (0, 1) => { new_face = 2; new_direction = (0, 1); new_x = x + 100; new_y = 1; }
                (-1, 0) => { new_face = 1; new_direction = (0, 1); new_x = 50 + (y - 150); new_y = 1; }
                _ => unreachable!()
            }
        },
        _ => unreachable!()
    }

    assert_eq!(
        new_face,
        which_face(new_x, new_y)
    );

    (new_x, new_y, new_direction)
}


fn step2(m: &HashMap<Pos, Tile>, instructions: &Vec<Instruction>) -> isize {
    // find starting position
    let mut pos = Pos{x: 1, y: 1};
    let mut current_direction = (1, 0); // going to right.

    // find map size.
    let mut max_x = 1;
    let mut max_y = 1;

    for c in m {
        if c.0.x > max_x { max_x = c.0.x };
        if c.0.y > max_y { max_y = c.0.y };
    }

    while !m.contains_key(&pos) {
        pos = Pos{x: pos.x + current_direction.0, y: pos.y + current_direction.1};
    }

    for instr in instructions {
        // println!("current position: {pos:?} current direction: {current_direction:?} instruction: {instr:?}");

        match instr { 
            Instruction::TurnLeft => {
                current_direction = match current_direction {
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    _ => unreachable!()
                }
            },
            Instruction::TurnRight => {
                current_direction = match current_direction {
                    (1, 0) => (0, 1),
                    (0, 1) => (-1, 0),
                    (-1, 0) => (0, -1),
                    (0, -1) => (1, 0),
                    _ => unreachable!()
                }
            },
            Instruction::Move(x) => {
                for _ in 0..*x {
                    let mut new_pos = Pos{x: pos.x + current_direction.0, y: pos.y + current_direction.1};
                    let mut new_current_direction = None;

                    if !m.contains_key(&new_pos) {
                        let (new_x, new_y, new_direction) = change_face(pos.x, pos.y, current_direction);

                        new_pos = Pos{x: new_x, y: new_y};
                        new_current_direction = Some(new_direction);
                    }

                    // println!("new pos: {new_pos:?}");

                    // check if new_pos is a wall or not. If so, just do not move
                    if *m.get(&new_pos).unwrap() == Tile::Wall {
                        break;
                    } else {
                        // else, could move
                        pos = new_pos;

                        if let Some(new_current_direction) = new_current_direction {
                            current_direction = new_current_direction;
                        }
                    }
                }
            }
        }
    }

    1000 * pos.y + 4 * pos.x + match current_direction {
        (1, 0) => 0,
        (0, 1) => 1,
        (-1, 0) => 2,
        (0, -1) => 3,
        _ => unreachable!()
    }
}

fn main() {
    let m = parse("input.txt");
    let mut result = step1(&m.0, &m.1);
    println!("#1 {result}"); // 66292

    result = step2(&m.0, &m.1);
    println!("#2 {result}")
}


#[test]
fn test_sample() {
    let m = parse("input.txt_test");
    assert_eq!(
        6032,
        step1(&m.0, &m.1)
    );
}

#[test]
fn test_change_face() {
    // face 1
    assert_eq!(
        (1, 151, (1, 0)),
        change_face(51, 1, (0, -1))
    );

    assert_eq!(
        (1, 150, (1, 0)),
        change_face(51, 1, (-1, 0))
    );

    // face 2
    assert_eq!(
        (1, 200, (0, -1)),
        change_face(101, 1, (0, -1))
    );

    assert_eq!(
        (100, 150, (-1, 0)),
        change_face(150, 1, (1, 0))
    );

    assert_eq!(
        (100, 90, (-1, 0)),
        change_face(140, 50, (0, 1))
    );

    // face 3
    assert_eq!(
        (117, 50, (0, -1)),
        change_face(100, 67, (1, 0))
    );

    assert_eq!(
        (1, 101, (0, 1)),
        change_face(51, 51, (-1, 0))
    );

    // face 4
    assert_eq!(
        (51, 51, (1, 0)),
        change_face(1, 101, (0, -1))
    );

    assert_eq!(
        (51, 40, (1, 0)),
        change_face(1, 111, (-1, 0))
    );

    // face 5
    assert_eq!(
        (150, 1, (-1, 0)),
        change_face(100, 150, (1, 0))
    );

    assert_eq!(
        (50, 175, (-1, 0)),
        change_face(75, 150, (0, 1))
    );

    // face 6
    assert_eq!(
        (75, 150, (0, -1)),
        change_face(50, 175, (1, 0))
    );
    
    assert_eq!(
        (110, 1, (0, 1)),
        change_face(10, 200, (0, 1))
    );

    assert_eq!(
        (75, 1, (0, 1)),
        change_face(1, 175, (-1, 0))
    );

}
