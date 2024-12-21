use std::fs;
use std::collections::HashMap;
use pathfinding::directed::astar::astar_bag;

const DOOR_KEYBOARD : [[char; 3]; 4] = [
    ['7', '8', '9'],
    ['4', '5', '6'],
    ['1', '2', '3'],
    ['.', '0', 'A'],
];

const ROBOT_KEYBOARD : [[char; 3]; 2] = [
    ['.', '^', 'A'],
    ['<', 'v', '>'],
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    A
}

impl Direction {
    fn char(&self) -> char {
        match self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
            Direction::A => 'A',
        }
    }
}

fn read_input(fp: &str) -> Vec<String> {
    fs::read_to_string(fp)
        .expect("Can't read input.txt")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn find_shortest_paths(highest: bool, from: char, to: char) -> Vec<Vec<char>> {
    let keyboard : &[[char; 3]] = if highest { &DOOR_KEYBOARD } else { &ROBOT_KEYBOARD };
    let res = astar_bag(
        &(from, Direction::Up.char()),
        |&c| {
            let mut res = Vec::new();
            for (y, row) in keyboard.iter().enumerate() {
                for (x, &key) in row.iter().enumerate() {
                    if key == c.0 {
                        for dir in &[Direction::Up, Direction::Left, Direction::Down, Direction::Right] {
                            let (dy, dx) = match dir {
                                Direction::Up => (-1, 0),
                                Direction::Down => (1, 0),
                                Direction::Left => (0, -1),
                                Direction::Right => (0, 1),
                                _ => unreachable!(),
                            };
                            let ny = y as isize + dy;
                            let nx = x as isize + dx;
                            if ny >= 0 && ny < keyboard.len() as isize && nx >= 0 && nx < keyboard[0].len() as isize {
                                let nkey = keyboard[ny as usize][nx as usize];
                                if nkey != '.' {
                                    res.push(((nkey, dir.char()), 1));
                                }
                            }
                        }
                    }
                }
            }
            res
        },
        |&_c| 0,
        |&c| c.0 == to,
    );

    let mut solutions = Vec::new();

    for path in res.unwrap().0 {
        let mut solution : Vec<char> = path.iter().skip(1).map(|(_, dir)| *dir).collect();
        solution.push(Direction::A.char());
        solutions.push(solution);
    }

    solutions
}

fn find_shortest_sequence(
    cache: &mut HashMap<(Vec<char>, usize, char), usize>,
    s: &Vec<char>,
    depth: usize,
    highest: bool,
    cursors: &mut Vec<char>,
) -> usize {
    let cache_key = (s.clone(), depth, cursors[depth]);
    if let Some(cached) = cache.get(&cache_key) {
        return *cached;
    }

    let mut result = 0;
    for &c in s {
        let paths =
            find_shortest_paths(highest, cursors[depth], c);
        if depth == 0 {
            result += paths.into_iter().map(|l| l.len()).min().unwrap();
        } else {
            result += paths
                .into_iter()
                .map(|p| {
                    find_shortest_sequence(cache, &p, depth - 1, false, cursors)
                })
                .min()
                .unwrap();
        }
        cursors[depth] = c;
    }

    cache.insert(cache_key, result);

    result
}

fn solve(lines: &[String], depth: usize) -> usize {
    let mut cache = HashMap::new();
    lines.iter()
        .map(|code| {
            let mut cursors = vec!['A'; depth + 1];
            let vcode: Vec<char> = code.chars().collect();
            find_shortest_sequence(&mut cache, &vcode, depth, true, &mut cursors) * code[0..3].parse::<usize>().unwrap()
        })
        .sum()
}

fn main() {
    let lines = read_input("input.txt");
    let result_step1 = solve(&lines, 2);
    let result_step2 = solve(&lines, 25);

    println!("#1: {}", result_step1);
    println!("#2: {}", result_step2);
}
