use regex::Regex;
use std::fs;

#[derive(Debug,Clone,Copy)]
struct Point {
    x: i32,
    y: i32,
    speed_x: i32,
    speed_y: i32,
}

fn dimension(points: &Vec<Point>) -> (usize, usize) {
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();

    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();

    (
        (max_x - min_x) as usize,
        (max_y - min_y) as usize
    )
}

fn dump(points: &Vec<Point>) {
    let dims = dimension(&points);

    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();

    let mut v = vec![vec![false; dims.0+1]; dims.1+1];

    points.iter().map(|p| v[(p.y - min_y) as usize][(p.x - min_x) as usize] = true).count();

    for i in 0..v.len() {
        let mut s = String::from("");
        for j in 0..v[i].len() {
            if v[i][j] {
                s.push('#');
            } else {
                s.push(' ');
            }
        }

        println!("{}", s);
    }
}

fn step(points: &Vec<Point>) -> (Vec<Point>, usize, usize) {
    let mut points = points.clone();
    for p in points.iter_mut() {
        p.x += p.speed_x;
        p.y += p.speed_y;
    }

    let dim = dimension(&points);

    (points, dim.0, dim.1)
}

fn main() {
    let re = Regex::new(r"^position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>").unwrap();
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut points = vec![];

    for line in contents.lines() {
        let captures = re.captures(line).unwrap();

        points.push(Point{
            x: captures.get(1).unwrap().as_str().parse::<i32>().unwrap(),
            y: captures.get(2).unwrap().as_str().parse::<i32>().unwrap(),
            speed_x: captures.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            speed_y: captures.get(4).unwrap().as_str().parse::<i32>().unwrap(),
        });
    }

    let mut old_dim_x = 0;
    let mut old_dim_y = 0;

    let mut loop_n = 0;

    loop {
        let new_points = step(&points);

        if old_dim_x != 0 && old_dim_x < new_points.1 && old_dim_y < new_points.2 {
            dump(&points);
            println!("Loops: {}", loop_n);
            break;
        }

        loop_n += 1;

        old_dim_x = new_points.1;
        old_dim_y = new_points.2;

        points = new_points.0;
    }
}
