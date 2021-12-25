use std::{fs, panic};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cucumber {
    None,
    East,
    South,
}

type Sea = Vec<Vec<Cucumber>>;

fn read(fp: &str) -> Sea {
    let mut res = vec![];
    let contents = fs::read_to_string(fp).expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();

    for line in lines {
        let mut line_vec = vec![];

        for c in line.chars() {
            line_vec.push(match c {
                '.' => Cucumber::None,
                '>' => Cucumber::East,
                'v' => Cucumber::South,
                _ => panic!("This should not happen"),
            });
        }

        res.push(line_vec);
    }

    res
}

fn dump(sea: &Sea) {
    for l in sea {
        for c in l {
            let c = match *c {
                Cucumber::None => '.',
                Cucumber::East => '>',
                Cucumber::South => 'v',
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn step(sea: &Sea) -> (usize, Sea) {
    let mut new_sea: Vec<Vec<Cucumber>> = sea.clone();
    let mut moves_number = 0;

    for (y_index, y) in sea.iter().enumerate() {
        for (x_index, &x) in y.iter().enumerate() {
            let new_x_index;
            let new_y_index;

            match x {
                Cucumber::East => {
                    new_x_index = (x_index + 1) % sea[0].len();
                    new_y_index = y_index;
                },
                Cucumber::South => continue,
                Cucumber::None => continue,
            };
    
            if sea[new_y_index][new_x_index] == Cucumber::None {
                new_sea[new_y_index][new_x_index] = x;
                new_sea[y_index][x_index] = Cucumber::None;

                moves_number += 1;
            }
        }
    }

    let sea = new_sea.clone();

    for (y_index, y) in sea.iter().enumerate() {
        for (x_index, &x) in y.iter().enumerate() {
            let new_x_index;
            let new_y_index;

            match x {
                Cucumber::East => continue,
                Cucumber::South => {
                    new_x_index = x_index;
                    new_y_index = (y_index + 1) % sea.len();  
                }
                Cucumber::None => continue,
            };
    
            if sea[new_y_index][new_x_index] == Cucumber::None {
                new_sea[new_y_index][new_x_index] = x;
                new_sea[y_index][x_index] = Cucumber::None;

                moves_number += 1;
            }
        }
    }


    (moves_number, new_sea)
}

fn main() {
    let mut sea = read("input.txt");
    let mut steps = 0;

    // println!("Initial state:");
    // dump(&sea);

    loop {
        let res = step(&sea);
        steps += 1;

        if res.0 == 0 {
            // println!("No more move");
            break;
        }

        sea = res.1;

        // println!("After {} step(s):", steps);
        // dump(&sea);
    }

    println!("#1 {}", steps);
}
