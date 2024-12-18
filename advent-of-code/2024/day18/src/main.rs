use std::{fmt,fs};
use pathfinding::prelude::astar;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct Coord {
    x: isize,
    y: isize
}

impl Coord {
    fn successors(&self, grid: &Grid) -> Vec<(Coord, isize)> {
        let mut result = Vec::new();

        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

        for dir in dirs {
            let x = self.x + dir.0;
            let y = self.y + dir.1;

            if x < 0 || x >= grid.size || y < 0 || y >= grid.size {
                continue;
            }

            let coord = Coord { x, y };

            if !grid.failing[0..grid.threshold].contains(&coord) {
                result.push((coord, 1));
            }
        }

        result
    }

    fn manhattan_distance(&self, other: &Coord) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[derive(Debug, Clone)]
struct Grid {
    failing: Vec<Coord>,
    threshold: usize,
    size: isize,
}

fn read_input(fp: &str, threshold: usize, size: isize) -> Grid {
    let contents = fs::read_to_string(fp).expect("Error reading the file");

    let mut failing = Vec::new();

    for line in contents.lines() {
        let parts = line.split(",").collect::<Vec<&str>>();
        let x = parts[0].parse::<isize>().unwrap();
        let y = parts[1].parse::<isize>().unwrap();

        failing.push(Coord { x, y });
    }

    Grid { failing, threshold, size }
}

fn solve(grid: &Grid, start: Coord, end: Coord) -> Option<Vec<Coord>> {
    let result = astar(
        &start,
        |p| p.successors(grid),
        |p| p.manhattan_distance(&end),
        |p| *p == end
    );

    result.map(|(path, _cost)| path)
}

fn solve_step1(grid: &Grid) -> usize {
    let path = solve(grid, Coord { x: 0, y: 0 }, Coord { x: grid.size - 1, y: grid.size - 1 });

    path.unwrap().len() - 1 // number of steps is number of coords - 1
}

fn solve_step2(grid: &Grid) -> String {
    let mut b1 = grid.threshold;
    let mut b2 = grid.failing.len();

    loop {
        if b1 == b2 {
            return grid.failing[b1 - 1].to_string();
        }

        let mut ngrid = grid.clone();
        ngrid.threshold = (b1 + b2) / 2;
        let path = solve(&ngrid, Coord { x: 0, y: 0 }, Coord { x: ngrid.size - 1, y: ngrid.size - 1 });
        if path.is_some() {
            b1 = ngrid.threshold + 1;
        } else {
            b2 = ngrid.threshold;
        }
    }
}

fn main() {
    let grid = read_input("input.txt", 1024, 71);
    println!("#1 {:?}", solve_step1(&grid));
    println!("#2 {}", solve_step2(&grid));
}

#[test]
fn sample() {
    let grid = read_input("input_test.txt", 12, 7);
    let path = solve(&grid, Coord { x: 0, y: 0 }, Coord { x: grid.size - 1, y: grid.size - 1 });
    assert_eq!(22, path.unwrap().len() - 1);

    let result_step2 = solve_step2(&grid);
    assert_eq!("6,1".to_string(), result_step2);
}