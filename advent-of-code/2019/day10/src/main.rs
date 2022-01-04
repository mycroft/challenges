use std::fs;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Point {
    x: isize,
    y: isize,
}

fn parse(fp: &str) -> Vec<Point> {
    let contents = fs::read_to_string(fp).expect("input file");
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut points = vec![];

    for (y, &line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            points.push(Point {
                x: x as isize,
                y: y as isize,
            })
        }
    }

    points
}

fn is_between(p0: &Point, p1: &Point, c: &Point) -> bool {
    let dist_max = distance(p0, p1);
    let dist_p0 = distance(p0, c);
    let dist_p1 = distance(p1, c);

    dist_max > dist_p0 && dist_max > dist_p1
        && ((c.y - p0.y) * (p1.x - p0.x) == (c.x - p0.x) * (p1.y - p0.y))
}

fn distance(p0: &Point, p1: &Point) -> f64 {
    f64::sqrt((p1.x - p0.x).pow(2) as f64 + (p1.y - p0.y).pow(2) as f64)
}

fn find_point_between(points: &[Point], p0: &Point, p1: &Point) -> bool {
    if p0 == p1 {
        return false;
    }

    for p in points {
        if p == p0 || p == p1 {
            continue;
        }

        if is_between(p0, p1, p) {
            return true;
        }
    }

    false
}

fn get_viewed_by(points: &[Point], p: &Point) -> Vec<Point> {
    let mut res = vec![];
    for point_candidate in 0..points.len() {
        let point_candidate = points[point_candidate];
        if point_candidate == *p {
            continue;
        }

        if !find_point_between(points, p, &point_candidate) {
            res.push(point_candidate);
        }
    }

    res
}

fn get_best_location(fp: &str) -> (usize, Point, Vec<Point>) {
    let points = parse(fp);

    let mut visibles: Vec<Point> = vec![];
    let mut candidate: Option<Point> = None;

    for point_idx in 0..points.len() {
        let p = points[point_idx];

        let viewed = get_viewed_by(&points, &p);

        if visibles.len() < viewed.len() {
            candidate = Some(p);
            visibles = viewed;
        }
    }

    (visibles.len(), candidate.expect("point"), points)
}

fn angle(orig: &Point, p: &Point) -> f64 {
    let refpoint = Point { x: orig.x, y: 0 };

    let ab = Point { x: orig.x - refpoint.x, y: orig.y - refpoint.y };
    let cb = Point { x: orig.x - p.x, y: orig.y - p.y };

    let dot = (ab.x * cb.x + ab.y * cb.y) as f64;
    let cross = (ab.x * cb.y - ab.y * cb.x) as f64;

    let alpha = f64::atan2(cross, dot);

    let res = alpha * 180f64 / std::f64::consts::PI;
    if res >= 0f64 {
        res
    } else {
        180f64 + (180f64 - res.abs())
    }
}

fn main() {
    let res = get_best_location("input.txt");
    println!("#1 {} (with {:?})", res.0, res.1);

    let points = res.2;
    let p = res.1;

    let mut targets = get_viewed_by(&points, &p);
    targets.sort_by(|a, b| angle(&p, a).partial_cmp(&angle(&p, b)).unwrap());

    println!("#2 {}", targets[199].x * 100 + targets[199].y);
}

#[test]
fn test0() {
    assert_eq!(10, parse("input.txt_test0").len());
}

#[test]
fn test_between() {
    assert!(is_between(&Point{x: 1, y: 1}, &Point{x: 3, y: 3}, &Point{x: 2, y: 2}));
    assert!(is_between(&Point{x: 0, y: 0}, &Point{x: 10000, y: 10000}, &Point{x: 2, y: 2}));
    assert!(is_between(&Point{x: 1, y: 0}, &Point{x: 3, y: 4}, &Point{x: 2, y: 2}));
}

#[test]
fn test_play() {
    let res = get_best_location("input.txt_test0");
    assert_eq!(8, res.0);

    let res = get_best_location("input.txt_test1");
    assert_eq!(33, res.0);

    let res = get_best_location("input.txt_test2");
    assert_eq!(35, res.0);

    let res = get_best_location("input.txt_test3");
    assert_eq!(41, res.0);

    let res = get_best_location("input.txt_test4");
    assert_eq!(210, res.0);
}