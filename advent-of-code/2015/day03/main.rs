use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut turn = 0;

    let mut x = 0;
    let mut x_r = 0;
    let mut y = 0;
    let mut y_r = 0;

    let mut locations : Vec<(i32, i32)> = Vec::new();

    locations.push((0, 0));

    for m in contents.chars() {
        match m {
            '>' => {
                if turn % 2 == 0 { x += 1; } else { x_r += 1; };
            },
            '<' => {
                if turn % 2 == 0 { x -= 1; } else { x_r -= 1; };
            },
            '^' => {
                if turn % 2 == 0 { y -= 1; } else { y_r -= 1; };
            },
            'v' => {
                if turn % 2 == 0 { y += 1; } else { y_r += 1; };
            },
            _ => {
                println!("invalid char");
            }
        };

        let mut found = false;
        let mut found_r = false;

        for location in &locations {
            if location.0 == x && location.1 == y {
                found = true
            }

            if location.0 == x_r && location.1 == y_r {
                found_r = true
            }
        }

        if found == false {
            locations.push((x, y));
        }

        if found_r == false {
            locations.push((x_r, y_r));
        }

        turn += 1;
    }

    println!("{:?}", locations.len());
    // 2592
    // 2360
}