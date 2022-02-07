//
// IntCode executor module
//

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IntCode {
    program: Vec<i64>,
    input: Vec<i64>,
    ip: usize,
    base: i64,
    pub output: Vec<i64>,
    debug: bool,
    is_halted: bool,
}

impl IntCode {
    pub fn new(program: &[i64], input: &[i64]) -> Self {
        let mut memory = program.to_owned();
        memory.resize(65535, 0);

        Self {
            program: memory,
            input: input.to_owned(),
            ip: 0,
            base: 0,
            output: vec![],
            debug: false,
            is_halted: false,
        }
    }

    pub fn push(&mut self, val: i64) {
        self.input.push(val);
    }

    pub fn execute(&mut self) {
        loop {
            let mut opcode = self.program[self.ip];
            let mut parameters = 0;
            let mut a = 0;
            let mut b = 0;
            let mut c = 0;

            if opcode > 99 {
                parameters = (opcode - opcode % 100) / 100;
                opcode %= 100;
            }

            // 0: position mode (value = memory[x])
            // 1: immediate mode (value = x)
            // 2: relative mode (value = base + x)

            let resolver = |parameters: i64, offset: usize| {
                let addr = self.ip + offset;
                match parameters % 10 {
                    0 => self.program[addr] as usize,
                    1 => addr,
                    2 => (self.program[addr] + self.base) as usize,
                    _ => unreachable!(),
                }
            };

            if [1, 2, 4, 5, 6, 7, 8, 9].contains(&opcode) {
                let addr = resolver(parameters, 1);
                a = self.program[addr];

                parameters = (parameters - (parameters % 10)) / 10;
            }

            if [1, 2, 5, 6, 7, 8].contains(&opcode) {
                let addr = resolver(parameters, 2);
                b = self.program[addr];

                parameters = (parameters - (parameters % 10)) / 10;
            }

            if [1, 2, 7, 8].contains(&opcode) {
                // c is destination: we pick address only.
                let addr = resolver(parameters, 3);
                c = addr;
            }

            if [3].contains(&opcode) {
                // a is destination: we pick address only.
                let addr = resolver(parameters, 1);
                a = addr as i64;
            }

            if self.debug {
                println!("ip:{} (orig:{}) opcode:{} a:{} b:{} c:{}",
                    self.ip, self.program[self.ip], opcode, a, b, c);
            }

            match opcode {
                1 => {
                    self.program[c] = a + b;
                    self.ip += 4;
                },
                2 => {
                    self.program[c] = a * b;
                    self.ip += 4;
                },
                3 => {
                    if self.input.is_empty() {
                        return;
                    }
                    self.program[a as usize] = self.input[0];
                    self.input.remove(0);
                    self.ip += 2;
                },
                4 => {
                    self.output.push(a);
                    self.ip += 2;
                },
                5 => { // jump-if-true
                    if a != 0 {
                        self.ip = b as usize;
                    } else {
                        self.ip += 3;
                    }
                },
                6 => { // jump-if-false
                    if a == 0 {
                        self.ip = b as usize;
                    } else {
                        self.ip += 3;
                    }
                },
                7 => { // less than
                    if a < b {
                        self.program[c as usize] = 1;
                    } else {
                        self.program[c as usize] = 0;
                    }
                    self.ip += 4;
                },
                8 => {
                    if a == b {
                        self.program[c as usize] = 1;
                    } else {
                        self.program[c as usize] = 0;
                    }
                    self.ip += 4;
                },
                9 => {
                    // relative base adjustement
                    self.base += a;
                    self.ip += 2;
                },
                99 => {
                    self.is_halted = true;
                    return;
                }
                _ => {
                    println!("Invalid opcode: {} (ip: {})", opcode, self.ip);
                    unreachable!()
                }
            };
        }
    }
}


pub fn str_to_prog(s: &str) -> Vec<i64> {
    s.split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

pub fn run(s: &str, input: &[i64]) -> Vec<i64> {
    let program = str_to_prog(s);

    let mut vm = IntCode::new(&program, input);
    vm.execute();

    vm.output
}

