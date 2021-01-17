use std::collections::HashMap;
use std::fs;

#[derive(Debug,Clone,Eq,PartialEq)]
enum InstructionType {
    CPY,
    DEC,
    INC,
    JNZ,
    TGL,
    OUT,
}

#[derive(Debug,Clone)]
struct Instruction {
    inst: InstructionType,
    arg1: Option<String>,
    arg2: Option<String>,
}

impl Instruction {
    fn new(input: String) -> Self {
        let parts = input.split(" ").collect::<Vec<&str>>();

        let _instruction = match parts[0] {
            "cpy" => InstructionType::CPY,
            "dec" => InstructionType::DEC,
            "inc" => InstructionType::INC,
            "jnz" => InstructionType::JNZ,
            "tgl" => InstructionType::TGL,
            "out" => InstructionType::OUT,
            _ => {
                println!("{:?}", parts);
                unreachable!()
            },
        };

        let arg1 = Some(parts[1].to_string());

        let arg2 = if parts.len() == 3 {
            Some(parts[2].to_string())
        } else {
            None
        };

        Instruction {
            inst: _instruction,
            arg1: arg1,
            arg2: arg2,
        }
    }

    fn dump(self: &Self) {
        println!("{:?}", self);
    }

    fn valid(self: &Self) -> bool {
        true
    }
/*
tgl x toggles the instruction x away (pointing at instructions like jnz does:
positive means forward; negative means backward):

For one-argument instructions, inc becomes dec, and all other one-argument instructions become inc.
For two-argument instructions, jnz becomes cpy, and all other two-instructions become jnz.
The arguments of a toggled instruction are not affected.
If an attempt is made to toggle an instruction outside the program, nothing happens.
If toggling produces an invalid instruction (like cpy 1 2) and an attempt is later made to execute that instruction, skip it instead.
If tgl toggles itself (for example, if a is 0, tgl a would target itself and become inc a), the resulting instruction is not executed until the next time it is reached.
*/

    fn toggle(self: &mut Self) {
        self.inst = match self.inst {
            InstructionType::INC => InstructionType::DEC,
            InstructionType::DEC => InstructionType::INC,
            InstructionType::JNZ => InstructionType::CPY,
            InstructionType::CPY => InstructionType::JNZ,
            InstructionType::TGL => InstructionType::INC,
            InstructionType::OUT => InstructionType::OUT,
        };
    }
}

#[derive(Debug)]
struct Machine {
    instructions: Vec<Instruction>,
    registry: HashMap<char, i128>,
    ip: i128,
    output: Vec<i128>,
    output_limit: usize,
}

fn get_name_from_arg(arg: String) -> char {
    if arg.len() != 1 {
        panic!("Invalid: {}", arg);
    }
    arg.chars().nth(0).unwrap()
}

impl Machine {
    fn new(insctructions: Vec<Instruction>, limit: usize) -> Self {
        Machine{
            instructions: insctructions,
            registry: HashMap::new(),
            ip: 0,
            output: vec![],
            output_limit: limit,
        }
    }

    fn reset(self: &mut Self) {
        self.registry = HashMap::new();
        self.output = vec![];
        self.ip = 0;
    }

    fn toggle(self: &mut Self, r: i128) {
        if r >= 0 && r < self.instructions.len() as i128 {
            self.instructions[r as usize].toggle();    
        }
    }

    fn get_from_arg(self: &mut Self, arg: Option<String>) -> i128 {
        let string = arg.unwrap();

        if let Ok(v) = string.parse::<i128>() {
            return v;
        }

        self.get(get_name_from_arg(string))
    }

    fn get(self: &mut Self, r: char) -> i128 {
        *self.registry.entry(r).or_insert(0)
    }

    fn set(self: &mut Self, r: char, v: i128) {
        self.registry.insert(r, v);
    }

    fn optimizable(self: &mut Self) -> bool {
        self.instructions[self.ip as usize].inst == InstructionType::CPY &&
            self.instructions[(self.ip+1) as usize].inst == InstructionType::INC && 
            self.instructions[(self.ip+2) as usize].inst == InstructionType::DEC && 
            self.instructions[(self.ip+3) as usize].inst == InstructionType::JNZ && 
            self.instructions[(self.ip+4) as usize].inst == InstructionType::DEC && 
            self.instructions[(self.ip+5) as usize].inst == InstructionType::JNZ
    }

    fn step(self: &mut Self) -> bool {
        if self.ip < 0 || self.ip >= self.instructions.len() as i128 {
            return false;
        }

        let _inst = self.instructions[self.ip as usize].clone();

        if !_inst.valid() {
            self.ip += 1;
            return true;
        }

        // check for optim.
        // cpy b c
        // inc a
        // dec c
        // jny c -2
        // dec d
        // jny d -5

        if self.optimizable()  {
/*
            println!("Is optim");
            println!("{:?}", self.registry);

            self.instructions[(self.ip) as usize].dump();
            self.instructions[(self.ip+1) as usize].dump();
            self.instructions[(self.ip+2) as usize].dump();
            self.instructions[(self.ip+3) as usize].dump();
            self.instructions[(self.ip+4) as usize].dump();
            self.instructions[(self.ip+5) as 7usize].dump();
*/
            let _old = self.get_from_arg(self.instructions[(self.ip+1) as usize].arg1.clone());
            let _val1 = self.get_from_arg(self.instructions[(self.ip) as usize].arg1.clone());
            let _val2 = self.get_from_arg(self.instructions[(self.ip+4) as usize].arg1.clone());

            let _dest = get_name_from_arg(self.instructions[(self.ip+1) as usize].arg1.clone().unwrap());
            self.set(_dest, _val1 * _val2 + _old);

            let _res1 = get_name_from_arg(self.instructions[self.ip as usize].arg2.clone().unwrap());
            let _res2 = get_name_from_arg(self.instructions[(self.ip+4) as usize].arg1.clone().unwrap());

            self.set(_res1, 0);
            self.set(_res2, 0);

            self.ip += 5;

            return true;
        }

        match _inst.inst {
            InstructionType::CPY => {
                let _val = self.get_from_arg(_inst.arg1.clone());
                self.set(
                    get_name_from_arg(_inst.arg2.unwrap()),
                    _val
                );
            },
            InstructionType::DEC => {
                let _val = self.get_from_arg(_inst.arg1.clone());
                self.set(
                    get_name_from_arg(_inst.arg1.unwrap()),
                    _val - 1
                );
            },
            InstructionType::INC => {
                let _val = self.get_from_arg(_inst.arg1.clone());
                self.set(
                    get_name_from_arg(_inst.arg1.unwrap()),
                    _val + 1
                );
            },
            InstructionType::JNZ => {
                let _val = self.get_from_arg(_inst.arg1.clone());
                let _offset = self.get_from_arg(_inst.arg2.clone());

                if _val != 0 && _offset != 0 {
                    self.ip += _offset;
                    return true;
                }
            },
            InstructionType::TGL => {
                let _val = self.get_from_arg(_inst.arg1.clone());

                self.toggle(self.ip + _val);
            },
            InstructionType::OUT => {
                // Add something to the output XXX
                let _val = self.get_from_arg(_inst.arg1.clone());
                self.output.push(_val);
                if self.output.len() == self.output_limit {
                    return false;
                }
            },
        };

        self.ip += 1;
        true
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();

    let mut instructions : Vec<Instruction> = vec![];

    for line in lines {
        if line == "" {
            continue;
        }

        instructions.push(Instruction::new(line.to_string()));
    }

    let expected_output = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
    let mut machine = Machine::new(instructions.clone(), expected_output.len());

    let mut init = 0;
    loop {
        machine.set('a', init);
        while machine.step() {}

        if machine.output == expected_output {
            println!("Part #0: {}", init);
            break;
        }

        machine.reset();
        init += 1;
    }
}
