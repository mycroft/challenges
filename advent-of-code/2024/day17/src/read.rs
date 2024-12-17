use std::fs;

use crate::{VM, Instruction};

pub fn read_input(fp: &str) -> VM {
    let contents = fs::read_to_string(fp).expect("Error reading file");

    let mut registers = [0; 3];
    let mut program = Vec::new();
    let mut code = Vec::new();

    for (i, line) in contents.lines().enumerate() {
        if i == 0 {
            let mut parts = line.split_whitespace();
            registers[0] = parts.nth(2).unwrap().parse().unwrap();
        } else if i == 1 {
            let mut parts = line.split_whitespace();
            registers[1] = parts.nth(2).unwrap().parse().unwrap();
        } else if i == 2 {
            let mut parts = line.split_whitespace();
            registers[2] = parts.nth(2).unwrap().parse().unwrap();
        } else if i == 4 {
            let mut parts = line.split_whitespace();
            for (idx, num) in parts.nth(1).unwrap().split(",").enumerate() {
                if idx % 2 == 0 {
                    program.push(Instruction::from_int(num.parse().unwrap()));
                } else {
                    program.push(Instruction::Operand(num.parse().unwrap()));
                }
                code.push(num.parse().unwrap());
            }
        }
    }

    VM { registers, ip: 0, program, code, output: Vec::new() }
}
