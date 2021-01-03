use std::fs;
use std::collections::HashMap;

fn getvalue<'a>(registry: &mut HashMap<String, i128>, label: &'a String) -> i128 {
    registry.entry(label.to_string()).or_insert(0);

    *registry.get(label).unwrap()
}

fn run(instructions : &Vec<&str>, registry: &mut HashMap<String, i128>) -> i128 {
    let mut idx : i32 = 0;

    loop {
        if idx as usize >= instructions.len() {
            break;
        }

        let line = instructions[idx as usize];
        // println!("{}: {:?}", idx, line);

        let parts : Vec<String> = line
            .split(" ")
            .map(|x| x
                .chars()
                .filter(|y| *y != ',')
                .collect::<String>()
            )
            .collect();

        match parts[0].as_str() {
            "hlf" => {
                let value = getvalue(registry, &parts[1].to_string());
                registry.insert(parts[1].to_string(), value / 2);
                idx += 1;
            },
            "inc" => {
                let value = getvalue(registry, &parts[1]);
                registry.insert(parts[1].to_string(), value + 1);
                idx += 1;
            },
            "jio" => {
                let value = getvalue(registry, &parts[1]);
                if value == 1 {
                    idx += parts[2].parse::<i32>().unwrap();
                } else {
                    idx += 1;
                }
            },
            "tpl" => {
                let value = getvalue(registry, &parts[1]);
                registry.insert(parts[1].to_string(), value * 3);
                idx += 1;
            },
            "jmp" => {
                idx += parts[1].parse::<i32>().unwrap();
            },
            "jie" => {
                let value = getvalue(registry, &parts[1].to_string());
                if value % 2 == 0 {
                    idx += parts[2].parse::<i32>().unwrap();
                } else {
                    idx += 1;
                }
            },
            _ => {
                unimplemented!();
            }
        }
    }

    getvalue(registry, &"b".to_string())
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let instructions = contents.lines().collect::<Vec<&str>>();

    let mut registry : HashMap<String, i128> = HashMap::new();

    println!("Part #1: {:?}", run(&instructions, &mut registry));


    let mut registry : HashMap<String, i128> = HashMap::new();
    registry.insert("a".to_string(), 1);

    println!("Part #2: {:?}", run(&instructions, &mut registry));
}
