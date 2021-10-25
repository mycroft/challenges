use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Point {
    position: (i32, i32),
    velocity: (i32, i32),
}

fn draw(points: &Vec<Point>, min_x: i32, max_x: i32, min_y: i32, max_y: i32) {
    for y in min_y..=max_y {
        let mut line = String::from("");
        for x in min_x..=max_x {
            if points.iter().any(|p| p.position.0 == x && p.position.1 == y) {
                line.push('#');
            } else {
                line.push(' ');
            }
        }

        println!("{}", line);
    }

    println!("");
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut points : Vec<Point> = vec![];

    let re = Regex::new(r"^position=< *([^ ]*), *([^ ]*)> velocity=< *([^ ]*), *([^ ]*)>$").unwrap();

    for line in lines {
        let caps = re.captures(line).unwrap();
        let position = (
            caps[1].parse::<i32>().unwrap(),
            caps[2].parse::<i32>().unwrap(),
        );

        let velocity = (
            caps[3].parse::<i32>().unwrap(),
            caps[4].parse::<i32>().unwrap(),
        );

        points.push(Point{position, velocity});
    }

    let mut cycle = 0;

    loop {
        let min_x = points.iter().map(|p| p.position.0).min().unwrap();
        let max_x = points.iter().map(|p| p.position.0).max().unwrap();
        let min_y = points.iter().map(|p| p.position.1).min().unwrap();
        let max_y = points.iter().map(|p| p.position.1).max().unwrap();
    
        if max_y - min_y < 10 {
            println!("#2 Cycle: {}", cycle);
            println!("#1:");
            draw(&points, min_x, max_x, min_y, max_y);
            break;
        }

        for p in &mut points {
            p.position.0 += p.velocity.0;
            p.position.1 += p.velocity.1;
        }

        cycle += 1;
    }
}
