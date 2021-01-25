use std::fs;
use std::collections::HashMap;

#[derive(Debug,Copy,Clone,Eq,Hash,PartialEq)]
struct Pos {
    x: usize,
    y: usize,
}

fn next(segments: &Vec<(Pos, Pos)>, last: (Pos, Pos), current: Pos) -> Option<((Pos, Pos), Pos)> {
    for segment in segments {
        if *segment == last {
            continue;
        }

        if current == segment.0 {
            // println!("Matched: {:?}", segment);
            return Some((*segment, segment.1));
        } else if current == segment.1 {
            // println!("Matched: {:?}", segment);
            return Some((*segment, segment.0));
        }
    }

    None
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut grid : Vec<Vec<char>> = vec![];
    let mut out : String = String::from("");

    let lines = input.lines();

    for line in lines {
        let s : String = line.to_string();
        grid.push(s.chars().collect::<Vec<char>>());
    }

    let mut segments : Vec<(Pos, Pos)> = vec![];
    let mut letters : HashMap<Pos, char> = HashMap::new();

    for y in 0..grid.len() {
        let row_len = grid[y].len();
        let mut started = false;
        let mut starting_pos : Pos = Pos { x: 0, y: 0 };

        for x in 0..row_len {
            let c = grid[y][x];
            if c == '+' && !started {
                starting_pos = Pos { x: x, y: y };
                started = true;

            } else if c == '+' && started {
                segments.push(
                    (starting_pos, Pos { x: x, y: y })
                );
                started = false;

            }

            if c != '-' && c != '+' && c != ' ' && c != '|' {
                letters.insert(Pos{ x: x, y: y }, c);
            }
        }
    }

    let col_len = grid.len();
    let row_len = grid[0].len();

    for x in 0..row_len {
        let mut starting_pos : Pos = Pos { x: 0, y: 0 };
        let mut started = false;

        for y in 0..col_len {
            let c = grid[y][x];
            if (c == '+' || c == 'Y' || (c == '|' && y == 0)) && !started {
                starting_pos = Pos { x: x, y: y };
                started = true;

            } else if (c == '+' || c == 'Y' || (c == '|' && y == 0)) && started {
                segments.push(
                    (starting_pos, Pos { x: x, y: y })
                );
                started = false;
            }

            if c != '-' && c != '+' && c != ' ' && c != '|' {
                letters.insert(Pos{ x: x, y: y }, c);
            }
        }
    }

    let mut current = Pos { x: 163, y: 0 };
    let mut last_segment = (current, current);

    let mut steps = 1;

    loop {
        let last_pos = current;
        let res = next(&segments, last_segment, current);
        if res == None {
            break;
        }

        let res = res.unwrap();
        last_segment = res.0;
        current = res.1;

        // println!("Post: {:?} {:?}",
        //     current,
        //     (current.x as i32 - last_pos.x as i32).abs() + (current.y as i32 - last_pos.y as i32).abs()
        // );
        steps += (current.x as i32 - last_pos.x as i32).abs() + (current.y as i32 - last_pos.y as i32).abs();

        if let Some(c) = has_letter(&letters, last_pos, current) {
            out.push(c);
        }
    }

    println!("Part #1: {}", out);
    println!("Part #2: {}", steps);
}

fn between(a: usize, b: usize, l: usize) -> bool {
    (l >= a && b >= l) || (l <= a && b <= l)
}

fn has_letter(letters: &HashMap<Pos, char>, last: Pos, current: Pos) -> Option<char> {
    for (k, letter) in letters {
        if last.x == current.x && k.x == current.x && between(last.y, current.y, k.y) {
            return Some(*letter)
        }

        if last.y == current.y && k.y == current.y && between(last.x, current.x, k.x) {
            return Some(*letter)
        }
    }

    None
}