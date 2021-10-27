/*
 * AOC 2018 - 16
 */
use std::fs;
use std::collections::{HashMap, HashSet};

type OPF = fn(&mut Device, i64, i64, i64);

#[derive(Debug)]
struct Discovery {
    before: Vec<i64>,
    opcodes: Vec<i64>,
    after: Vec<i64>,
}

#[derive(Debug)]
struct Device {
    registers : Vec<i64>,
}

impl Device {
    fn addr(self: &mut Device, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] + self.registers[b as usize]
    }

    fn addi(self: &mut Device, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] + b
    }

    fn mulr(self: &mut Device, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] * self.registers[b as usize]
    }

    fn muli(self: &mut Device, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] * b
    }

    fn banr(self: &mut Device, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] & self.registers[b as usize]
    }

    fn bani(self: &mut Device, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] & b
    }

    fn borr(self: &mut Device, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] | self.registers[b as usize]
    }

    fn bori(self: &mut Device, a: i64, b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize] | b
    }

    fn setr(self: &mut Device, a: i64, _b: i64, c: i64) {
        self.registers[c as usize] = self.registers[a as usize]
    }

    fn seti(self: &mut Device, a: i64, _b: i64, c: i64) {
        self.registers[c as usize] = a
    }

    fn gtir(self: &mut Device, a: i64, b: i64, c: i64) {
        if a > self.registers[b as usize] {
            self.registers[c as usize] = 1;    
        } else {
            self.registers[c as usize] = 0;
        }
    }

    fn gtri(self: &mut Device, a: i64, b: i64, c: i64) {
        if self.registers[a as usize] > b {
            self.registers[c as usize] = 1;    
        } else {
            self.registers[c as usize] = 0;
        }
    }

    fn gtrr(self: &mut Device, a: i64, b: i64, c: i64) {
        if self.registers[a as usize] > self.registers[b as usize] {
            self.registers[c as usize] = 1;    
        } else {
            self.registers[c as usize] = 0;
        }
    }

    fn eqir(self: &mut Device, a: i64, b: i64, c: i64) {
        if a == self.registers[b as usize] {
            self.registers[c as usize] = 1;    
        } else {
            self.registers[c as usize] = 0;
        }
    }

    fn eqri(self: &mut Device, a: i64, b: i64, c: i64) {
        if self.registers[a as usize] == b {
            self.registers[c as usize] = 1;    
        } else {
            self.registers[c as usize] = 0;
        }
    }

    fn eqrr(self: &mut Device, a: i64, b: i64, c: i64) {
        if self.registers[a as usize] == self.registers[b as usize] {
            self.registers[c as usize] = 1;    
        } else {
            self.registers[c as usize] = 0;
        }
    }
}

fn main() {
    let mut device = Device { registers: vec![0; 4] };
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

    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut before_registers = vec![];
    let mut opcodes = vec![];
    let mut after_registers;
    let mut in_discovery = false;

    let mut discovery_entries : Vec<Discovery> = vec![];
    let mut instructions : Vec<Vec<i64>> = vec![];

    for line in &lines {
        if line.starts_with("Before: [") {
            before_registers = line[line.find('[').unwrap()+1..line.find(']').unwrap()]
                .split(", ")
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            in_discovery = true;
            continue;
        }

        if line.starts_with("After:  [") {
            after_registers = line[line.find('[').unwrap()+1..line.find(']').unwrap()]
                .split(", ")
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            in_discovery = false;

            discovery_entries.push(
                Discovery{
                    before: before_registers.clone(),
                    opcodes: opcodes.clone(),
                    after: after_registers,
                }
            );

            continue;
        }

        if *line == "" {
            continue;
        }

        opcodes = line.split(" ").map(|c| c.parse::<i64>().unwrap()).collect::<Vec<i64>>();

        if !in_discovery {
            instructions.push(opcodes.clone());
        }
    }

    let mut opcodes_bint : HashMap<i64, HashSet<String>> = HashMap::new();

    let mut part1_count = 0;

    for entry in discovery_entries {
        let mut possible_opcodes : HashSet<String> = HashSet::new();

        for (name, opcode) in &opcodes_bname {
            device.registers = entry.before.clone();

            opcode(
                &mut device,
                entry.opcodes[1],
                entry.opcodes[2],
                entry.opcodes[3],
            );

            if device.registers == entry.after {
                possible_opcodes.insert(name.to_string());
            }
        }

        if possible_opcodes.len() >= 3 {
            part1_count += 1;
        }

        if opcodes_bint.contains_key(&entry.opcodes[0]) {
            let intersect : HashSet<String> = possible_opcodes
                .intersection(opcodes_bint.get(&entry.opcodes[0]).unwrap())
                .map(|x| x.to_string())
                .collect();

            opcodes_bint.insert(
                entry.opcodes[0],
                intersect
            );
        } else {
            opcodes_bint.insert(entry.opcodes[0], possible_opcodes);
        }
    }

    loop {
        let mut done : Vec<String> = vec![];
        let mut removed = false;

        for (_, v) in &opcodes_bint {
            if v.len() != 1 {
                continue;
            }

            v.iter().map(|v| done.push(v.to_string())).count();
        }

        for (_k, v) in opcodes_bint.iter_mut() {
            if v.len() == 1 {
                continue;
            }

            for to_remove in &done {
                v.remove(to_remove);
                removed = true;
            }
        }

        if !removed {
            break;
        }
    }

    // Play instructions.
    let mut device = Device { registers: vec![0; 4] };

    for inst in instructions {
        let opcode = inst[0];
        let op = opcodes_bint.get(&opcode).unwrap();
        let h = opcodes_bname.get(op.iter().nth(0).unwrap()).unwrap();

        h(&mut device, inst[1], inst[2], inst[3]);
    }

    println!("#1: {}", part1_count);
    println!("#2: {}", device.registers[0]);
}
