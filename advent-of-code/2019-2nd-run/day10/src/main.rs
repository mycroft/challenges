use std::{fs::read_to_string, cmp::Ordering};
use float_eq::float_eq;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize
}

fn parse(fp: &str) -> Vec<Point> {
    let mut result = Vec::new();

    let contents = read_to_string(fp).expect("file to open");
    let lines = contents.lines().collect::<Vec<&str>>();

    for (y, line) in lines.into_iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                result.push(
                    Point { x: x as isize, y: y as isize}
                );
            }
        }
    }

    result
}

impl Point {
    fn can_view(&self, m: &[Point], other: &Point) -> bool {
        if self == other {
            return false;
        }
        for p in m.iter() {
            if p.is_between(self, other) {
                return false;
            }
        }

        true
    }

    fn is_between(&self, p0: &Point, p1: &Point) -> bool {
        let segment_distance = p0.distance_with(p1);
        let p0_distance = self.distance_with(p0);
        let p1_distance = self.distance_with(p1);

        segment_distance > p0_distance && segment_distance > p1_distance 
            && float_eq!(segment_distance, p0_distance + p1_distance, abs <= 0.000_1)
    }

    fn distance_with(&self, other: &Point) -> f64 {
        ((other.x - self.x).pow(2) as f64 + (other.y - self.y).pow(2) as f64).sqrt()
    }

    fn get_viewable(&self, m: &[Point]) -> Vec<Point> {
        m.iter().filter(|&p1| self.can_view(m, p1)).cloned().collect()
    }

    fn get_angle(&self, origin: &Point) -> f64 {
        let base_origin = Point{ x: origin.x, y: 0 };

        let ab = Point{ x: origin.x - base_origin.x, y: origin.y - base_origin.y };
        let cb = Point{ x: origin.x - self.x, y: origin.y - self.y };

        let dot = (ab.x * cb.x + ab.y * cb.y) as f64;
        let cross = (ab.x * cb.y - ab.y * cb.x) as f64;

        let result = cross.atan2(dot) * 180f64 / std::f64::consts::PI;
        if result >= 0f64 {
            result
        } else {
            180f64 + (180f64 - result.abs())
        }
    }
}

fn find_best_asteroid(m: &[Point]) -> (Point, usize) {
    let mut result = m[0];
    let mut max_viewable = 0;

    for p in m.iter() {
        let viewable = p.get_viewable(m).len();

        if viewable > max_viewable {
            result = *p;
            max_viewable = viewable;
        }
    }

    (result, max_viewable)
}

fn get_crushed_asteroids(m: &[Point]) -> Vec<Point> {
    let mut result = Vec::new();
    let mut m = m.to_owned();

    // find asteroid that will be our base
    let base = find_best_asteroid(&m).0;

    while m.len() > 1 {
        // find crushable asteroids
        let mut asteroids = base.get_viewable(&m);

        // sort the asteroids per angle
        asteroids.sort_by(|a, b| {
            if a == b {
                Ordering::Equal
            } else if a.get_angle(&base) > b.get_angle(&base) {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });

        // fill asteroids in the result vector, and remove them from the whole list
        asteroids.iter().map(|x| {
            result.push(*x);
            m.remove(
                m.iter().position(|y| y == x).unwrap()
            );
        }).count();
    }

    result
}

fn get_crushed_asteroids_store(m: &[Point]) -> isize {
    let result = get_crushed_asteroids(m);
    result[199].x * 100 + result[199].y
}

fn main() {
    let m = parse("input.txt");
    let result_step1 = find_best_asteroid(&m);
    println!("#1 {}", result_step1.1); //276

    let result_step2 = get_crushed_asteroids_store(&m);
    println!("#2 {}", result_step2); // 1321
}

#[test]
fn test_distance() {
    assert_eq!(
        1.0,
        Point{x: 0, y: 0}.distance_with(&Point{x:1, y: 0})
    );
}

#[test]
fn test_is_between() {
    assert!(Point{x: 2, y: 2}.is_between(&Point{x: 1, y: 1}, &Point{x: 4, y: 4}));
    assert!(!Point{x: 1, y: 2}.is_between(&Point{x: 1, y: 1}, &Point{x: 4, y: 4}));
}

#[test]
fn test_sample_0() {
    let m = parse("input.txt_test0");
    assert_eq!(
        (Point{x: 3, y: 4}, 8),
        find_best_asteroid(&m)
    );
}

#[test]
fn test_sample_1() {
    let m = parse("input.txt_test1");
    assert_eq!(
        (Point{x: 5, y: 8}, 33),
        find_best_asteroid(&m)
    );
}

#[test]
fn test_sample_2() {
    let m = parse("input.txt_test2");
    assert_eq!(
        (Point{x: 1, y: 2}, 35),
        find_best_asteroid(&m)
    );
}

#[test]
fn test_sample_3() {
    let m = parse("input.txt_test3");
    assert_eq!(
        (Point{x: 6, y: 3}, 41),
        find_best_asteroid(&m)
    );
}

#[test]
fn test_sample_4() {
    let m = parse("input.txt_test4");
    assert_eq!(
        (Point{x: 11, y: 13}, 210),
        find_best_asteroid(&m)
    );
}

#[test]
fn test_angle() {
    assert_eq!(
        0.,
        Point{ x : 10, y : 1}.get_angle(&Point{ x : 10, y : 10})
    );

    assert_eq!(
        90.,
        Point{ x : 10, y : 10}.get_angle(&Point{ x : 5, y : 10})
    );

    assert_eq!(
        270.,
        Point{ x : 0, y : 10}.get_angle(&Point{ x : 5, y : 10})
    );
}

#[test]
fn test_sample_step2() {
    let m = parse("input.txt_test4");
    assert_eq!(
        802,
        get_crushed_asteroids_store(&m)
    );
}