use std::fs;
use anyhow::Result;

#[macro_use] extern crate scan_fmt;

fn main() -> Result<()> {
    let contents = fs::read_to_string("input.txt").expect("A file to open");
    let lines = contents.lines();

    let mut p1 = 0;
    let mut p2 = 0;

    // 12 red cubes, 13 green cubes, and 14 blue cubes
    for line in lines {
        let mut p = line.split(": ");
        let game_id = p.next().unwrap().split(" ").nth(1).unwrap().parse::<u32>().unwrap();
        let games = p.next().unwrap().split("; ");

        let mut possible = true;
        let mut min_red: i32 = -1;
        let mut min_blue: i32 = -1;
        let mut min_green: i32 = -1;

        for game in games {
            let game_parts = game.split(", ");
            for game_part in game_parts {
                if let Ok((num, color)) = scan_fmt!(game_part, "{} {}", i32, String) {
                    if color == String::from("red") {
                        if num > 12 {
                            possible = false;
                        }
                        if min_red < num || min_red == -1 {
                            min_red = num;
                        }
                    } else if color== String::from("green") {
                        if num > 13 {
                            possible = false;
                        }
                        if min_green < num || min_green == -1 {
                            min_green = num;
                        }
                    } else if color == String::from("blue") {
                        if num > 14 {
                            possible = false;
                        }
                        if min_blue < num || min_blue == -1 {
                            min_blue = num;
                        } 
                    }
                }
            }
        }

        p2 += min_blue * min_green * min_red;

        if possible {
            p1 += game_id;
        }
    }

    println!("#1 {}", p1);
    println!("#1 {}", p2);

    Ok(())
}
