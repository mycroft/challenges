use std::fs::read_to_string;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn nb_adj(rolls: &HashSet<Point>, p: &Point) -> usize {
    let mut count = 0;

    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            let neighbor = Point {
                x: p.x + dx,
                y: p.y + dy,
            };

            if rolls.contains(&neighbor) {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let contents = read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut rolls = HashSet::new();

    for (i, line) in contents.lines().enumerate() {
        for (j, ch) in line.chars().enumerate() {

            if ch == '.' {
                continue;
            }

            rolls.insert(Point {
                x: i as i32,
                y: j as i32,
            });
        }
    }

    let mut step1 = 0;
    let mut step2 = 0;

    loop {
        let mut removable = HashSet::new();
        for p in &rolls {
            let adjacent_count = nb_adj(&rolls, p);
            if adjacent_count < 4 {
                removable.insert(*p);
            }
        }

        if removable.is_empty() {
            break;
        }

        if step1 == 0 {
            step1 = removable.len();
        }
        step2 += removable.len();

        for p in &removable {
            rolls.remove(p);
        }
    }

    println!("#1: {}", step1);
    println!("#2: {}", step2);
}
