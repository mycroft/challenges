use std::fs;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Location {
    x: isize,
    y: isize,
}

fn move_rocks(cube_shapped_rocks: &HashSet<Location>, max_x: isize, max_y: isize, rounded_rocks: &mut HashSet<Location>) {
    for y in 0..=max_y {
        for x in 0..=max_x {
            // if there is a rounded rock here
            let rock = rounded_rocks.get(&Location{x: x as isize, y: y as isize});
            if rock.is_none() {
                continue;
            }
            let mut rock = rock.unwrap().clone();
            rounded_rocks.remove(&rock);

            let mut new_y = rock.y - 1;

            loop {
                if new_y < 0 || rounded_rocks.contains(&Location{x: x as isize, y: new_y}) || cube_shapped_rocks.contains(&Location{x: x as isize, y: new_y}) {
                    break;
                }

                new_y -= 1;
            }

            rock.y = new_y+1;
            rounded_rocks.insert(rock);
        }
    }
}

// we flip the board so what was facing west is now facing north
// this means x=0 becomes y=0, y=0 becomes x=max_x, x=max_x becomes y=max_y and y=max_y becomes x=0
fn flip(cube_shapped_rocks: &HashSet<Location>, rounded_rocks: &HashSet<Location>, max_x: isize, max_y: isize) -> (HashSet<Location>, HashSet<Location>, isize, isize) {
    let mut new_cube_shapped_rocks = HashSet::new();
    let mut new_rounded_rocks = HashSet::new();

    let new_max_x = max_y;
    let new_max_y = max_x;

    for rock in cube_shapped_rocks {
        let loc = Location{
            x: max_y - rock.y,
            y: rock.x};
        new_cube_shapped_rocks.insert(loc);

        // println!("x:{} y:{} -> x:{} y:{}", rock.x, rock.y, loc.x, loc.y);
    }

    for rock in rounded_rocks {
        let loc = Location{
            x: max_y - rock.y,
            y: rock.x};
            new_rounded_rocks.insert(loc);

        // println!("x:{} y:{} -> x:{} y:{}", rock.x, rock.y, loc.x, loc.y);
    }


    //for rock in rounded_rocks {
    //    new_rounded_rocks.insert(Location{
    //        x: rock.y,
    //        y: max_x - rock.y,
    //    });
    //}

    (new_cube_shapped_rocks, new_rounded_rocks, new_max_x, new_max_y)
}

fn draw(rounded_rocks: &HashSet<Location>, cube_shapped_rocks: &HashSet<Location>, max_x: isize, max_y: isize) {
    println!("dimensions: {}/{}", max_x, max_y);
    for y in 0..=max_y {
        for x in 0..=max_x {
            if cube_shapped_rocks.contains(&Location{x: x as isize, y: y as isize}) {
                print!("#");
            } else if rounded_rocks.contains(&Location{x: x as isize, y: y as isize}) {
                print!("O");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn compute_score(rounded_rocks: &HashSet<Location>, max_x: isize, max_y: isize) -> isize {
    let mut score = 0;

    for y in 0..=max_y {
        for x in 0..=max_x {
            let rock = rounded_rocks.get(&Location{x: x as isize, y: y as isize});
            if rock.is_none() {
                continue;
            }

            score += (max_y + 1) - y;
        }
    }

    score
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("a file to open");
    let lines = contents.lines();
    let mut rounded_rocks = HashSet::new();
    let mut cube_shapped_rocks = HashSet::new();

    let mut max_y = 0;
    let mut max_x = 0;

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '#' => cube_shapped_rocks.insert(Location{x: x as isize, y: y as isize}),
                'O' => rounded_rocks.insert(Location{x: x as isize, y: y as isize}),
                '.' => false,
                _ => unreachable!(),
            };

            max_x = x as isize;
        }

        max_y = y as isize;
    }

    // draw(&rounded_rocks, &cube_shapped_rocks, max_x, max_y);

    // move all rounded rock on the north
    move_rocks(&cube_shapped_rocks, max_x, max_y, &mut rounded_rocks);

    // draw(&rounded_rocks, &cube_shapped_rocks, max_x, max_y);

    println!("#1: {}", compute_score(&rounded_rocks, max_x, max_y)); // 106517

    (cube_shapped_rocks, rounded_rocks, max_x, max_y) = flip(&cube_shapped_rocks, &rounded_rocks, max_x, max_y);
    move_rocks(&cube_shapped_rocks, max_x, max_y, &mut rounded_rocks);
    (cube_shapped_rocks, rounded_rocks, max_x, max_y) = flip(&cube_shapped_rocks, &rounded_rocks, max_x, max_y);
    move_rocks(&cube_shapped_rocks, max_x, max_y, &mut rounded_rocks);
    (cube_shapped_rocks, rounded_rocks, max_x, max_y) = flip(&cube_shapped_rocks, &rounded_rocks, max_x, max_y);
    move_rocks(&cube_shapped_rocks, max_x, max_y, &mut rounded_rocks);
    (cube_shapped_rocks, rounded_rocks, max_x, max_y) = flip(&cube_shapped_rocks, &rounded_rocks, max_x, max_y);


    // draw(&rounded_rocks, &cube_shapped_rocks, max_x, max_y);

    let mut cycles_results : HashMap<isize, usize> = HashMap::new();

    let mut cycle = 1;
    let mut max_period = 0;

    loop {
        cycle += 1;
    
        for _ in 0..4 {
            move_rocks(&cube_shapped_rocks, max_x, max_y, &mut rounded_rocks);
            (cube_shapped_rocks, rounded_rocks, max_x, max_y) = flip(&cube_shapped_rocks, &rounded_rocks, max_x, max_y);

        }

        let res = compute_score(&rounded_rocks, max_x, max_y);

        if cycles_results.contains_key(&res) {
            let period = cycle - cycles_results.get(&res).unwrap();
            if period > max_period {
                max_period = period;
            }

            if (1000000000-cycle) % max_period == 0 {
                // println!("Candidat value: {} with period {}", res, max_period);
                println!("#2: {}", res);

                break;
            }
        }

        if cycle > 200 {
            cycles_results.insert(res, cycle);
        }
    }

}
