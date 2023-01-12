use std::collections::HashSet;
use intcode::{parse,Machine};

fn display(hs: &HashSet<(isize, isize)>, size: isize) {
    for y in 0..size {
        for x in 0..size {
            if hs.contains(&(x, y)) {
                print!(".");
            } else {
                print!(" ");
            }
        }
        println!("   - {y}");
    } 
}

fn check(code: &[isize], x: isize, y: isize) -> isize {
    let mut machine = Machine::new(code);
    machine.add_input(x);
    machine.add_input(y);

    machine.run()
}

fn find_100(code: &[isize]) -> isize {
    let mut min_x = 0;
    let mut max_x = 0;

    let mut y = 0;

    // for each line, find out 
    loop {
        let mut x = min_x;

        // we have no points on line 1 & 3.
        // skipping them as I don't want to detect this edge case.
        if y == 1 || y == 3 {
            y += 1;
            continue;
        }

        // find new min_x for given y
        loop {
            if check(code, x, y) == 0 {
                min_x += 1;
            } else {
                break;
            }
            x += 1;
        }

        x = max_x;

        // find new max_x for given y
        loop {
            if check(code, x, y) == 1 || max_x <= min_x {
                max_x += 1;
            } else {
                max_x -= 1;
                break;
            }
            x += 1;
        }

        // now that we've got a line for N, check the related other square points are 1.
        // We're starting at (y, max - 1) -> (y + 99, max - 1)

        let sq_size = 100;

        if max_x - min_x >= sq_size {
            let p1 = check(code, max_x, y + (sq_size - 1));
            let p2 = check(code, max_x - (sq_size - 1), y + (sq_size - 1));

            // println!("y:{y} {min_x} -> {} p1:{p1} p2:{p2}", max_x - 1);

            if p1 == 1 && p2 == 1 {
                // println!("Found possible case at x:{} y:{}", max_x - (sq_size - 1), y);
                break (max_x - (sq_size - 1)) * 10000 + y
            }
        } else {
            // println!("y:{y} {min_x} -> {}", max_x - 1);
        }

        y += 1;
    }
}


fn main() {
    let code = parse("input.txt");
    let mut hs = HashSet::new();

    let size = 100;

    let mut nums = 0;

    for y in 0..size {
        for x in 0..size {
            let mut machine = Machine::new(&code);
            machine.add_input(x);
            machine.add_input(y);

            let res = machine.run();
            if res == 1 {
                nums += 1;
                hs.insert((x, y));
            }
        }
    }

    display(&hs, size);

    println!("#1 {}", nums); // 632
    println!("#2 {}", find_100(&code)); // 6191165
}
