use std::collections::HashSet;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize
}

fn get_point(s: &str) -> Point {
    let s = s.split(",").collect::<Vec<&str>>().iter().map(|x| x.parse::<usize>().expect("number")).collect::<Vec<usize>>();

    Point{x: s[0], y: s[1]}
}

fn get_all_points(a: Point, b: Point, diag: bool) -> Vec<Point> {
    let x_way = match b.x as i32 - a.x as i32 {
        n if n == 0 => 0,
        n if n < 0 => -1,
        n if n > 0 => 1,
        _ => unimplemented!()
    };
    let y_way = match b.y as i32 - a.y as i32 {
        n if n == 0 => 0,
        n if n < 0 => -1,
        n if n > 0 => 1,
        _ => unimplemented!()
    };

    let mut a = a;
    let mut res = vec![];

    if !diag && (a.x != b.x && a.y != b.y) {
        return vec![];
    }

    res.push(a);

    while a != b {
        a.x = (a.x as i32 + x_way) as usize;
        a.y = (a.y as i32 + y_way) as usize;

        res.push(a);
    }

    res
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut all_points: HashSet<Point> = HashSet::new();
    let mut bad_points: HashSet<Point> = HashSet::new();

    let mut all_points2: HashSet<Point> = HashSet::new();
    let mut bad_points2: HashSet<Point> = HashSet::new();

    for line in lines {
        let points = line.split(" -> ").collect::<Vec<&str>>();

        let a = get_point(points[0]);
        let b = get_point(points[1]);
        
        let line = get_all_points(a, b, false);

        for point in line.iter() {
            if all_points.contains(point) {
                bad_points.insert(*point);
            } else {
                all_points.insert(*point);
            }
        }

        let line = get_all_points(a, b, true);

        for point in line.iter() {
            if all_points2.contains(point) {
                bad_points2.insert(*point);
            } else {
                all_points2.insert(*point);
            }
        }
    }

    println!("#1: {}", bad_points.len());
    println!("#2: {}", bad_points2.len());
}
