use std::fs;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Pair {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    a: Pair,
    b: Pair,
    prize: Pair,
}


fn read_input(fp: &str) -> Vec<Machine> {
    let contents = fs::read_to_string(fp).expect("Error reading the file");
    let mut machines = Vec::new();

    let re = Regex::new(r".*[+=](\d+), .*[+=](\d+)$").unwrap();

    let mut line_num = 0;
    let mut a = Pair {x: 0, y: 0};
    let mut b = Pair {x: 0, y: 0};

    for line in contents.lines() {
        if line_num % 4 == 3 {
            line_num += 1;
            continue;
        }

        let caps = re.captures(line).unwrap();

        if line_num % 4 == 0 {
            a = Pair {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
            };    
        } else if line_num % 4 == 1 {
            b = Pair {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
            };
        } else if line_num % 4 == 2 {
            let prize = Pair {
                x: caps[1].parse().unwrap(),
                y: caps[2].parse().unwrap(),
            };

            let machine = Machine {
                a,
                b,
                prize,
            };
        
            machines.push(machine);
        }

        line_num += 1;
    }

    machines
}

fn solve(machine: &Machine) -> isize {
    for n in 0..100 {
        if (machine.prize.x - n * machine.a.x) % machine.b.x == 0 {
            if (machine.prize.y - n * machine.a.y) % machine.b.y == 0 {
                let b_tokens = (machine.prize.x - n * machine.a.x) / machine.b.x;
                if b_tokens == (machine.prize.y - n * machine.a.y) / machine.b.y {
                    return 3 * n + b_tokens;
                }
            }
        }
    }

    for n in 0..100 {
        if (machine.prize.x - n * machine.b.x) % machine.a.x == 0 {
            if (machine.prize.y - n * machine.b.y) % machine.a.y == 0 {
                let b_tokens = (machine.prize.x - n * machine.b.x) / machine.a.x;
                if b_tokens == (machine.prize.y - n * machine.b.y) / machine.a.y {
                    return n + 3 * b_tokens;
                }
            }
        }
    }

    0
}

fn solve_step2(machine: &Machine) -> isize {
    let den = machine.a.x * machine.b.y - machine.b.x * machine.a.y;
    if den == 0 {
        return -1;
    }
    let offset = 10000000000000;

    let prize_x = machine.prize.x + offset;
    let prize_y = machine.prize.y + offset;

    let x = (prize_x * machine.b.y - machine.b.x * prize_y) / den;
    let y = (machine.a.x * prize_y - prize_x * machine.a.y) / den;

    if x >= 0 && y >= 0 && machine.a.x * x + machine.b.x * y == prize_x && machine.a.y * x + machine.b.y * y == prize_y {
        return 3 * x + y;
    }

    0
}

fn solve_all(machines: &Vec<Machine>) -> isize {
    let mut total = 0;
    for machine in machines {
        total += solve(machine);
    }

    total
}

fn solve_step2_all(machines: &Vec<Machine>) -> isize {
    let mut total = 0;
    for machine in machines {
        total += solve_step2(machine);
    }
    total
}

fn main() {
    let machines = read_input("input.txt");

    println!("#1 {}", solve_all(&machines));
    println!("#2 {}", solve_step2_all(&machines));
}
