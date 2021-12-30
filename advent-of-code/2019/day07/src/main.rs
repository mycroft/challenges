use itertools::Itertools;

struct IntCode {
    program: Vec<i64>,
    input: Vec<i64>,
    ip: usize,
    output: i64,
    is_halted: bool,
}

impl IntCode {
    fn new(program: &Vec<i64>, input: &Vec<i64>) -> Self {
        Self {
            program: program.clone(),
            input: input.clone(),
            ip: 0,
            output: 0,
            is_halted: false,
        }
    }

    fn next(&mut self) -> Option<i64> {
        loop {
            let mut opcode = self.program[self.ip];
            let mut parameters = 0;
            let mut a = 0;
            let mut b = 0;
    
            if opcode > 99 {
                parameters = (opcode - opcode % 100) / 100;
                opcode %= 100;
            }
    
            if [1, 2, 5, 6, 7, 8].contains(&opcode) {
                if parameters % 10 == 0 {
                    a = self.program[self.program[self.ip + 1] as usize];
                } else {
                    a = self.program[self.ip + 1];
                }
                parameters = (parameters - (parameters % 10)) / 10;
                if parameters % 10 == 0 {
                    b = self.program[self.program[self.ip + 2] as usize];
                } else {
                    b = self.program[self.ip + 2];
                }
            }
    
            if [4].contains(&opcode) {
                if parameters % 10 == 0 {
                    a = self.program[self.program[self.ip + 1] as usize];
                } else {
                    a = self.program[self.ip + 1];
                }
            }
    
            match opcode {
                1 => {
                    let dest = self.program[self.ip + 3] as usize;
                    self.program[dest] = a + b;
                    self.ip += 4;
                },
                2 => {
                    let dest = self.program[self.ip + 3] as usize;
                    self.program[dest] = a * b;
                    self.ip += 4;
                },
                3 => {
                    let dest = self.program[self.ip + 1] as usize;
                    if self.input.len() == 0 {
                        return None;
                    }
                    self.program[dest] = self.input[0];
                    self.input.remove(0);
                    self.ip += 2;
                },
                4 => {
                    self.output = a;
                    self.ip += 2;

                    return Some(a);
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
                    let dest = self.program[self.ip + 3] as usize;
                    if a < b {
                        self.program[dest] = 1;
                    } else {
                        self.program[dest] = 0;
                    }
                    self.ip += 4;
                },
                8 => {
                    let dest = self.program[self.ip + 3] as usize;
                    if a == b {
                        self.program[dest] = 1;
                    } else {
                        self.program[dest] = 0;
                    }
                    self.ip += 4;
                },
                99 => {
                    self.is_halted = true;
                    return Some(self.output);
                }
                _ => {
                    println!("Invalid opcode: {} (ip: {})", opcode, self.ip);
                    unreachable!()
                }
            };
        }
    }
}

fn run(program: &Vec<i64>, input: &Vec<i64>) -> i64 {
    let mut vm = IntCode::new(program, input);
    vm.next().unwrap()
}

fn str_to_prog(s: &str) -> Vec<i64> {
    s.split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn run_ampli(program: &Vec<i64>, first_input: i64, settings: &Vec<i64>) -> i64 {
    let mut current_input = first_input;

    for setting in settings {
        current_input = run(program, &vec![*setting, current_input]);
    }

    current_input
}

fn run_ampli2(program: &Vec<i64>, first_input: i64, settings: &Vec<i64>) -> i64 {
    let mut machines = vec![];

    for setting in settings {
        machines.push(IntCode::new(program, &vec![*setting]));
    };

    machines[0].input.push(first_input);

    let mut index = 0;

    loop {
        let ret = machines[index].next();

        if let Some(val) = ret {
            machines[(index+1)%5].input.push(val);
        }

        if machines[4].is_halted {
            break machines[4].output;
        }

        index += 1;

        if index == 5 {
            index = 0;
        }
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("invalid file");
    let contents = contents.trim();

    let mut p0max = 0;

    for el in (0..5).collect::<Vec<i64>>().iter().map(|&x| x).permutations(5).unique() {
        let val = run_ampli(&str_to_prog(contents), 0, &el);
        if val > p0max {
            p0max = val
        }
    }

    println!("#1 {}", p0max);


    let mut p1max = 0;

    for el in (5..10).collect::<Vec<i64>>().iter().map(|&x| x).permutations(5).unique() {
        let val = run_ampli2(&str_to_prog(contents), 0, &el);
        if val > p1max {
            p1max = val
        }
    }

    println!("#2 {}", p1max);
}

#[test]
fn prog1() {
    assert_eq!(43210,
        run_ampli(&str_to_prog("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"), 0, &vec![4, 3, 2, 1, 0])
    );

    assert_eq!(
        54321,
        run_ampli(&str_to_prog("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"), 0, &vec![0,1,2,3,4])
    );

    assert_eq!(
        65210,
        run_ampli(&str_to_prog("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"), 0, &vec![1,0,4,3,2])
    );
}

#[test]
fn prog2() {
    assert_eq!(
        139629729,
        run_ampli2(
            &str_to_prog("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"),
            0,
            &vec![9,8,7,6,5]
        )
    );

    assert_eq!(
        18216,
        run_ampli2(
            &str_to_prog("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"),
            0,
            &vec![9,7,8,5,6]
        )
    );
}