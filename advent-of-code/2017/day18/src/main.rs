use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
enum InstructionType {
    SND,
    SET,
    ADD,
    MUL,
    MOD,
    RCV,
    JGZ
}

#[derive(Debug, Clone)]
struct Instruction<'a> {
    inst_type : InstructionType,
    arg1: &'a str,
    arg2: &'a str,
}

fn str_to_instruct<'a>(instruction: &'a str, arg1: &'a str, arg2: &'a str) -> Instruction<'a> {
    let inst_type = match instruction {
            "snd" => InstructionType::SND,
            "set" => InstructionType::SET,
            "add" => InstructionType::ADD,
            "mul" => InstructionType::MUL,
            "mod" => InstructionType::MOD,
            "rcv" => InstructionType::RCV,
            "jgz" => InstructionType::JGZ,
            _ => unreachable!()
    };

    Instruction {
        arg1: arg1,
        arg2: arg2,
        inst_type: inst_type,
    }
}

fn get_val(registry: &mut HashMap<char, i128>, arg1: &str) -> i128 {
    if let Ok(v) = arg1.parse::<i128>() {
        v
    } else {
        let reg_name = arg1.chars().nth(0).unwrap();
        *registry.entry(reg_name).or_insert(0)
    }
}

fn set_val(registry: &mut HashMap<char, i128>, arg1: &str, arg2: &str) {
    let reg_name = arg1.chars().nth(0).unwrap();
    let reg_value = get_val(registry, arg2);

    registry.insert(reg_name, reg_value);
}

fn set(registry: &mut HashMap<char, i128>, arg1: &str, arg2: i128) {
    let reg_name = arg1.chars().nth(0).unwrap();

    registry.insert(reg_name, arg2);
}

#[derive(Debug)]
struct Machine<'a> {
    registry: HashMap<char, i128>,
    instructions: Vec<Instruction<'a>>,
    index: usize,
    recv_stack: Vec<i128>,
    sent_stack: Vec<i128>,
    is_recv_stuck: bool,
    sent: usize,
}

fn run(machine: &mut Machine, part1: bool) -> i128 {
    let mut recv = 0;
    let mut play = 0;

    loop {
        if machine.index >= machine.instructions.len() {
            break;
        }

        let curr = &machine.instructions[machine.index];

        match curr.inst_type {
            InstructionType::SET => {
                set_val(&mut machine.registry, curr.arg1, curr.arg2);
            },
            InstructionType::ADD => {
                let v1 = get_val(&mut machine.registry, curr.arg1);
                let v2 = get_val(&mut machine.registry, curr.arg2);

                set(&mut machine.registry, curr.arg1, v1 + v2);
            },
            InstructionType::MUL => {
                let v1 = get_val(&mut machine.registry, curr.arg1);
                let v2 = get_val(&mut machine.registry, curr.arg2);

                set(&mut machine.registry, curr.arg1, v1 * v2);
            },
            InstructionType::MOD => {
                let v1 = get_val(&mut machine.registry, curr.arg1);
                let v2 = get_val(&mut machine.registry, curr.arg2);

                set(&mut machine.registry, curr.arg1, v1 % v2);
            },
            InstructionType::JGZ => {
                let v1 = get_val(&mut machine.registry, curr.arg1);
                let v2 = get_val(&mut machine.registry, curr.arg2);

                if v1 > 0 {
                    machine.index = (machine.index as i128 + v2) as usize;
                    continue;
                }
            },
            InstructionType::RCV => {
                let v1 = get_val(&mut machine.registry, curr.arg1);

                if part1 {
                    if v1 != 0 {
                        recv = play;
                        break;
                    }
                } else {
                    if machine.recv_stack.len() != 0 {
                        let v = machine.recv_stack.pop().unwrap();
                        set(&mut machine.registry, curr.arg1, v);
                    } else {
                        machine.is_recv_stuck = true;
                        break;
                    }
                }
            },
            InstructionType::SND => {
                let v1 = get_val(&mut machine.registry, curr.arg1);

                if part1 {
                    play = v1;
                } else {
                    machine.sent_stack.push(v1);
                    machine.sent += 1;
                }
            }
        }

        machine.index += 1;
    }

    recv
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut instructions : Vec<Instruction> = vec![];

    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let p0 = parts[0];
        let p1 = parts[1];
        let p2 = if parts.len() >= 3 {
            parts[2]
        } else {
            ""
        };
        let instruction = str_to_instruct(p0, p1, p2);

        instructions.push(instruction);
    }

    let mut machine = Machine {
        registry: HashMap::new(),
        instructions: instructions.clone(),
        index: 0,
        recv_stack: Vec::<i128>::new(),
        sent_stack: Vec::<i128>::new(),
        is_recv_stuck: false,
        sent: 0,
    };

    let res = run(&mut machine, true);
    println!("Part #1: {:?}", res);

    let mut machine1 = Machine {
        registry: HashMap::new(),
        instructions: instructions.clone(),
        index: 0,
        recv_stack: Vec::<i128>::new(),
        sent_stack: Vec::<i128>::new(),
        is_recv_stuck: false,
        sent: 0,
    };

    let mut machine2 = Machine {
        registry: HashMap::new(),
        instructions: instructions.clone(),
        index: 0,
        recv_stack: Vec::<i128>::new(),
        sent_stack: Vec::<i128>::new(),
        is_recv_stuck: false,
        sent: 0,
    };

    machine1.registry.insert('p', 0);
    machine2.registry.insert('p', 1);

    loop {
        run(&mut machine1, false);

        while machine1.sent_stack.len() > 0 {
            let item = machine1.sent_stack.pop().unwrap();
            machine2.recv_stack.push(item);
        }

        run(&mut machine2, false);

        while machine2.sent_stack.len() > 0 {
            let item = machine2.sent_stack.pop().unwrap();
            machine1.recv_stack.push(item);
        }

        if machine1.is_recv_stuck && machine1.recv_stack.len() == 0 && machine2.is_recv_stuck && machine2.recv_stack.len() == 0 {
            break;
        }
    }

    println!("Part #2: {}", machine2.sent);
}
