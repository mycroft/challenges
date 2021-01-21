use mkz_aoc::file;
use std::cmp::max;

fn run(dirs: &str) -> (usize, usize) {
    let dirs = dirs.split(",").filter(|x| *x != "").collect::<Vec<&str>>();

    let mut x : i32 = 0;
    let mut y : i32 = 0;

    let mut current_distance = 0;
    let mut max_distance = 0;

    for dir in dirs {
        match dir {
            "n" => {
                y -= 2;
            },
            "s" => {
                y += 2;
            }
            "ne" => {
                x += 1;
                y -= 1;
            },
            "se" => {
                x += 1;
                y += 1;
            },
            "nw" => {
                x -= 1;
                y -= 1;
            },
            "sw" => {
                x -= 1;
                y += 1;
            },
            _ => {
                println!("dir: {}", dir);
                unreachable!()
            }
        };

        current_distance = max(x.abs(), (y.abs() % 2 + y.abs()) / 2);
        if current_distance > max_distance {
            max_distance = current_distance;
        }
    }

    (current_distance as usize, max_distance as usize)
}

fn main() {
    let content = file::read_to_string("input.txt").unwrap();

    let _dirs = content.split(",").collect::<Vec<&str>>();
    let _res = run(&content);

    println!("Part #1: {}", _res.0);
    println!("Part #2: {}", _res.1);
}

#[test]
fn name() {
    assert_eq!(0, run("").0);
    assert_eq!(3, run("ne,ne,ne").0);
    assert_eq!(0, run("ne,ne,sw,sw").0);
    assert_eq!(2, run("ne,ne,s,s").0);
    assert_eq!(3, run("se,sw,se,sw,sw").0);
}

