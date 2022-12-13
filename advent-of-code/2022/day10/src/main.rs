use std::fs::read_to_string;

fn parse(fp: &str) -> Vec<String> {
    let contents = read_to_string(fp).unwrap();

    contents.lines().map(|x| x.to_string()).collect()
}

// 20, 60, 100, 140, 180, 220
fn run(prog: &Vec<String>) -> (i32, Vec<bool>) {
    let mut x = 1;
    let mut cycle = 1;
    let mut idx: usize = 0;

    let mut sum = 0;

    let mut current_is_addx = false;
    let watched_cycles = [20, 60, 100, 140, 180, 220];

    let mut sprites : Vec<bool> = Vec::new();

    loop {
        // cycle start
        let instruction_parts: Vec<&str> = prog[idx].split(" ").collect();

        // during cycle
        // println!("cycle:{cycle} x:{x} current instruction: {:?} ~ strengh: {}", instruction_parts, cycle * x);
        if watched_cycles.contains(&cycle) {
            sum += cycle * x;
        }

        sprites.push(
            (x-1..=x+1).contains(&((cycle-1) % 40))
        );

        // end of cycle
        if instruction_parts.len() == 2 && !current_is_addx {
            // middle of the addx instruction
            current_is_addx = true;

        } else if instruction_parts.len() == 2 {
            // end of the addx instruction
            current_is_addx = false;

            x += instruction_parts[1].parse::<i32>().unwrap();
            idx += 1;
        } else {
            idx += 1;
        }

        cycle += 1;

        if idx >= prog.len() {
            break;
        }
    }

    (sum, sprites)
}

fn display_sprites(sprites: &Vec<bool>) {
    for (idx, v) in sprites.iter().enumerate() {
        if idx > 0 && idx % 40 == 0 {
            println!();
        }

        if *v {
            print!("#");
        } else {
            print!(" ");
        }
    }
}
fn main() {
    let prog = parse("input.txt");
    let res = run(&prog);
    println!("#1 {}", res.0);
    display_sprites(&res.1);
}
