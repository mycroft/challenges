mod read;
use read::read_input;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct VM {
    registers: [isize; 3],
    ip: isize,
    program: Vec<Instruction>,
    code: Vec<isize>,
    output: Vec<isize>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
    Operand(isize),
    Invalid,
}

impl Instruction {
    fn from_int(i: isize) -> Self {
        match i {
            0 => Instruction::Adv,
            1 => Instruction::Bxl,
            2 => Instruction::Bst,
            3 => Instruction::Jnz,
            4 => Instruction::Bxc,
            5 => Instruction::Out,
            6 => Instruction::Bdv,
            7 => Instruction::Cdv,
            _ => Instruction::Invalid,
        }
    }
}

impl VM {
    fn exec(&mut self) -> Vec<isize>{
        loop {
            let result = self.run_inst();
            if !result {
                break;
            }
        }

        self.output.clone()
    }

    fn run_inst(&mut self) -> bool {
        if self.ip < 0 || self.ip >= self.program.len() as isize {
            return false;
        }

        match self.program[self.ip as usize] {
            Instruction::Adv => self.exec_adv(),
            Instruction::Bxl => self.exec_bxl(),
            Instruction::Bst => self.exec_bst(),
            Instruction::Jnz => self.exec_jnz(),
            Instruction::Bxc => self.exec_bxc(),
            Instruction::Out => self.exec_out(),
            Instruction::Bdv => self.exec_bdv(),
            Instruction::Cdv => self.exec_cdv(),
            Instruction::Invalid | Instruction::Operand(_) => panic!("Invalid instruction"),
        }

        true
    }

    fn combo_value(&self, operand: Instruction) -> isize {
        if let Instruction::Operand(operand) = operand {
            match operand {
                0..=3 => operand,
                4 => self.registers[0],
                5 => self.registers[1],
                6 => self.registers[2],
                _ => panic!("Invalid operand"),
            }
        } else {
            0
        }
    }

    fn literal_value(&self, operand: Instruction) -> isize {
        if let Instruction::Operand(operand) = operand {
            operand
        } else {
            0
        }
    }

    fn exec_adv(&mut self) {
        let operand = self.combo_value(self.program[self.ip as usize + 1]);
        self.ip += 2;

        let denominator = 2_i32.pow(operand as u32);
        self.registers[0] /= denominator as isize;
    }

    fn exec_bxl(&mut self) {
        let operand = self.literal_value(self.program[self.ip as usize + 1]);
        self.ip += 2;

        self.registers[1] ^= operand;
    }

    fn exec_bst(&mut self) {
        let operand = self.combo_value(self.program[self.ip as usize + 1]);
        self.ip += 2;

        self.registers[1] = operand % 8;
    }

    fn exec_jnz(&mut self) {
        let operand = self.literal_value(self.program[self.ip as usize + 1]);

        if self.registers[0] == 0 {
            self.ip += 2;
            return;
        }

        self.ip = operand;
    }

    fn exec_bxc(&mut self) {
        self.ip += 2;

        self.registers[1] ^= self.registers[2];
    }

    fn exec_out(&mut self) {
        let operand = self.combo_value(self.program[self.ip as usize + 1]);
        self.ip += 2;

        self.output.push(operand % 8);
    }

    fn exec_bdv(&mut self) {
        let operand = self.combo_value(self.program[self.ip as usize + 1]);
        self.ip += 2;

        let denominator = 2_i32.pow(operand as u32);
        self.registers[1] = self.registers[0] / denominator as isize;
    }

    fn exec_cdv(&mut self) {
        let operand = self.combo_value(self.program[self.ip as usize + 1]);
        self.ip += 2;

        let denominator = 2_i32.pow(operand as u32);
        self.registers[2] = self.registers[0] / denominator as isize;
    }
}

fn solve_step2(input: &VM) -> isize {
    let idx = input.program.len() - 1;
    let mut valid: Vec<isize> = vec![0];

    for length in 0..input.program.len() {
        let oldvalid = valid.clone();
        valid = Vec::new();
        for num in oldvalid {
            for offset in 0..8 {
                let newnum = 8 * num + offset;
                let mut vm = input.clone();

                vm.registers[0] = newnum;
                let output = vm.exec();

                if check(&output, &input.code[idx - length..]) {
                    valid.push(newnum);
                }
            }
        }
    }

    *valid.iter().min().unwrap()
}

fn check(output: &[isize], code: &[isize]) -> bool {
    let mut idx = output.len() - 1;

    for inst in code.iter().rev() {
        if *inst != output[idx]{
            return false;
        }
        if idx == 0 {
            break;
        }
        idx -= 1;
    }

    true
}

fn main() {
    let mut vm = read_input("input.txt");
    vm.exec();
    println!("#1: {}", vm.output.iter().map(|c| c.to_string()).join(","));

    let vm = read_input("input.txt");
    let result_step2 = solve_step2(&vm);
    println!("#2: {}", result_step2);
}

#[test]
fn samples() {
    let mut vm = VM { registers: [0, 0, 9], ip: 0, program: vec![Instruction::Bst, Instruction::Operand(6)], output: vec![] };
    vm.exec();
    assert_eq!(vm.registers, [0, 1, 9]);

    let mut vm = VM { registers: [10, 0, 0], ip: 0, program: vec![Instruction::Out, Instruction::Operand(0), Instruction::Out, Instruction::Operand(1), Instruction::Out, Instruction::Operand(4)], output: vec![] };
    vm.exec();
    assert_eq!(vm.output, [0, 1, 2]);

    let mut vm = VM { registers: [2024, 0, 0], ip: 0, program: vec![
        Instruction::Adv,
        Instruction::Operand(1),
        Instruction::Out,
        Instruction::Operand(4),
        Instruction::Jnz,
        Instruction::Operand(0),
    ], output: vec![] };
    vm.exec();
    assert_eq!(vm.output, [4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
    assert_eq!(vm.registers[0], 0);

    let mut vm = VM { registers: [0, 29, 0], ip: 0, program: vec![Instruction::Bxl, Instruction::Operand(7)], output: vec![] };
    vm.exec();
    assert_eq!(vm.registers[1], 26);

    let mut vm = VM { registers: [0, 2024, 43690], ip: 0, program: vec![Instruction::Bxc, Instruction::Operand(0)], output: vec![] };
    vm.exec();
    assert_eq!(vm.registers[1], 44354);
}

