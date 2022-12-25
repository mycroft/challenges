use std::fs::read_to_string;
use std::collections::HashMap;
use pathfinding::prelude::bfs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl From<&PosState> for Pos {
    fn from(ps: &PosState) -> Self {
        Pos { x: ps.x, y: ps.y }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct PosState {
    x: isize,
    y: isize,
    t: usize,
}

impl From<Pos> for PosState {
    fn from(p: Pos) -> Self {
        PosState { x: p.x, y: p.y, t: 0 }
    }
}

impl PartialEq for PosState {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
    fn ne(&self, other: &Self) -> bool {
        self.eq(other)
    }
}

impl Eq for PosState {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Direction {
    x: isize,
    y: isize,
}

impl Direction {
    fn inverse(&self) -> Self {
        Direction {
            x: self.x * -1,
            y: self.y * -1
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MapSize {
    x: isize,
    y: isize,
}

// returns the map. The wind can blow at 0, 0. Starting position is x:0, y:-1
fn parse(fp: &str) -> (HashMap<Pos, Direction>, MapSize, Pos, Pos) {
    let contents = read_to_string(fp).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut hm = HashMap::new();
    let mut starting_pos: Pos = Pos{x: 0, y: 0};
    let mut ending_pos: Pos = Pos{x: 0, y: 0};

    for (y, line) in lines.iter().enumerate() {
        if line.chars().nth(2).unwrap() == '#' {
            if y == 0 {
                starting_pos = Pos{
                    x: line.chars().position(|c| c == '.').unwrap() as isize - 1,
                    y: y as isize - 1
                };
            } else {
                ending_pos = Pos{
                    x: line.chars().position(|c| c == '.').unwrap() as isize - 1,
                    y: y as isize - 1
                };
            }
            continue;
        }

        for (x, c) in line.chars().enumerate() {
            if let Some(direction) = match c {
                '<' => Some(Direction{x: -1, y: 0}),
                '>' => Some(Direction{x: 1, y: 0}),
                'v' => Some(Direction{x: 0, y: 1}),
                '^' => Some(Direction{x: 0, y: -1}),
                _ => None,
            } {
                hm.insert(
                    Pos{x: x as isize - 1, y: y as isize - 1},
                    direction
                );
            }
        }
    }

    (hm, MapSize{x: ending_pos.x + 1, y: ending_pos.y}, starting_pos, ending_pos)
}

// check if there is wind in giving position at time t
fn has_wind(m: &HashMap<Pos, Direction>, ms: &MapSize, p: &Pos, t: usize) -> bool {
    // we check if there is wind in given direction t steps aways
    let directions = [
        Direction{x: -1, y: 0},
        Direction{x: 1, y: 0},
        Direction{x: 0, y: 1},
        Direction{x: 0, y: -1},
    ];

    for direction in directions {
        let mut current_pos = *p;

        // to optimize...
        current_pos.x += direction.x * t as isize;
        while current_pos.x < 0 {
            current_pos.x += ms.x;
        }
        current_pos.x %= ms.x;

        current_pos.y += direction.y * t as isize;
        while current_pos.y < 0 {
            current_pos.y += ms.y;
        }
        current_pos.y %= ms.y;

        // println!("At {:?} - {:?} after {t}", current_pos, m.get(&current_pos));
        
        if m.contains_key(&current_pos) {
            if m.get(&current_pos).unwrap().inverse() == direction {
                return true;
            }
        }
    }

    false
}

fn display(m: &HashMap<Pos, Direction>, ms: &MapSize, t: usize) {
    for y in 0..ms.y {
        for x in 0..ms.x {
            if has_wind(&m, &ms, &Pos{x, y}, t) {
                print!("x");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn successors(m: &HashMap<Pos, Direction>, ms: &MapSize, ps: &PosState, start: Pos, end: Pos, start_t: usize) -> Vec<PosState> {
    let directions = [
        Direction{x: -1, y: 0},
        Direction{x: 1, y: 0},
        Direction{x: 0, y: 1},
        Direction{x: 0, y: -1},
    ];

    let mut result = Vec::new();

    // wait only if there is no blizzard.
    let pos: Pos = ps.into();
    if pos == end || pos == start || !has_wind(m, ms, &ps.into(), ps.t + 1 + start_t) {
        result.push(
            PosState { x: ps.x, y: ps.y, t: ps.t + 1 }
        );
    }

    for direction in directions {
        // go to dir
        let mut pos: Pos = ps.into();
        pos.x += direction.x;
        pos.y += direction.y;

        if pos.x < 0 || pos.x >= ms.x {
            continue;
        }

        if (pos.y < 0 || pos.y >= ms.y) && pos != end && pos != start {
            continue;
        }

        if pos == end || pos == start || !has_wind(m, ms, &pos, ps.t + 1 + start_t) {
            result.push(
                PosState { x: pos.x, y: pos.y, t: ps.t + 1 }
            );
        }
    }

    // successors for PosState { x: 5, y: 3, t: 17 } are [PosState { x: 4, y: 3, t: 18 }]
    // println!("successors for {ps:?} are {result:?}");

    result
}

fn play(m: &HashMap<Pos, Direction>, ms: MapSize, start: Pos, end: Pos, start_t: usize) -> usize {
    let res = bfs(
        &PosState::from(start),
        |ps| successors(&m, &ms, &ps, start, end, start_t),
        |ps| *ps == PosState::from(end)
    ).unwrap();

    res.len() - 1 + start_t
}

fn step1(fp: &str) -> usize {
    let (m, ms, start, end) = parse(fp);

    play(&m, ms, start, end, 0)
}

fn step2(fp: &str) -> usize {
    let (m, ms, start, end) = parse(fp);

    let t = play(&m, ms, start, end, 0);
    let t = play(&m, ms, end, start, t);
    let t = play(&m, ms, start, end, t);

    t
}

fn main() {
    println!("#1 {}", step1("input.txt")); // 301
    println!("#2 {}", step2("input.txt")); // ?
}

#[test]
fn test_parsing() {
    let (m, ms, _, _) = parse("input.txt_test");
    assert_eq!(
        MapSize{x: 6, y: 4},
        ms,
    );

    assert_eq!(
        Direction{x: 1, y: 0},
        *m.get(&Pos{x: 0, y: 0}).unwrap()
    );
}

#[test]
fn test_has_wind() {
    let (m, ms, _, _) = parse("input.txt_test");
    assert_eq!(
        true,
        has_wind(&m, &ms, &Pos{x: 0, y: 0}, 0),
    );
    
    assert_eq!(
        false,
        has_wind(&m, &ms,  &Pos{x: 0, y: 0}, 1),
    );

    assert_eq!(
        false,
        has_wind(&m, &ms,  &Pos{x: 0, y: 0}, 16),
    );

    assert_eq!(
        true,
        has_wind(&m, &ms,  &Pos{x: 1, y: 0}, 16),
    );
}

#[test]
fn test_sample() {
    assert_eq!(
        18,
        step1("input.txt_test")
    );
}

#[test]
fn test_sample_step2() {
    assert_eq!(
        54,
        step2("input.txt_test")
    );
}