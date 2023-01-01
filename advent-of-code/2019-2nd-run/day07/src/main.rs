use std::fs::read_to_string;
use itertools::Itertools;

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
            99 => OpCode::Quit,
            _ => {
                println!("Unknown opcode {s}");
                todo!()
            }
        }
    }
}

fn str_to_opcodes(s: &str) -> Vec<isize> {
    s.split(',').filter(|x| !x.is_empty()).map(|x| x.parse::<isize>().unwrap()).collect()
}

fn parse(fp: &str) -> Vec<isize> {
    let contents = read_to_string(fp).unwrap();    
    str_to_opcodes(contents.trim_end())
}

fn pos_param_value(prog: &[isize], idx: usize, mode: isize, param: isize) -> isize {
    let code = prog[idx];
    let dec = 10isize.pow((param - 1) as u32);
    
    if mode & dec == 0 {
        // default mode: return the content at prog[code]
        prog[code as usize]
    } else {
        // parameter mode: return content of code
        code
    }
}

struct Machine {
    opcodes: Vec<isize>,
    idx: usize,
    input: Vec<isize>,
    input_idx: usize,
    output: Vec<isize>,
    last_output: isize,
    finished: bool
}

impl Machine {
    fn new(code: &[isize]) -> Self {
        Self {
            opcodes: code.to_owned(),
            idx: 0,
            input: Vec::new(),
            input_idx: 0,
            output: Vec::new(),
            finished: false,
            last_output: 0,
        }
    }

    fn add_input(&mut self, n: isize) {
        self.input.push(n);
    }

    fn clean_output(&mut self) {
        self.output = Vec::new();
    }

    fn run(&mut self) -> isize {    
        loop {
            let opcode: OpCode = (self.opcodes[self.idx] % 100).into();
            let modes = (self.opcodes[self.idx] - (self.opcodes[self.idx] % 100)) / 100;
    
            // println!("idx:{} opcode:{opcode:?} modes:{modes}", self.idx);
    
            match opcode {
                OpCode::Addition => {
                    let idx_to = self.opcodes[self.idx + 3] as usize;
                    // println!("idx:{idx} opcode:{opcode:?} modes:{modes} {} + {} => idx:{idx_to}",
                    //     pos_param_value(&opcodes, idx + 1, modes, 1),
                    //     pos_param_value(&opcodes, idx + 2, modes, 2)
                    // );
            
                    self.opcodes[idx_to] = pos_param_value(&self.opcodes, self.idx + 1, modes, 1) + pos_param_value(&self.opcodes, self.idx + 2, modes, 2);
                    self.idx += 4;
                },
                OpCode::Multiplication => {
                    let idx_to = self.opcodes[self.idx + 3] as usize;
    
                    self.opcodes[idx_to] = pos_param_value(&self.opcodes, self.idx + 1, modes, 1) * pos_param_value(&self.opcodes, self.idx + 2, modes, 2);
                    self.idx += 4;
                },
                OpCode::InputToPosition => {
                    let idx_to = self.opcodes[self.idx + 1] as usize;

                    // if there is not enough input, just return waiting for another input.
                    if self.input_idx == self.input.len() {
                        return 0;
                    }
    
                    self.opcodes[idx_to] = self.input[self.input_idx];
                    self.input_idx += 1;
                    self.idx += 2;
                },
                OpCode::PositionToOutput => {
                    self.output.push(
                        pos_param_value(&self.opcodes, self.idx + 1, modes, 1)
                    );

                    self.last_output = self.output[self.output.len() - 1];
    
                    self.idx += 2;
                },
                OpCode::JumpIfTrue => {
                    let param0 = pos_param_value(&self.opcodes, self.idx + 1, modes, 1);
                    let param1 = pos_param_value(&self.opcodes, self.idx + 2, modes, 2);
    
                    if param0 != 0 {
                        self.idx = param1 as usize;
                    } else {
                        self.idx += 3;
                    };
                },
                OpCode::JumpIfFalse => {
                    let param0 = pos_param_value(&self.opcodes, self.idx + 1, modes, 1);
                    let param1 = pos_param_value(&self.opcodes, self.idx + 2, modes, 2);
    
                    if param0 == 0 {
                        self.idx = param1 as usize;
                    } else {
                        self.idx += 3;
                    };
                },
                OpCode::LessThan => {
                    let param0 = pos_param_value(&self.opcodes, self.idx + 1, modes, 1);
                    let param1 = pos_param_value(&self.opcodes, self.idx + 2, modes, 2);
                    let idx_to = self.opcodes[self.idx + 3] as usize;
    
                    self.opcodes[idx_to as usize] = isize::from(param0 < param1);
                    self.idx += 4;
                },
                OpCode::Equals => {
                    let param0 = pos_param_value(&self.opcodes, self.idx + 1, modes, 1);
                    let param1 = pos_param_value(&self.opcodes, self.idx + 2, modes, 2);
                    let idx_to = self.opcodes[self.idx + 3] as usize;
    
                    self.opcodes[idx_to] = isize::from(param0 == param1);
                    self.idx += 4;
                },
                OpCode::Quit => {
                    self.finished = true;

                    // return last output only; if there is no output, returns 0.
                    return if self.output.is_empty() {
                        0
                    } else {
                        self.output[self.output.len() - 1]
                    };
                }
            }
        }
    }

    fn create_and_run(code: &[isize], input: &[isize]) -> isize {
        let mut machine = Self::new(code);

        input.iter().map(|x| machine.add_input(*x)).count();

        machine.run()
    }
}

fn multiple_run(prog: &[isize], input: Vec<isize>) -> isize {
    input.iter().fold(0, |signal, x| Machine::create_and_run(prog, [*x, signal].as_ref()))
}

fn multiple_run_loop(prog: &[isize], input: Vec<isize>) -> isize {
    // we need to modify the "run" function so it can start process a program then stop when waiting some input,
    // and continue it when new input is here and ready.

    // Create all the machines with initial inputs: phase setting sequence, 

    let mut machines = input.iter().map(|x| {
        let mut m = Machine::new(prog);
        m.add_input(*x);
        m
    }).collect::<Vec<Machine>>();

    // Adding initial signal to first machine.
    machines[0].add_input(0);

    let machine_len = machines.len();

    loop {
        let mut has_code_running = false;
        for n in 0..machine_len {
            if machines[n].finished {
                continue;
            }

            machines[n].run();
            has_code_running = true;

            // if m generated some input, give it to machines[n+1%machines.len()]
            for o in machines[n].output.clone() {
                machines[(n+1)%machine_len].add_input(o);
            }

            machines[n].clean_output();
        }

        if !has_code_running {
            break;
        }
    }

    machines[machines.len() - 1].last_output
}


fn main() {
    let code = parse("input.txt");
    let result = (0..5).permutations(5).map(|x| multiple_run(&code, x)).max().unwrap();
    println!("#1 {result}"); // 366376

    let result = (5..10).permutations(5).map(|x| multiple_run_loop(&code, x)).max().unwrap();
    println!("#2 {result}"); // 21596786
}

#[test]
fn test_sample() {
    let code = str_to_opcodes("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    assert_eq!(
        43210,
        multiple_run(&code, [4, 3, 2, 1, 0].to_vec())
    );

    let code = str_to_opcodes("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
    assert_eq!(
        54321,
        multiple_run(&code, [0, 1, 2, 3, 4].to_vec())
    );

    let code = str_to_opcodes("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
    assert_eq!(
        65210,
        multiple_run(&code, [1, 0, 4, 3, 2].to_vec())
    );
}

#[test]
fn test_sample_step2() {
    let code = str_to_opcodes("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
    assert_eq!(
        139629729,
        multiple_run_loop(&code, [9, 8, 7, 6, 5].to_vec())
    );

    let code = str_to_opcodes("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
    assert_eq!(
        18216,
        multiple_run_loop(&code, [9, 7, 8, 5, 6].to_vec())
    );
}