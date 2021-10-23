use std::fs;
use regex::Regex;

fn parse_line(re: &regex::Regex, line: &str) -> (usize, usize, usize, usize, usize) {
    let caps = re.captures(line).unwrap();

    let _id = caps[1].parse::<usize>().unwrap();
    let _x = caps[2].parse::<usize>().unwrap();
    let _y = caps[3].parse::<usize>().unwrap();
    let _width = caps[4].parse::<usize>().unwrap();
    let _height = caps[5].parse::<usize>().unwrap();

    (_id, _x, _y, _width, _height)
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut matrix = vec![vec![0; 1000]; 1000];
    let re = Regex::new(r"^#([0-9]*) @ ([0-9]*),([0-9]*): ([0-9]*)x([0-9]*)$").unwrap();

    for line in &lines {
        let (_id, _x, _y, _width, _height) = parse_line(&re, line);

        for i in _x .. (_x +_width) {
            for j in _y .. (_y + _height) {
                matrix[i][j] += 1;
            }
        }
    }

    let mut alone : Option<usize> = None;
    let mut is_valid;

    for line in &lines {
        let (_id, _x, _y, _width, _height) = parse_line(&re, line);

        is_valid = true;

        for i in _x .. (_x +_width) {
            for j in _y .. (_y + _height) {
                if matrix[i][j] != 1 {
                    is_valid = false;
                }
            }
        }

        if is_valid {
            alone = Some(_id);
            break;
        }
    }

    let mut count_non_zero = 0;

    for i in 0..1000 {
        for j in 0..1000 {
            if matrix[i][j] > 1 {
                count_non_zero += 1;
            }
        }
    }

    println!("#1 {}", count_non_zero);
    println!("#2 {}", alone.unwrap());
}
