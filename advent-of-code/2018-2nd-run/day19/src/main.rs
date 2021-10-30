/*
 * AOC 2018 - 19
 */

use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
struct Instruction {
    opcode: String,
    args: Vec<i64>
}

#[derive(Debug)]
struct Device<'a> {
    ip_register: usize,
    registers: Vec<i64>,
    instructions: &'a Vec<Instruction>,
}

type OPF<'a> = fn(&mut Device<'a>, i64, i64, i64);

impl<'a> Device<'a> {
    fn addr(self: &mut Device<'a>, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] + self.registers[b as usize]
    }

    fn addi(self: &mut Device<'a>, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] + b
    }

    fn mulr(self: &mut Device<'a>, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] * self.registers[b as usize]
    }

    fn muli(self: &mut Device<'a>, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] * b
    }

    fn banr(self: &mut Device<'a>, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] & self.registers[b as usize]
    }

    fn bani(self: &mut Device<'a>, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] & b
    }

    fn borr(self: &mut Device<'a>, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] | self.registers[b as usize]
    }

    fn bori(self: &mut Device<'a>, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] | b
    }

    fn setr(self: &mut Device<'a>, a: i64, _b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize]
    }

    fn seti(self: &mut Device<'a>, a: i64, _b: i64, c: i64) {
        self.registers[c as usize] = a
    }

    fn gtir(self: &mut Device<'a>, a: i64, b: i64, c: i64) {
        if a > self.registers[b as usize] {
            self.registers[c as usize] = 1;    
        } else {
            self.registers[c as usize] = 0;
        }
    }

    fn gtri(self: &mut Device<'a>, a: i64, b: i64, c: i64) {
        if self.registers[a as usize] > b {
            self.registers[c as usize] = 1;    
        } else {
            self.registers[c as usize] = 0;
        }
    }

    fn gtrr(self: &mut Device<'a>, a: i64, b: i64, c: i64) {
        if self.registers[a as usize] > self.registers[b as usize] {
            self.registers[c as usize] = 1;    
        } else {
            self.registers[c as usize] = 0;
        }
    }

    fn eqir(self: &mut Device<'a>, a: i64, b: i64, c: i64) {
        if a == self.registers[b as usize] {
            self.registers[c as usize] = 1;    
        } else {
            self.registers[c as usize] = 0;
        }
    }

    fn eqri(self: &mut Device<'a>, a: i64, b: i64, c: i64) {
        if self.registers[a as usize] == b {
            self.registers[c as usize] = 1;    
        } else {
            self.registers[c as usize] = 0;
        }
    }

    fn eqrr(self: &mut Device<'a>, a: i64, b: i64, c: i64) {
        if self.registers[a as usize] == self.registers[b as usize] {
            self.registers[c as usize] = 1;    
        } else {
            self.registers[c as usize] = 0;
        }
    }
}

fn opscode_map<'a>() -> HashMap<String, OPF<'a>> {
    let mut opcodes_bname : HashMap<String, OPF> = HashMap::new();

    opcodes_bname.insert("addr".to_string(), Device::addr);
    opcodes_bname.insert("addi".to_string(), Device::addi);

    opcodes_bname.insert("mulr".to_string(), Device::mulr);
    opcodes_bname.insert("muli".to_string(), Device::muli);

    opcodes_bname.insert("banr".to_string(), Device::banr);
    opcodes_bname.insert("bani".to_string(), Device::bani);

    opcodes_bname.insert("borr".to_string(), Device::borr);
    opcodes_bname.insert("bori".to_string(), Device::bori);

    opcodes_bname.insert("setr".to_string(), Device::setr);
    opcodes_bname.insert("seti".to_string(), Device::seti);

    opcodes_bname.insert("gtir".to_string(), Device::gtir);
    opcodes_bname.insert("gtri".to_string(), Device::gtri);
    opcodes_bname.insert("gtrr".to_string(), Device::gtrr);

    opcodes_bname.insert("eqir".to_string(), Device::eqir);
    opcodes_bname.insert("eqri".to_string(), Device::eqri);
    opcodes_bname.insert("eqrr".to_string(), Device::eqrr);

    opcodes_bname
}

fn run(device: &mut Device) -> i64 {
    let opscodes = opscode_map();

    let is_part_2 = device.registers[0] == 1;

    loop {
        let ip = device.registers[device.ip_register];

        if ip as usize >= device.instructions.len() {
            break;
        }
        
        let current_instruction = &device.instructions[ip as usize];
        let f = opscodes.get(&current_instruction.opcode).unwrap();

        /*
        println!("IP: {} INST: {:?} REG: {:?}",
            ip,
            current_instruction,
            device.registers,
        );
        */

        f(
            device,
            current_instruction.args[0],
            current_instruction.args[1],
            current_instruction.args[2],
        );

        if ip == 1 && is_part_2 {
            // The program is basically computing the sum of factors of one of the registers here:
            // println!("REGS; {:?}", device.registers);

            // In my case, it was register 1 (but was also the max value of the registers)
            let n = *device.registers.iter().max().unwrap();
            return (1..=n).filter(|x| n % x == 0).sum();
        }

        device.registers[device.ip_register] += 1;
    }

    device.registers[0]
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut initial_ip = 0i64;
    let mut instructions : Vec<Instruction> = vec![];

    for (line_id, line) in lines.iter().enumerate() {
        if line_id == 0 {
            initial_ip = line.split(" ").nth(1).unwrap().parse::<i64>().unwrap();
            continue;
        }

        let op = line.split(" ").nth(0).unwrap();
        let args = line[1+line.find(" ").unwrap()..].split(" ").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>();
        instructions.push(
            Instruction {
                opcode: op.to_string(),
                args: args,
            }
        );
    }

    // println!("IP: #{}", initial_ip);
    // println!("{:?}", instructions);

    let res = run(&mut Device {
        ip_register: initial_ip as usize,
        registers: vec![0; 6],
        instructions: &instructions,
    });

    println!("Part #1: {}", res);

    let res = run(&mut Device {
        ip_register: initial_ip as usize,
        registers: vec![1, 0, 0, 0, 0, 0],
        instructions: &instructions,
    });

    println!("Part #2: {}", res);

}
