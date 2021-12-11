fn run(program: &Vec<i64>, input: i64) -> i64 {
    let mut ip = 0;
    let mut program = program.clone();
    let mut output = 0;

    loop {
        let mut opcode = program[ip];
        let mut parameters = 0;
        let mut a = 0;
        let mut b = 0;

        if opcode > 99 {
            parameters = (opcode - opcode % 100) / 100;
            opcode %= 100;
        }

        if [1, 2, 5, 6, 7, 8].contains(&opcode) {
            if parameters % 10 == 0 {
                a = program[program[ip + 1] as usize];
            } else {
                a = program[ip + 1];
            }
            parameters = (parameters - (parameters % 10)) / 10;
            if parameters % 10 == 0 {
                b = program[program[ip + 2] as usize];
            } else {
                b = program[ip + 2];
            }
        }

        if [4].contains(&opcode) {
            if parameters % 10 == 0 {
                a = program[program[ip + 1] as usize];
            } else {
                a = program[ip + 1];
            }
        }

        match opcode {
            1 => {
                let dest = program[ip + 3] as usize;
                program[dest] = a + b;
                ip += 4;
            },
            2 => {
                let dest = program[ip + 3] as usize;
                program[dest] = a * b;
                ip += 4;
            },
            3 => {
                let dest = program[ip + 1] as usize;
                program[dest] = input;
                ip += 2;
            },
            4 => {
                output = a;
                ip += 2;
            },
            5 => { // jump-if-true
                if a != 0 {
                    ip = b as usize;
                } else {
                    ip += 3;
                }
            },
            6 => { // jump-if-false
                if a == 0 {
                    ip = b as usize;
                } else {
                    ip += 3;
                }
            },
            7 => { // less than
                let dest = program[ip + 3] as usize;
                if a < b {
                    program[dest] = 1;
                } else {
                    program[dest] = 0;
                }
                ip += 4;
            },
            8 => {
                let dest = program[ip + 3] as usize;
                if a == b {
                    program[dest] = 1;
                } else {
                    program[dest] = 0;
                }
                ip += 4;
            },
            99 => {
                return output;
            }
            _ => {
                println!("Invalid opcode: {} (ip: {})", opcode, ip);
                unreachable!()
            }
        };
    }
}

fn str_to_prog(s: &str) -> Vec<i64> {
    s.split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("invalid file");
    let contents = contents.trim();

    let res = run(&str_to_prog(contents), 1);
    println!("#1: {}", res);

    let res = run(&str_to_prog(contents), 5);
    println!("#2: {}", res);
}

#[test]
fn prog1() {
    assert_eq!(1, run(&str_to_prog("3,9,8,9,10,9,4,9,99,-1,8"), 8));
    assert_eq!(0, run(&str_to_prog("3,9,8,9,10,9,4,9,99,-1,8"), 9));

    assert_eq!(1, run(&str_to_prog("3,9,7,9,10,9,4,9,99,-1,8"), 7));
    assert_eq!(0, run(&str_to_prog("3,9,7,9,10,9,4,9,99,-1,8"), 8));

    assert_eq!(1, run(&str_to_prog("3,3,1108,-1,8,3,4,3,99"), 8));
    assert_eq!(0, run(&str_to_prog("3,3,1108,-1,8,3,4,3,99"), 9));

    assert_eq!(0, run(&str_to_prog("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"), 0));
    assert_eq!(1, run(&str_to_prog("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9"), 1));

    assert_eq!(0, run(&str_to_prog("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"), 0));
    assert_eq!(1, run(&str_to_prog("3,3,1105,-1,9,1101,0,0,12,4,12,99,1"), 1));
 
    let s = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

    assert_eq!(999, run(&str_to_prog(s), -1));
    assert_eq!(1000, run(&str_to_prog(s), 8));
    assert_eq!(1001, run(&str_to_prog(s), 9));
}
