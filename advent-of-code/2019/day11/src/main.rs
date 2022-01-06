
struct IntCode {
    program: Vec<i64>,
    input: Vec<i64>,
    ip: usize,
    base: i64,
    output: Vec<i64>,
    debug: bool,
    is_halted: bool,
}

impl IntCode {
    fn new(program: &[i64], input: &[i64]) -> Self {
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

    fn execute(&mut self) {
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
                        // There is no input: Do nothing & return.
                        return
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

fn str_to_prog(s: &str) -> Vec<i64> {
    s.split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn run(s: &str, input: &[i64]) -> Vec<i64> {
    let program = str_to_prog(s);

    let mut vm = IntCode::new(&program, input);
    vm.execute();

    vm.output
}

fn run_last(s: &str, input: &[i64]) -> i64 {
    let output = run(s, input);

    *output.last().unwrap()
}

use std::collections::HashSet;

fn paintaroo(initial_color: i64) -> (usize, HashSet<(isize, isize)>) {
    let contents = std::fs::read_to_string("input.txt").expect("invalid file");
    let contents = contents.trim();

    let program = str_to_prog(contents);

    let mut vm = IntCode::new(&program, &[]);

    let directions = [
        (1, 0),
        (0, 1),
        (-1, 0),
        (0, -1),
    ];

    let mut paint: HashSet<(isize, isize)> = HashSet::new();

    let mut current_direction: isize = -1;
    let mut current_coords = (0isize, 0isize);
    let mut hs: HashSet<(isize, isize)> = HashSet::new();

    if initial_color == 1 {
        hs.insert(current_coords);
    }

    while !vm.is_halted {
        // set current color as input.
        let color = if hs.contains(&current_coords) {
            1
        } else {
            0
        };
        vm.input.push(color);

        // run machine
        vm.execute();

        // check output
        // first number: 0 paint to black, 1 paint to white
        // second number: 0 turn left, 1 turn right
        let paint_order = vm.output[vm.output.len() - 2];
        let dir_order = vm.output[vm.output.len() - 1];

        if paint_order == 1 {
            hs.insert(current_coords);
        } else {
            hs.remove(&current_coords);
        }

        paint.insert(current_coords);

        if dir_order == 0 {
            current_direction -= 1;
            if current_direction < 0 {
                current_direction = 3;
            }
        } else {
            current_direction += 1;
            if current_direction > 3 {
                current_direction = 0;
            }
        }

        current_coords.0 += directions[current_direction as usize].0;
        current_coords.1 += directions[current_direction as usize].1;
    }

    (paint.len(), hs)
}

fn print(hs: &HashSet<(isize, isize)>) {
    let min_x = hs.iter().map(|&(x, _)| x).min().unwrap();
    let max_x = hs.iter().map(|&(x, _)| x).max().unwrap();

    let min_y = hs.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = hs.iter().map(|&(_, y)| y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if hs.contains(&(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn main() {
    let res = paintaroo(0);
    println!("#1 {:?}", res.0); // 2428

    let res = paintaroo(1);
    print(&res.1);
    //println!("#2 {:?}", res.1);
}

#[test]
fn test_prog_day05() {
    assert_eq!(1, run_last("3,9,8,9,10,9,4,9,99,-1,8", &[8]));
    assert_eq!(0, run_last("3,9,8,9,10,9,4,9,99,-1,8", &[9]));

    assert_eq!(1, run_last(&"3,9,7,9,10,9,4,9,99,-1,8", &[7]));
    assert_eq!(0, run_last(&"3,9,7,9,10,9,4,9,99,-1,8", &[8]));

    assert_eq!(1, run_last(&"3,3,1108,-1,8,3,4,3,99", &[8]));
    assert_eq!(0, run_last(&"3,3,1108,-1,8,3,4,3,99", &[9]));

    assert_eq!(0, run_last(&"3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", &[0]));
    assert_eq!(1, run_last(&"3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9", &[1]));

    assert_eq!(0, run_last(&"3,3,1105,-1,9,1101,0,0,12,4,12,99,1", &[0]));
    assert_eq!(1, run_last(&"3,3,1105,-1,9,1101,0,0,12,4,12,99,1", &[1]));

    let s = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
    assert_eq!(999, run_last(&s, &[-1]));
    assert_eq!(1000, run_last(&s, &[8]));
    assert_eq!(1001, run_last(&s, &[9]));
}

#[test]
fn test_prog_day09() {
    let s0 = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
    let output = run(&s0, &vec![]);

    assert_eq!(
        s0.split(",").map(|x| x.parse::<i64>().unwrap()).collect::<Vec<i64>>(),
        output
    );

    assert_eq!(1219070632396864, run_last("1102,34915192,34915192,7,4,7,99,0", &[]));
    assert_eq!(1125899906842624, run_last("104,1125899906842624,99", &[]));
}
