use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug)]
struct Point {
    id: usize,
    x: usize,
    y: usize,
}

fn distance(p: &Point, z: &Point) -> usize {
    ((p.x as i16 - z.x as i16).abs() + (p.y as i16 - z.y as i16).abs()) as usize
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut points : Vec<Point> = vec![];

    for (id, line) in contents.lines().enumerate() {
        let res = line.split(", ").collect::<Vec<&str>>();
        let x = res[0].parse::<usize>().unwrap();
        let y = res[1].parse::<usize>().unwrap();

        points.push(Point{ id: id, x: x, y: y });
    }

    let max_x = points.iter().map(|x| x.x).max().unwrap();
    let max_y = points.iter().map(|x| x.y).max().unwrap();

    let mut grid = vec![vec![-1; max_x]; max_y];

    let mut finite_region_10000 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            let mut min_distance = max_x + max_y;
            let mut total_distance = 0;

            for p in &points {
                let current_distance = distance(&Point{id: 42, x: i, y: j}, p);
                total_distance += current_distance;
                if current_distance < min_distance {
                    min_distance = current_distance;
                    grid[i][j] = p.id as i8;
                }
            }

            if total_distance < 10000 {
                finite_region_10000 += 1;
            }
        }
    }

    let mut hs = HashSet::new();
    let mut hm = HashMap::new();

    grid[0].iter().map(|x| if !hs.contains(x) { hs.insert(x); }).count();
    grid[grid.len()-1].iter().map(|x| if !hs.contains(x) { hs.insert(x); }).count();
    grid.iter().map(|x| &x[0]).map(|x| if !hs.contains(x) { hs.insert(x); }).count();
    grid.iter().map(|x| &x[x.len()-1]).map(|x| if !hs.contains(x) { hs.insert(x); }).count();

    grid.iter().map(
        |x|
        x.iter().map(
            |n|
            *hm.entry(n).or_insert(0)+=1
        ).count()
    ).count();

    let mut max_size = 0;

    for (id, m) in hm {
        if hs.contains(id) {
            continue;
        }

        if m > max_size {
            max_size = m;
        }
    }

    println!("Part #1: {}", max_size);
    println!("Part #2: {}", finite_region_10000);
}
