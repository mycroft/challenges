use std::fs::read_to_string;
use std::collections::HashMap;
use std::cmp::max;

#[derive(Debug, Clone)]
enum Operator {
    None,
    Plus,
    Minus,
    Mul,
    Div,
}

impl From<&str> for Operator {
    fn from(s: &str) -> Operator {
        match s {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Mul,
            "/" => Operator::Div,
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    is_final: bool,
    num: i64,
    op: Operator,
    op1: String,
    op2: String,
}

fn parse(fp: &str) -> HashMap<String, Instruction> {
    let contents = read_to_string(fp).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut result = HashMap::new();

    for line in lines {
        let parts = line.split_once(":").unwrap();
        let monkey_name = parts.0.to_string();
        let parts = line.split(" ").collect::<Vec<&str>>();


        result.insert(monkey_name, Instruction{
            is_final: parts.len() == 2,
            num: if parts.len() == 2 {
                parts[1].parse::<i64>().unwrap()
            } else {
                0
            },
            op: if parts.len() == 4 {
                parts[2].into()
            } else {
                Operator::None
            },
            op1: if parts.len() == 4 { parts[1].to_string() } else { String::new() },
            op2: if parts.len() == 4 { parts[3].to_string() } else { String::new() },
        });
    }

    result
}

fn compute(who: &String, instructions: &HashMap<String, Instruction>) -> i64 {
    let instruction = instructions.get(who).unwrap();

    if instruction.is_final {
        instruction.num
    } else {
        match instruction.op {
            Operator::None => 0,
            Operator::Plus => compute(&instruction.op1, instructions) + compute(&instruction.op2, instructions),
            Operator::Minus => compute(&instruction.op1, instructions) - compute(&instruction.op2, instructions),
            Operator::Mul => compute(&instruction.op1, instructions) * compute(&instruction.op2, instructions),
            Operator::Div => compute(&instruction.op1, instructions) / compute(&instruction.op2, instructions),
        }
    }
}

fn step2(instructions: &mut HashMap<String, Instruction>) -> i64 {
    let root_instruction = instructions.get(&"root".to_string()).unwrap();
    let mut increment = 10000000000;
    let mut i = 0;

    loop {
        let mut instructions = instructions.clone();
        let human_instruction = instructions.get_mut(&"humn".to_string()).unwrap();

        human_instruction.num = i;
        let c0 = compute(&root_instruction.op1, &instructions);
        let c1 = compute(&root_instruction.op2, &instructions);

        if c0 == c1 {
            return i;
        }

        if c0 < c1 && increment != 1 {
            i -= increment;
            increment /= 10;

            increment = max(1, increment);
            if i < 1 && increment == 1 {
                i = increment;
            }
        }

        i += increment;
    }
}

fn main() {
    let mut instructions = parse("input.txt");
    println!("#1 {}", compute(&"root".to_string(), &instructions)); // 169525884255464
    println!("#2 {}", step2(&mut instructions)); // 3247317268284
}

#[test]
fn test_sample() {
    let mut instructions = parse("input.txt_test");
    assert_eq!(
        152,
        compute(&"root".to_string(), &instructions)
    );

    assert_eq!(
        301,
        step2(&mut instructions)
    )
}