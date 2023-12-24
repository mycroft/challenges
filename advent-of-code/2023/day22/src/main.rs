use std::{fs, collections::{VecDeque, HashSet}};

use std::cmp::{min, max};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    id: usize,
    cubes: HashSet<Position>,
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let mut p = value.split(',');
        
        Position {
            x: p.next().unwrap().parse().unwrap(),
            y: p.next().unwrap().parse().unwrap(),
            z: p.next().unwrap().parse().unwrap(),
        }
    }
}

fn get_cubes(p1: &Position, p2: &Position) -> HashSet<Position> {
    let mut cubes = HashSet::new();

    for x in min(p1.x, p2.x)..=max(p1.x, p2.x) {
        for y in min(p1.y, p2.y)..=max(p1.y, p2.y) {
            for z in min(p1.z, p2.z)..=max(p1.z, p2.z) {
                cubes.insert(Position{
                    x, y, z
                });
            }
        }
    }

    cubes
}

impl From<&str> for Brick {
    fn from(value: &str) -> Self {
        let mut p = value.split('~');

        let from : Position = p.next().unwrap().into();
        let to : Position = p.next().unwrap().into();

        Brick{
            id: 0,
            cubes: get_cubes(&from, &to),
        }
    }
    
}

fn read_file(fp: &str) -> VecDeque<Brick> {
    let mut result = VecDeque::new();

    let contents = fs::read_to_string(fp).expect("a file to open");
    let lines = contents.lines();

    for (id, line) in lines.enumerate() {
        let mut brick: Brick = line.into();
        brick.id = id;
        result.push_back(brick);
    }

    result
}

fn fall_brick(bricks: &VecDeque<Brick>, current: &Brick) -> Brick {
    let mut brick = current.clone();

    loop {
        let mut new_brick = Brick{ id: current.id, cubes: HashSet::new() };
    
        for cube_position in &brick.cubes {
            let new_position = Position{
                x: cube_position.x,
                y: cube_position.y,
                z: cube_position.z - 1,
            };
    
            if new_position.z == 0 {
                return brick.clone();
            }
            new_brick.cubes.insert(new_position);
        }
    
        for existing_brick in bricks {
            let intersection = existing_brick.cubes.intersection(&new_brick.cubes).collect::<Vec<&Position>>();
            if !intersection.is_empty() {
                return brick.clone();
            }
        }

        brick = new_brick;
    }
}

fn fall(bricks: &mut VecDeque<Brick>) -> usize {
    let mut count = 0;

    let mut moved = HashSet::new();

    loop {
        let current_brick = bricks.pop_front().unwrap();
        let moved_brick = fall_brick(bricks, &current_brick);

        if moved_brick == current_brick {
            count += 1;
        } else {
            count = 0;

            if !moved.contains(&moved_brick.id) {
                moved.insert(moved_brick.id);
            }
        }

        bricks.push_back(moved_brick);

        if count >= bricks.len() {
            break;
        }
    }

    moved.len()
}

fn solve1(bricks: &VecDeque<Brick>) -> (usize, usize) {
    let mut result = 0;
    let mut bricks = bricks.clone();

    let mut moved_total = 0;

    for n in 0..bricks.len() {
        // remove first element
        let element = bricks.pop_front().unwrap();

        let res = fall(&mut bricks.clone());

        // fall and add 1 to result if nothing moved
        if res == 0 {
            result += 1;
        } else {
            moved_total += res;
        }

        // println!("{n} p1:{result} {res} p2:{moved_total}");

        // queue element back
        bricks.push_back(element);
    }

    (result, moved_total)
}

fn main() {
    let mut bricks = read_file("input.txt");

    fall(&mut bricks);

    // warning: this is taking a while...
    let (p1, p2) = solve1(&bricks);
    println!("#1 {}", p1);
    println!("#2 {}", p2);
}
