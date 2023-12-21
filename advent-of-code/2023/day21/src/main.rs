use std::fs;
use std::collections::{HashMap, VecDeque, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Element {
    Ground,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

fn read_map(fp: &str) -> (HashMap<Position, Element>, Position) {
    let mut result = HashMap::new();
    let mut starting_position: Option<Position> = None;

    let contents = fs::read_to_string(fp).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let element = match c {
                '#' => {
                    Element::Wall
                },
                '.' | 'S' => {
                    if c == 'S' {
                        starting_position = Some(Position { x: x as isize, y: y as isize });
                    }
                    Element::Ground
                },
                _ => unreachable!(),
            };

            result.insert(Position{x: x as isize, y: y as isize}, element);
        }
    }

    (result, starting_position.unwrap())
}

fn solve(map: &HashMap<Position, Element>, starting_position: Position, steps: usize, is_infinite: bool) -> usize {
    let dirs = [(0isize, -1isize), (0, 1), (1, 0), (-1, 0)];

    let mut to_visit: VecDeque<(Position, usize)> = VecDeque::new();
    to_visit.push_back((starting_position, steps));

    let mut previous_set = HashSet::new();
    previous_set.insert(starting_position);

    for n in 1..steps+1 {
        let mut new_set: HashSet<Position> = HashSet::new();

        for current_position in previous_set {
            for dir in dirs {
                let mut new_position = current_position;
                new_position.x += dir.0;
                new_position.y += dir.1;

                let mut map_position = new_position;

                if is_infinite {
                    while map_position.x < 0 { map_position.x += 131; }
                    while map_position.y < 0 { map_position.y += 131; }
                    while map_position.x > 130 { map_position.x -= 131; }
                    while map_position.y > 130 { map_position.y -= 131; }    
                }

                if !map.contains_key(&map_position) {
                    continue;
                }
    
                if *map.get(&map_position).unwrap() == Element::Wall || new_set.contains(&new_position) {
                    continue;
                }

                new_set.insert(new_position);
            }
        }

        if n > 64 && (n - 65) % 131 == 0 {
             println!("{} {}", n, new_set.len());
        }

        previous_set = new_set;
    }

    previous_set.len()
}

fn main() {
    let (map, starting_position) = read_map("input.txt");
    let p1 = solve(&map, starting_position, 64, false);
    println!("#1: {}", p1); // 3542

    println!("See comments in code");
    solve(&map, starting_position, 65 + 131*2, true);

    let steps = 26501365;
    let x: u128 = (steps - 65) / 131;

    let p2 = 14494 * x*x + 14677 * x + 3725;
    println!("#2: {}", p2);
}

/*
It's a quadratic where you have values for x = 0, x = 1, and x =2 (which are equal to f(65), f(65 + 131), f(65 + 131 * 2)).

65 3725
196 32896
327 91055
#2: 593174154384306

https://www.wolframalpha.com/input?i=quadratic+fit+calculator&assumption=%7B%22F%22%2C+%22QuadraticFitCalculator%22%2C+%22data3x%22%7D+-%3E%22%7B0%2C+1%2C+2%7D%22&assumption=%7B%22F%22%2C+%22QuadraticFitCalculator%22%2C+%22data3y%22%7D+-%3E%22%7B3725%2C+32896%2C+91055%7D%22

14494 x^2 + 14677 x + 3725
(data is perfectly fit by a 2nd degree polynomial)
*/