use std::fs::read_to_string;

enum OpCode {
    Addition,
    Multiplication,
    Quit,
}

enum PatchMode {
    NoPatch,
    Patch(u32, u32)
}

impl From<u32> for OpCode {
    fn from(s: u32) -> Self {
        match s {
            1 => OpCode::Addition,
            2 => OpCode::Multiplication,
            99 => OpCode::Quit,
            _ => todo!()
        }
    }
} 

fn parse(fp: &str) -> Vec<u32> {
    let contents = read_to_string(fp).unwrap();
    let contents = contents.trim_end();
    
    contents.split(",").filter(|x| !x.is_empty()).map(|x| x.parse::<u32>().unwrap()).collect()
}

fn play(fp: &str, patch: PatchMode) -> u32 {
    let mut opcodes = parse(fp);
    let mut idx = 0;

    match patch {
        PatchMode::NoPatch => {},
        PatchMode::Patch(x, y) => {
            opcodes[1] = x;
            opcodes[2] = y;        
        }
    }

    loop {
        let opcode: OpCode = opcodes[idx].into();

        match opcode {
            OpCode::Addition => {
                let idx_op1 = opcodes[idx + 1] as usize;
                let idx_op2 = opcodes[idx + 2] as usize;
                let idx_to = opcodes[idx + 3] as usize;

                opcodes[idx_to] = opcodes[idx_op1] + opcodes[idx_op2];
                idx += 4;
            },
            OpCode::Multiplication => {
                let idx_op1 = opcodes[idx + 1] as usize;
                let idx_op2 = opcodes[idx + 2] as usize;
                let idx_to = opcodes[idx + 3] as usize;

                opcodes[idx_to] = opcodes[idx_op1] * opcodes[idx_op2];
                idx += 4;
            },
            OpCode::Quit => {
                return opcodes[0]
            }
        }
    }
}

fn main() {
    println!("#1 {}", play("input.txt", PatchMode::Patch(12, 2))); // 3706713

    for x in 0..=99 {
        for y in 0..=99 {
            let result = play("input.txt", PatchMode::Patch(x, y));
            if result == 19690720 {
                println!("#2 {}", x*100 + y);
                return;
            }
        }
    }
}

#[test]
fn test_sample() {
    assert_eq!(
        3500,
        play("input.txt_test0", PatchMode::NoPatch)
    );

    assert_eq!(
        2,
        play("input.txt_test1", PatchMode::NoPatch)
    );

    assert_eq!(
        2,
        play("input.txt_test2", PatchMode::NoPatch)
    );

    assert_eq!(
        2,
        play("input.txt_test3", PatchMode::NoPatch)
    );

    assert_eq!(
        30,
        play("input.txt_test4", PatchMode::NoPatch)
    );
}

#[test]
fn test_input() {
    assert_eq!(
        3706713,
        play("input.txt", PatchMode::Patch(12, 2))
    );
}