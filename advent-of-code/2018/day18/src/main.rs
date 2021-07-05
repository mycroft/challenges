use std::fs;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
struct Position {
    x: usize,
    y: usize,
}

const FILE : &str = "input.txt";
const LAND_SIZE : usize = 50;

#[derive(Copy, Clone, Debug)]
struct Land {
    contents: [[char; LAND_SIZE]; LAND_SIZE],
    size: usize,
}

fn dump(t: Land) {
    for l in t.contents.iter() {
        for c in l.iter() {
            print!("{}", c);
        }
        println!("");
    }
}

// returns: open (.), tree (|), lumberyard (#)
fn adj(t: Land, p: Position) -> (usize, usize, usize) {
    let deltas = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    let mut res = (0, 0, 0);

    for delta in deltas.iter() {
        let x : isize = p.x as isize + delta.0 as isize;
        let y : isize = p.y as isize + delta.1 as isize;

        if x < 0 || x >= t.size as isize || y < 0 || y >= t.size as isize {
            continue;
        }

        let l = t.contents[y as usize][x as usize];
        // println!("{:?} {:?} -> {:?}", x as usize, y as usize, l);
        match l {
            '.' => { res.0 += 1; },
            '|' => { res.1 += 1; },
            '#' => { res.2 += 1; },
            _ => unreachable!()
        }
    }

    res
}

fn count(land: Land) -> (usize, usize, usize) {
    let mut res = (0, 0, 0);

    for l in land.contents.iter() {
        for c in l.iter() {
            match c {
                '.' => { res.0 += 1; },
                '|' => { res.1 += 1; },
                '#' => { res.2 += 1; },
                _ => unreachable!(),
            }
        }
    }

    res
}

fn gen(oldland: Land) -> Land {
    let mut land = Land{
        contents: [['.'; LAND_SIZE]; LAND_SIZE],
        size: LAND_SIZE,
    };

    let mut y = 0;

    for l in oldland.contents.iter() {
        let mut x = 0;
        for c in l.iter() {
            let r = adj(oldland, Position{x: x, y: y});
            // println!("{:?}", r);

            match c {
                '.' => {
                    land.contents[y][x] = if r.1 >= 3 {
                        '|'
                    } else {
                        '.'
                    };
                },
                '|' => {
                    land.contents[y][x] = if r.2 >= 3 {
                        '#'
                    } else {
                        '|'
                    };
                },
                '#' => {
                    land.contents[y][x] = if r.1 >= 1 && r.2 >= 1 {
                        '#'
                    } else {
                        '.'
                    };
                },
                _ => unreachable!()
            }

            x += 1;
        }

        y += 1;
    }

    land
}

fn main() {
    let contents = fs::read_to_string(FILE).unwrap();
    let lines = contents.lines();

    let mut land = Land{
        contents: [['.'; LAND_SIZE]; LAND_SIZE],
        size: LAND_SIZE,
    };

    let mut y = 0;

    for line in lines {
        let mut x = 0;
        for c in line.chars() {
            land.contents[y][x] = c;
            x += 1;
        }

        y += 1;
    }

    println!("State:");
    dump(land);

    let mut minutes = 0;

    for _ in 1..=10 {
        land = gen(land);
        minutes += 1;
        println!("After {} minutes:", minutes);
        dump(land);
    }

    let mut counts = count(land);
    println!("Part #1: {}", counts.1 * counts.2);

    let mut results : HashMap<(usize, usize, usize), usize>  = HashMap::new();

    // continue a bit to start seeing always the same pattern
    for _ in 10..=500 {
        land = gen(land);
        minutes += 1;

    }

    let mut jumped = false;

    loop {
        land = gen(land);
        minutes += 1;

        counts = count(land);

        if minutes == 1000000000 {
            break;
        }

        if results.contains_key(&counts) && !jumped {
            let cycle_length = minutes - results.get(&counts).unwrap();

            println!("Jumping of {}", (1000000000 - minutes) - (1000000000 - minutes) % cycle_length);
            minutes += (1000000000 - minutes) - (1000000000 - minutes) % cycle_length;

            jumped = true;
        } else {
            results.insert(counts, minutes);
        }
    }

    println!("Part #1: {}", counts.1 * counts.2);
}
