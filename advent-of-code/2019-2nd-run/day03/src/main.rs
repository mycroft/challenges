use std::collections::{HashSet, HashMap};
use std::fs::read_to_string;
use std::hash::Hash;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn manhattan_from_zero(&self) -> isize {
        (self.x.abs() + self.y.abs()) as isize
    }
}

fn parse_string(s: &str) -> (HashSet<Pos>, HashMap<Pos, usize>) {
    let mut current = Pos{x: 0, y: 0};
    let mut hs = HashSet::new();
    let mut hm = HashMap::new();
    let mut total_steps = 0;

    for dir in s.split(",") {
        let direction = match dir.chars().nth(0).unwrap() {
            'L' => Pos{x: -1, y: 0},
            'R' => Pos{x: 1, y: 0},
            'U' => Pos{x: 0, y: -1},
            'D' => Pos{x: 0, y: 1},
            _ => todo!()
        };

        let steps = &dir[1..].parse::<u32>().unwrap();
        for _ in 0..*steps {
            current.x += direction.x;
            current.y += direction.y;

            hs.insert(current);

            total_steps += 1;

            hm.insert(current, total_steps);
        }

        // println!("{direction:?} {steps} {hs:?}");
    }

    (hs, hm)
}

fn intersections(s0: &HashSet<Pos>, s1: &HashSet<Pos>) -> HashSet<Pos> {
    HashSet::from_iter(s0.intersection(&s1).map(|x| *x))
}

fn closest(hs: &HashSet<Pos>) -> isize {
    let mut result: Option<isize> = None;

    for p in hs {
        let m = p.manhattan_from_zero();
        if result == None || result.unwrap() > m {
            result = Some(m)
        }
    }

    result.unwrap()
}

fn step1(s0: &str, s1: &str) -> isize {
    let s0 = parse_string(s0);
    let s1 = parse_string(s1);

    closest(
        &intersections(&s0.0, &s1.0)
    )
}

fn step2(s0: &str, s1: &str) -> usize {
    let s0 = parse_string(s0);
    let s1 = parse_string(s1);

    let intersections = intersections(&s0.0, &s1.0);
    let mut min_score: Option<usize> = None;

    for intersection in intersections {
        let score = s0.1.get(&intersection).unwrap() + s1.1.get(&intersection).unwrap();

        if min_score == None || min_score.unwrap() > score {
            min_score = Some(score);
        }
    }

    min_score.unwrap()
}

fn main() {
    let contents = read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    println!("#1 {}", step1(lines[0], lines[1])); // 1064
    println!("#2 {}", step2(lines[0], lines[1])); // 25676
}

#[test]
fn test_sample() {
    // first step
    assert_eq!(
        6,
        step1("R8,U5,L5,D3", "U7,R6,D4,L4")
    );

    assert_eq!(
        159,
        step1("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83")
    );

    assert_eq!(
        135,
        step1("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
    );
}

#[test]
fn test_sample_step2() {
    // second step
    assert_eq!(
        30,
        step2("R8,U5,L5,D3", "U7,R6,D4,L4")
    );

    assert_eq!(
        610,
        step2("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83")
    );

    assert_eq!(
        410,
        step2("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")
    );    
}

#[test]
fn test_input() {
    let contents = read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    assert_eq!(
        1064,
        step1(lines[0], lines[1])
    );

    assert_eq!(
        25676,
        step2(lines[0], lines[1])
    );

}