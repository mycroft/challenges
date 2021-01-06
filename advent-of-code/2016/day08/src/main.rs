use regex::Regex;
use std::fs;

fn dump(grid: &Vec<Vec<bool>>) {
    for j in 0..grid[0].len() {
        let mut s = String::new();
        for i in 0..grid.len() {
            if grid[i][j] {
                s.push('#');
            } else {
                s.push(' ');
            }
        }
        println!("{}", s);
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();

    let re_rect = Regex::new(r"rect (\d+)x(\d+)").unwrap();
    let re_rotate = Regex::new(r"^rotate (.*)=(\d+) by (\d+)$").unwrap();

    let x = 50;
    let y = 6;

    let mut grid : Vec<Vec<bool>> = vec![vec![false; y]; 50];

    for line in lines {
        if re_rect.is_match(line) {
            let captures = re_rect.captures(line).unwrap();

            for i in 0..captures.get(1).unwrap().as_str().parse::<usize>().unwrap() {
                for j in 0..captures.get(2).unwrap().as_str().parse::<usize>().unwrap() {
                    grid[i][j] = true;
                }
            }

            continue;
        }

        let captures = re_rotate.captures(line).unwrap();

        if captures.get(1).unwrap().as_str() == "column x" {
            let column_x = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let by = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();

            for _i in 0..by {
                let mut old = vec![false; y];
                for j in 0..y {
                    old[j] = grid[column_x][j];
                }

                for j in 0..6 {
                    grid[column_x][(j + 1) % y] = old[j];
                }
            }

        } else {
            let row_y = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let by = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();

            for _i in 0..by {
                let mut old = vec![false; x];
                for j in 0..x {
                    old[j] = grid[j][row_y];
                }

                for j in 0..x {
                    grid[(j + 1) % x][row_y] = old[j];
                }

            }
        }
    }

    let mut count = 0;

    for i in 0..x {
        for j in 0..y {

            if grid[i][j] {
                count += 1;
            }
        }
    }

    println!("Part #1: {:?}", count);

    println!("Part #2: ");
    dump(&grid);
}
