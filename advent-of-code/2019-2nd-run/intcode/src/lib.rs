// intcode simulator
use std::fs::read_to_string;

#[derive(Debug)]
enum OpCode {
    Addition,
    Multiplication,
    InputToPosition,
    PositionToOutput,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    SetRelativeBase,
    Quit,
}

impl From<isize> for OpCode {
    fn from(s: isize) -> Self {
        match s {
            1 => OpCode::Addition,
            2 => OpCode::Multiplication,
            3 => OpCode::InputToPosition,
            4 => OpCode::PositionToOutput,
            5 => OpCode::JumpIfTrue,
            6 => OpCode::JumpIfFalse,
            7 => OpCode::LessThan,
            8 => OpCode::Equals,
            9 => OpCode::SetRelativeBase,
            99 => OpCode::Quit,
            _ => {
                println!("Unknown opcode {s}");
                todo!()
            }
        }
    }
}

pub fn str_to_opcodes(s: &str) -> Vec<isize> {
    s.split(',').filter(|x| !x.is_empty()).map(|x| x.parse::<isize>().unwrap()).collect()
}

pub fn parse(fp: &str) -> Vec<isize> {
    let contents = read_to_string(fp).unwrap();    
    str_to_opcodes(contents.trim_end())
}

fn pos_param_value(prog: &[isize], relative_base: isize, idx: usize, mode: isize, param: isize) -> isize {
    let code = prog[idx];
    let dec = 10isize.pow((param - 1) as u32);

    let val = match (mode / dec) % 10 {
        // default mode: return the content at prog[code]
        0 => prog[code as usize],
        // parameter mode: return the value
        1 => code,
        // relative mode: return content of prog[relative_base + code]
        2 => prog[(relative_base + code) as usize],
        _ => unreachable!(),
    };

    // println!("code:{code} base:{} mode:{mode} dec:{dec} - match:{} => {}", relative_base, (mode / dec) % 10, val);

    val
}

fn pos_param_addr(prog: &[isize], relative_base: isize, idx: usize, mode: isize, param: isize) -> usize {
    let code = prog[idx];
    let dec = 10isize.pow((param - 1) as u32);

    let val = match (mode / dec) % 10 {
        // default mode: return the content at prog[code]
        // parameter mode: return the value
        0 | 1 => code as usize,
        // relative mode: return content of prog[relative_base + code]
        2 => (relative_base + code) as usize,
        _ => unreachable!(),
    };

    // println!("code:{code} base:{} mode:{mode} dec:{dec} - match:{} => {}", relative_base, (mode / dec) % 10, val);

    val
}

pub struct Machine {
    opcodes: Vec<isize>,
    idx: usize,
    input: Vec<isize>,
    input_idx: usize,
    output: Vec<isize>,
    last_output: isize,
    finished: bool,
    relative_base: isize,
}

impl Machine {
    pub fn new(code: &[isize]) -> Self {
        let mut machine = Self {
            opcodes: code.to_owned(),
            idx: 0,
            input: Vec::new(),
            input_idx: 0,
            output: Vec::new(),
            finished: false,
            last_output: 0,
            relative_base: 0,
        };

        // Add some more memory to our machine. 
        machine.opcodes.resize(100000, 0);

        machine
    }

    pub fn add_input(&mut self, n: isize) {
        self.input.push(n);
    }

    pub fn clean_output(&mut self) {
        self.output = Vec::new();
    }

    pub fn get_output(&self) -> Vec<isize> {
        self.output.clone()
    }

    pub fn run(&mut self) -> isize {    
        loop {
            let opcode: OpCode = (self.opcodes[self.idx] % 100).into();
            let modes = (self.opcodes[self.idx] - (self.opcodes[self.idx] % 100)) / 100;
    
            // println!("idx:{} opcode:{opcode:?}({}) modes:{modes}", self.idx, self.opcodes[self.idx]);
    
            match opcode {
                OpCode::Addition => {
                    let idx_to = pos_param_addr(&self.opcodes, self.relative_base, self.idx + 3, modes, 3);
                    // println!("idx:{idx} opcode:{opcode:?} modes:{modes} {} + {} => idx:{idx_to}",
                    //     pos_param_value(&opcodes, idx + 1, modes, 1),
                    //     pos_param_value(&opcodes, idx + 2, modes, 2)
                    // );
            
                    self.opcodes[idx_to] = pos_param_value(&self.opcodes, self.relative_base, self.idx + 1, modes, 1) + pos_param_value(&self.opcodes, self.relative_base, self.idx + 2, modes, 2);
                    self.idx += 4;
                },
                OpCode::Multiplication => {
                    let idx_to = pos_param_addr(&self.opcodes, self.relative_base, self.idx + 3, modes, 3);
    
                    self.opcodes[idx_to] = pos_param_value(&self.opcodes, self.relative_base, self.idx + 1, modes, 1) * pos_param_value(&self.opcodes, self.relative_base, self.idx + 2, modes, 2);
                    self.idx += 4;
                },
                OpCode::InputToPosition => {
                    //let idx_to = self.opcodes[self.idx + 1] as usize;
                    let idx_to = pos_param_addr(&self.opcodes, self.relative_base, self.idx + 1, modes, 1);

                    // if there is not enough input, just return waiting for another input.
                    if self.input_idx == self.input.len() {
                        return 0;
                    }
    
                    self.opcodes[idx_to] = self.input[self.input_idx];
                    //self.opcodes[1000] = 1;
                    self.input_idx += 1;
                    self.idx += 2;
                },
                OpCode::PositionToOutput => {
                    self.output.push(
                        pos_param_value(&self.opcodes, self.relative_base, self.idx + 1, modes, 1)
                    );

                    self.last_output = self.output[self.output.len() - 1];
                    self.idx += 2;
                },
                OpCode::JumpIfTrue => {
                    let param0 = pos_param_value(&self.opcodes, self.relative_base, self.idx + 1, modes, 1);
                    let param1 = pos_param_value(&self.opcodes, self.relative_base, self.idx + 2, modes, 2);
    
                    if param0 != 0 {
                        self.idx = param1 as usize;
                    } else {
                        self.idx += 3;
                    };
                },

                OpCode::JumpIfFalse => {
                    let param0 = pos_param_value(&self.opcodes, self.relative_base, self.idx + 1, modes, 1);
                    let param1 = pos_param_value(&self.opcodes, self.relative_base, self.idx + 2, modes, 2);
    
                    if param0 == 0 {
                        self.idx = param1 as usize;
                    } else {
                        self.idx += 3;
                    };
                },
                OpCode::LessThan => {
                    let param0 = pos_param_value(&self.opcodes, self.relative_base, self.idx + 1, modes, 1);
                    let param1 = pos_param_value(&self.opcodes, self.relative_base, self.idx + 2, modes, 2);
                    let idx_to = pos_param_addr(&self.opcodes, self.relative_base, self.idx + 3, modes, 3);

                    self.opcodes[idx_to as usize] = isize::from(param0 < param1);
                    self.idx += 4;
                },
                OpCode::Equals => {
                    let param0 = pos_param_value(&self.opcodes, self.relative_base, self.idx + 1, modes, 1);
                    let param1 = pos_param_value(&self.opcodes, self.relative_base, self.idx + 2, modes, 2);
                    let idx_to = pos_param_addr(&self.opcodes, self.relative_base, self.idx + 3, modes, 3);
    
                    self.opcodes[idx_to] = isize::from(param0 == param1);
                    self.idx += 4;
                },
                OpCode::SetRelativeBase => {
                    let param0 = pos_param_value(&self.opcodes, self.relative_base, self.idx + 1, modes, 1);

                    self.relative_base += param0;
                    // self.relative_base = self.opcodes[self.idx + 1] as isize;
                    self.idx += 2;
                },
                OpCode::Quit => {
                    self.finished = true;

                    // return last output only; if there is no output, returns 0.
                    return if self.output.is_empty() {
                        // day02 requires to output the first opcode if it did not output anything else.
                        self.opcodes[0]
                    } else {
                        self.output[self.output.len() - 1]
                    };
                }
            }
        }
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn create_and_run(code: &[isize], input: &[isize]) -> isize {
        let mut machine = Self::new(code);

        input.iter().map(|x| machine.add_input(*x)).count();

        machine.run()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02_sample_00() {
        let code = parse("tests/day02/input.txt_test0");
        let mut machine = Machine::new(&code);
        assert_eq!(
            3500,
            machine.run()
        );
    }

    #[test]
    fn test_day02_sample_01() {
        let code = parse("tests/day02/input.txt_test1");
        let mut machine = Machine::new(&code);
        assert_eq!(
            2,
            machine.run()
        );
    }

    #[test]
    fn test_day02_sample_02() {
        let code = parse("tests/day02/input.txt_test2");
        let mut machine = Machine::new(&code);
        assert_eq!(
            2,
            machine.run()
        );
    }

    #[test]
    fn test_day02_sample_03() {
        let code = parse("tests/day02/input.txt_test3");
        let mut machine = Machine::new(&code);
        assert_eq!(
            2,
            machine.run()
        );
    }

    #[test]
    fn test_day02_sample_04() {
        let code = parse("tests/day02/input.txt_test4");
        let mut machine = Machine::new(&code);
        assert_eq!(
            30,
            machine.run()
        );
    }

    #[test]
    fn test_day05_sample_00() {
        let code = str_to_opcodes("3,0,4,0,99");
        assert_eq!(
            17,
            Machine::create_and_run(&code, &[17].to_vec())
        );
    }

    #[test]
    fn test_day05_sample_01() {
        let code = str_to_opcodes("1002,4,3,4,33");
        assert_eq!(
            1002,
            Machine::create_and_run(&code, &[].to_vec())
        );
    }

    #[test]
    fn test_day05_sample_02() {
        let code = str_to_opcodes("1101,100,-1,4,0");
        assert_eq!(
            1101,
            Machine::create_and_run(&code, &[].to_vec())
        );
    }

    #[test]
    fn test_day05_sample_03() {
        let code = str_to_opcodes("3,9,8,9,10,9,4,9,99,-1,8");
        assert_eq!(
            1,
            Machine::create_and_run(&code, &[8].to_vec())
        );

        assert_eq!(
            0,
            Machine::create_and_run(&code, &[9].to_vec())
        );
    }

    #[test]
    fn test_day05_sample_04() {
        let code = parse("tests/day05/input.txt_test0");
        assert_eq!(
            999,
            Machine::create_and_run(&code, &[7].to_vec())
        );

        assert_eq!(
            1000,
            Machine::create_and_run(&code, &[8].to_vec())
        );

        assert_eq!(
            1001,
            Machine::create_and_run(&code, &[9].to_vec())
        );
    }

    #[test]
    fn test_day09_sample_01() {
        let code = str_to_opcodes("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99");
        let mut machine = Machine::new(&code);
        machine.run();

        assert_eq!(
            machine.output,
            code
        );
    }

    #[test]
    fn test_day09_sample_02() {
        let code = str_to_opcodes("1102,34915192,34915192,7,4,7,99,0");
        let mut machine = Machine::new(&code);
        machine.run();

        assert_eq!(
            [1219070632396864].to_vec(),
            machine.output
        );
    }

    #[test]
    fn test_day09_sample_03() {
        let code = str_to_opcodes("104,1125899906842624,99");
        let mut machine = Machine::new(&code);
        machine.run();

        assert_eq!(
            [code[1]].to_vec(),
            machine.output
        );
    }

}
