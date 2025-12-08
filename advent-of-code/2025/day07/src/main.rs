use std::fs::read_to_string;

use std::collections::{HashSet, HashMap};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

fn parse(fp: &str) -> (Point, HashSet<Point>, usize) {
    let contents = read_to_string(fp).expect("failed to open input file");

    let mut starting_point = None;
    let mut splitters = HashSet::new();
    let lines_num = contents.lines().count();

    for (idx_line, line) in contents.lines().enumerate() {
        for (idx_char, char) in line.chars().enumerate() {
            match char {
                'S' => {
                    starting_point = Some(Point{x: idx_char as isize, y: idx_line as isize});
                },
                '^' => {
                    splitters.insert(
                        Point{x: idx_char as isize, y: idx_line as isize}
                    );
                },
                _ => {},
            }
        }
    }

    (starting_point.unwrap(), splitters, lines_num)
}

fn solve(start_point: Point, splitters: &HashSet<Point>, lines_num: usize) -> (usize, usize) {
    let mut split = 0;
    let mut beams: HashMap<Point, usize> = HashMap::new();
    let mut current_line = 0;

    let first_beam = Point {
        x: start_point.x,
        y: start_point.y + 1,
    };

    beams.insert(first_beam, 1);

    while current_line < lines_num {
        let to_play = beams.clone();
        for (beam, _) in to_play {
            if beam.y != current_line as isize {
                continue;
            }
            let current_timelines = *beams.get(&beam).unwrap();

            let next = Point {
                x: beam.x,
                y: beam.y + 1,
            };

            if splitters.contains(&next) {
                split += 1;

                *beams.entry(Point {x: beam.x - 1, y: beam.y + 1}).or_insert(0) += current_timelines;
                *beams.entry(Point {x: beam.x + 1, y: beam.y + 1}).or_insert(0) += current_timelines;

            } else {
                *beams.entry(next).or_insert(0) += current_timelines;
            }
        }

        current_line += 1;
    }

    (split, beams.iter().filter(|(b, _)| 1+b.y == lines_num as isize).map(|b| *b.1).sum())
}

fn main() {
    let (start_point, splitters, lines_num) = parse("input.txt");

    let (split_num, pathes) = solve(start_point, &splitters, lines_num);
    println!("#1: {}", split_num);
    println!("#2: {}", pathes);
}
