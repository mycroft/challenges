use std::fs;
use regex::Regex;

struct Square {
    id: usize,
    x: usize,
    y: usize,
    size_x: usize,
    size_y: usize,
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();

    let mut squares : Vec<Square> = vec![];

    let size = 1000;

    let mut matrix : Vec<usize> = vec![0; size*size];

    for line in contents.lines() {
        let caps = re.captures(line).unwrap();

        squares.push(Square {
            id: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            x: caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
            y: caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
            size_x: caps.get(4).unwrap().as_str().parse::<usize>().unwrap(),
            size_y: caps.get(5).unwrap().as_str().parse::<usize>().unwrap(),
        });
    }

    for square in &squares {
        for i in square.x..square.size_x+square.x {
            for j in square.y..square.size_y+square.y {
                matrix[i + size * j] += 1;
            }
        }
    }

    println!("Part #1: {}", matrix.iter().filter(|x| **x > 1).count());


    for square in &squares {
        let mut found = true;

        for i in square.x..square.size_x+square.x {
            for j in square.y..square.size_y+square.y {
                if matrix[i + size * j] != 1 {
                    found = false;
                    break;
                }
            }
            if found == false {
                break;
            }
        }

        if found == true {
            println!("Part #2: {}", square.id);
            break;
        }
    }
}
