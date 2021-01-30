use std::collections::HashMap;
use std::fs;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
enum InstructionType {
    SET,
    ADD,
    SUB,
    MUL,
    MOD,
    JGZ,
    JNZ,
}

#[derive(Debug, Clone)]
struct Instruction<'a> {
    inst_type : InstructionType,
    arg1: &'a str,
    arg2: &'a str,
}

fn str_to_instruct<'a>(instruction: &'a str, arg1: &'a str, arg2: &'a str) -> Instruction<'a> {
    let inst_type = match instruction {
            "set" => InstructionType::SET,
            "add" => InstructionType::ADD,
            "sub" => InstructionType::SUB,
            "mul" => InstructionType::MUL,
            "mod" => InstructionType::MOD,
            "jgz" => InstructionType::JGZ,
            "jnz" => InstructionType::JNZ,
            _ => {
                println!("Invalid instruction: {:?}", instruction);
                unreachable!()
            },
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

fn run(machine: &mut Machine) -> i128 {
    let mut ret = 0;

    loop {
        if machine.index >= machine.instructions.len() {
            break;
        }

        // println!("{:?}: {:?}", machine.index, machine.registry);

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
            InstructionType::SUB => {
                let v1 = get_val(&mut machine.registry, curr.arg1);
                let v2 = get_val(&mut machine.registry, curr.arg2);

                set(&mut machine.registry, curr.arg1, v1 - v2);
            },
            InstructionType::MUL => {
                ret += 1;
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
            InstructionType::JNZ => {
                let v1 = get_val(&mut machine.registry, curr.arg1);
                let v2 = get_val(&mut machine.registry, curr.arg2);

                if v1 != 0 {
                    machine.index = (machine.index as i128 + v2) as usize;
                    continue;
                }
            },

        }

        machine.index += 1;
    }

    ret
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

    let res = run(&mut machine);
    println!("Part #1: {:?}", res);

    println!("Part #2: 917");
}

/*
With some reverse engineering:
"""
a = 1;
b = 106500;
c = 123500;

do {
    f = 1;
    d = 2;

    do {
        e = 2;

        do {
            if (d * e == b) { f = 0; }
            e++;
        } while(e != b);

        d++;
    } while(d != b);

    if (f == 0) { h++; }
    if (b == c) { return; }

    b += 17;
}
while(true);
"""

def is_prime(n):
    for i in range(2, n//2):
        if (n % i) == 0:
            return False

    return True


a = 1
b = 106500
c = 123500
h = 0

while True:
    f = 1
    d = 2

    if not is_prime(b):
        h += 1

#    while d != b:
#        e = 2
#
#        while e != b:
#            if d * e == b:
#                f = 0
#                break;
#            e += 1
#
#        d += 1
#
#        if f==0:
#            break;
#
#    if f == 0:
#        h += 1

    if b == c:
        break

    b += 17

print(h)


*/
