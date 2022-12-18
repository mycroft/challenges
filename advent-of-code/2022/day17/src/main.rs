use std::fs::read_to_string;
use std::collections::{HashMap,HashSet};
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Shape {
    points: HashSet<Point>,
    x_size: isize,
    y_size: isize,
}

fn parse(fp: &str) -> Vec<isize> {
    let contents = read_to_string(fp).unwrap();

    contents.chars()
        .filter(|x| *x == '<' || *x == '>')
        .map(|x| match x {
            '>' => 1 as isize,
            '<' => -1 as isize,
            _ => unreachable!()
        })
        .collect()
}

fn shapes() -> Vec<Shape> {
    let mut shapes = Vec::new();

    shapes.push(
        Shape{points: HashSet::from_iter([Point{x: 0, y: 0}, Point{x: 1, y: 0}, Point{x: 2, y: 0}, Point{x: 3, y: 0},].iter().cloned()), x_size: 4, y_size: 1}
    );

    shapes.push(
        Shape{points: HashSet::from_iter([Point{x: 1, y: 0}, Point{x: 0, y: 1}, Point{x: 1, y: 1}, Point{x: 2, y: 1}, Point{x: 1, y: 2},].iter().cloned()), x_size: 3, y_size: 3}
    );

    shapes.push(
        Shape{points: HashSet::from_iter([Point{x: 0, y: 0}, Point{x: 1, y: 0}, Point{x: 2, y: 0}, Point{x: 2, y: 1}, Point{x: 2, y: 2},].iter().cloned()), x_size: 3, y_size: 3}
    );

    shapes.push(
        Shape{points: HashSet::from_iter([Point{x: 0, y: 0}, Point{x: 0, y: 1}, Point{x: 0, y: 2}, Point{x: 0, y: 3},].iter().cloned()), x_size: 1, y_size: 4}
    );

    shapes.push(
        Shape{points: HashSet::from_iter([Point{x: 0, y: 0}, Point{x: 1, y: 0}, Point{x: 0, y: 1}, Point{x: 1, y: 1},].iter().cloned()), x_size: 2, y_size: 2}
    );

    shapes
}

impl Shape {
    fn can_move(&self, occupied: &HashSet<Point>, direction: (isize, isize)) -> Option<HashSet<Point>> {
        // build new shape
        let mut new_points = HashSet::new();

        for p in &self.points {
            if p.x + direction.0 < 1 || p.x + direction.0 > 7 || p.y + direction.1 < 1 {
                return None
            }
            new_points.insert(
                Point{x: p.x + direction.0, y: p.y + direction.1}
            );
        }

        for p in &new_points {
            if occupied.contains(&p) {
                return None
            }
        }

        Some(new_points)        
    }
}

fn display(occupied: &HashSet<Point>, shape: &Shape, highest_position: isize) {
    for idx in 0..highest_position + shape.y_size + 3{
        let y = highest_position - idx + (shape.y_size + 3);

        print!("|");
        for x in 1..=7 {
            if occupied.contains(&Point{x, y}) {
                print!("#")
            } else if shape.points.contains(&Point{x, y}) {
                print!("@")
            } else {
                print!(".")
            }
        }
        print!("|");
        println!()
    }

    println!("+-------+")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    shape_idx: usize,
    mov_idx: usize,
    heights: [isize; 7],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct StateValue {
    idx: isize,
    highestpoint: isize,
}

fn get_heights_profile(max_height: isize, profiles: [isize; 7]) -> [isize; 7] {
    let mut result = [0isize; 7];

    for p in profiles.iter().enumerate() {
        result[p.0] = max_height - p.1;
    }

    result
}

fn play_step1(fp: &str, runs: isize) -> isize {
    let directions = parse(fp);
    let mut current_direction_idx = 0;
    let mut highestpoint = 0;
    let mut current_heights = [0isize; 7];

    let mut occupied_positions = HashSet::<Point>::new();

    let shapes = shapes();

    let mut known_states = HashMap::<State, StateValue>::new();

    for idx in 0..runs {
        let mut shape = shapes[idx as usize % 5].clone();

        // println!();
        // println!("New shape!");

        let mut shape_points = HashSet::new();
        for p in shape.points {
            shape_points.insert(
                Point{x: p.x + 3, y: p.y + 3 + highestpoint + 1}
            );
        }
        shape.points = shape_points;

        // new shape; lets check the cache


        loop {
            // println!("new loop");
            // display(&occupied_positions, &shape, highestpoint);

            let current_direction = directions[current_direction_idx%directions.len()];

            // let profile = get_heights_profile(highestpoint, current_heights);
            // println!("{profile:?}");
            // heights: get_heights_profile(highestpoint, current_heights)

            if let Some(points) = shape.can_move(&occupied_positions, (current_direction, 0)) {
                shape.points = points;
            } else {
                // ignoring movement
            }

            current_direction_idx += 1;

            if let Some(points) = shape.can_move(&occupied_positions, (0, -1)) {
                shape.points = points;
                continue;
            }

            let last_highestpoint = highestpoint;

            // can not move forward.
            for p in &shape.points {
                if p.y > highestpoint {
                    highestpoint = p.y;
                }
                occupied_positions.insert(*p);

                if current_heights[(p.x - 1) as usize] < p.y {
                    current_heights[(p.x - 1) as usize] = p.y;
                }
            }
            // println!("{:?} new highest point: {highestpoint}", shape);
            // display(&occupied_positions, &shape, highestpoint);

            // put this state in cache.
            let new_state = State {
                shape_idx: idx as usize % 5,
                mov_idx: current_direction_idx % directions.len(),
                heights: get_heights_profile(highestpoint, current_heights),
            };

            let new_value = StateValue {
                idx: idx,
                highestpoint,
            };

            if known_states.contains_key(&new_state) {
                let val = known_states.get(&new_state).unwrap();
                let period = idx - val.idx;
                // for testing sample, period is 35
                // for my play, it is 1695

                // compute how many runs we have still to do
                if (runs - idx) % period == 0 {
                    // println!("Hey, I know this state (happening at idx:{idx}) value was: {} ~ period is {}", val.idx, period);
                    // println!("Current height is {}; it grown by {} this cycle.", highestpoint, highestpoint - last_highestpoint);
                    // println!("Each cycle, it is growing {}", highestpoint - val.highestpoint);
    
                    let cycles = (runs - idx) / period;
                    let period_cycle_grow = highestpoint - val.highestpoint;

                    // println!("Value should be: {}", cycles * period_cycle_grow + highestpoint);

                    return cycles * period_cycle_grow + last_highestpoint;
                }
            } else if idx > 50 {
                known_states.insert(new_state, new_value);
            }

            break;
        }
    }

    highestpoint
}

fn main() {
    println!("#1 {}", play_step1("input.txt", 2022)); // 3215
    println!("#2 {}", play_step1("input.txt", 1000000000000)); // 1575811209487
}

#[test]
fn test_sample() {
    let res = play_step1("input.txt_test", 2022);
    assert_eq!(
        3068,
        res
    );

    let res = play_step1("input.txt_test", 1000000000000);
    assert_eq!(
        1514285714288,
        res
    );

}