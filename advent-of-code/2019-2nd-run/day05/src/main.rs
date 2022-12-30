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
    s.split(",").filter(|x| !x.is_empty()).map(|x| x.parse::<isize>().unwrap()).collect()
}

fn parse(fp: &str) -> Vec<isize> {
    let contents = read_to_string(fp).unwrap();    
    str_to_opcodes(contents.trim_end())
}

fn pos_param_value(prog: &Vec<isize>, idx: usize, mode: isize, param: isize) -> isize {
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

fn play(prog: &Vec<isize>, input: Vec<isize>) -> isize {
    let mut opcodes = prog.clone();
    let mut idx = 0;
    let mut input_idx = 0;
    let mut output = None;

    loop {
        let opcode: OpCode = (opcodes[idx] % 100).into();
        let modes = (opcodes[idx] - (opcodes[idx] % 100)) / 100;

        // println!("idx:{idx} opcode:{opcode:?} modes:{modes}");

        match opcode {
            OpCode::Addition => {
                let idx_to = opcodes[idx + 3] as usize;
                // println!("idx:{idx} opcode:{opcode:?} modes:{modes} {} + {} => idx:{idx_to}",
                //     pos_param_value(&opcodes, idx + 1, modes, 1),
                //     pos_param_value(&opcodes, idx + 2, modes, 2)
                // );
        
                opcodes[idx_to] = pos_param_value(&opcodes, idx + 1, modes, 1) + pos_param_value(&opcodes, idx + 2, modes, 2);
                idx += 4;
            },
            OpCode::Multiplication => {
                let idx_to = opcodes[idx + 3] as usize;

                opcodes[idx_to] = pos_param_value(&opcodes, idx + 1, modes, 1) * pos_param_value(&opcodes, idx + 2, modes, 2);
                idx += 4;
            },
            OpCode::InputToPosition => {
                let idx_to = opcodes[idx + 1] as usize;

                opcodes[idx_to] = input[input_idx];
                input_idx += 1;
                idx += 2;
            },
            OpCode::PositionToOutput => {
                output = Some(pos_param_value(&opcodes, idx + 1, modes, 1));

                idx += 2;
            },
            OpCode::JumpIfTrue => {
                let param0 = pos_param_value(&opcodes, idx + 1, modes, 1);
                let param1 = pos_param_value(&opcodes, idx + 2, modes, 2);

                if param0 != 0 {
                    idx = param1 as usize;
                } else {
                    idx += 3;
                };
            },
            OpCode::JumpIfFalse => {
                let param0 = pos_param_value(&opcodes, idx + 1, modes, 1);
                let param1 = pos_param_value(&opcodes, idx + 2, modes, 2);

                if param0 == 0 {
                    idx = param1 as usize;
                } else {
                    idx += 3;
                };
            },
            OpCode::LessThan => {
                let param0 = pos_param_value(&opcodes, idx + 1, modes, 1);
                let param1 = pos_param_value(&opcodes, idx + 2, modes, 2);
                let idx_to = opcodes[idx + 3] as usize;

                opcodes[idx_to as usize] = if param0 < param1 {
                    1
                } else {
                    0
                };
                idx += 4;
            },
            OpCode::Equals => {
                let param0 = pos_param_value(&opcodes, idx + 1, modes, 1);
                let param1 = pos_param_value(&opcodes, idx + 2, modes, 2);
                let idx_to = opcodes[idx + 3] as usize;

                opcodes[idx_to] = if param0 == param1 {
                    1
                } else {
                    0
                };
                idx += 4;
            },
            OpCode::Quit => {
                return output.unwrap_or(opcodes[0]);
            }
        }
    }
}

fn main() {
    let code = parse("input.txt");
    println!("#1 {}", play(&code, [1].to_vec())); // 7265618
    println!("#2 {}", play(&code, [5].to_vec())); // 7731427
}

#[test]
fn test_sample() {
    let code = str_to_opcodes("3,0,4,0,99");
    assert_eq!(
        17,
        play(&code, [17].to_vec()),
    );

    let code = str_to_opcodes("1002,4,3,4,33");
    assert_eq!(
        1002,
        play(&code, [].to_vec()),
    );

    let code = str_to_opcodes("1101,100,-1,4,0");
    assert_eq!(
        1101,
        play(&code, [].to_vec()),
    );
}

#[test]
fn test_sample_step2_0() {
    let code = str_to_opcodes("3,9,8,9,10,9,4,9,99,-1,8");
    assert_eq!(
        1,
        play(&code, [8].to_vec()),
    );

    assert_eq!(
        0,
        play(&code, [7].to_vec()),
    );

}

#[test]
fn test_sample_step2_1() {
    let code = str_to_opcodes("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
    assert_eq!(
        999,
        play(&code, [7].to_vec()),
    );

    assert_eq!(
        1000,
        play(&code, [8].to_vec()),
    );

    assert_eq!(
        1001,
        play(&code, [9].to_vec()),
    );
}