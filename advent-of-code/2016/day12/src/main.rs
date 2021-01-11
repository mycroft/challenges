use std::fs;
use std::collections::HashMap;

fn val(registers: &mut HashMap<char, i128>, reg: &str) -> i128 {
    match reg.parse::<i128>() {
        Ok(val) => val,
        _ => *registers.entry(reg.chars().nth(0).unwrap()).or_insert(0)
    }
}

fn play(instructions: &Vec<&str>, registers: &mut HashMap<char, i128>) -> i128 {
    let mut idx : i128 = 0;

    loop {
        if idx >= instructions.len() as i128 {
            break;
        }

        let line = instructions[idx as usize];
        let _parts = line.split(" ").collect::<Vec<&str>>();

        match _parts[0] {
            "cpy" => {
                let num = _parts[1].parse::<i128>();
                match num {
                    Ok(val) => {
                        registers.insert(_parts[2].chars().nth(0).unwrap(), val);
                    },
                    _ => {
                        let src = _parts[1].chars().nth(0).unwrap();
                        let dst = _parts[2].chars().nth(0).unwrap();

                        let val = *registers.get(&src).unwrap();
                        registers.insert(dst, val);
                    }
                }
            },
            "jnz" => {
                let num = _parts[2].parse::<i128>().unwrap();
                let val = val(registers, _parts[1]);

                if val != 0 {
                    idx += num;
                    continue;
                }
            },
            "inc" => {
                let reg = _parts[1].chars().nth(0).unwrap();
                *registers.entry(reg).or_insert(0) += 1;
            },
            "dec" => {
                let reg = _parts[1].chars().nth(0).unwrap();
                *registers.entry(reg).or_insert(0) -= 1;
            },
            _ => {
                println!("{:?}", _parts);
                unimplemented!();
            }
        }

        idx += 1;
    }

    *registers.get(&'a').unwrap()
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();
    let mut instructions : Vec<&str> = vec![];

    for line in lines {
        instructions.push(line.clone());
    }

    let mut registers : HashMap<char, i128> = HashMap::new();
    println!("Part #1: {:?}", play(&instructions, &mut registers));

    let mut registers : HashMap<char, i128> = HashMap::new();
    registers.insert('c', 1);
    println!("Part #2: {:?}", play(&instructions, &mut registers));
}
