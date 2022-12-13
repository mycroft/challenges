use std::fs::read_to_string;

#[macro_use] extern crate scan_fmt;


#[derive(Clone, Copy, Debug)]
enum OpType {
    Sum,
    Mul,
    Square
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i128>,
    op_type: OpType,
    op_val: i128,
    div_val: i128,
    div_true: usize,
    div_false: usize,
    inspection: usize
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            items: Vec::new(),
            op_type: OpType::Sum,
            op_val: 0,
            div_val: 1,
            div_true: 0,
            div_false: 0,
            inspection: 0
        }
    }

    fn empty_items(&mut self) {
        self.items = Vec::new();
    }
}

fn parse(fp: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::new();

    let contents = read_to_string(fp).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut line_idx = 0;

    let mut current_monkey = Monkey::new();
    loop {
        if line_idx >= lines.len() {
            break;
        }

        if lines[line_idx].starts_with("Monkey") {
            // Create new monkey
            current_monkey = Monkey::new();
        } else if lines[line_idx].starts_with("  Starting items:") {
            if let Ok(vals) = scan_fmt!(&lines[line_idx], "  Starting items: {[0-9, ]}", String) {
                current_monkey.items = vals.split(", ").map(|x| x.parse().unwrap()).collect();
            } else {
                panic!("Could not parse {}", lines[line_idx]);
            }

        } else if lines[line_idx].starts_with("  Operation: new = old") {
            if let Ok((op, val)) = scan_fmt!(&lines[line_idx], "  Operation: new = old {} {}", String, String) {
                current_monkey.op_type = match op.as_str() {
                    "*" => OpType::Mul,
                    "+" => OpType::Sum,
                    _ => unreachable!()
                };
                if val == "old" {
                    current_monkey.op_type = OpType::Square
                } else {
                    current_monkey.op_val = val.parse().unwrap();
                }
                
            } else {
                panic!("Could not parse {}", lines[line_idx]);
            }
        } else if lines[line_idx].starts_with("  Test: divisible by ") {
            if let Ok(val) = scan_fmt!(&lines[line_idx], "  Test: divisible by {}", i128) {
                current_monkey.div_val = val;
            } else {
                panic!("Could not parse {}", lines[line_idx]);
            }
        } else if lines[line_idx].starts_with("    If true: throw to monkey") {
            if let Ok(val) = scan_fmt!(&lines[line_idx], "    If true: throw to monkey {}", usize) {
                current_monkey.div_true = val;
            } else {
                panic!("Could not parse {}", lines[line_idx]);
            }
        } else if lines[line_idx].starts_with("    If false: throw to monkey") {
            if let Ok(val) = scan_fmt!(&lines[line_idx], "    If false: throw to monkey {}", usize) {
                current_monkey.div_false = val;
            } else {
                panic!("Could not parse {}", lines[line_idx]);
            }

        } else if lines[line_idx] == "" {
            monkeys.push(current_monkey.clone());
        }

        line_idx += 1;
    }

    // Adding last monkey.
    monkeys.push(current_monkey.clone());

    monkeys
}

fn play_rounds(monkeys: &mut Vec<Monkey>, level_div: i128, rounds: usize) -> usize {
    let mut factors = 1;

    for monkey in monkeys.iter() {
        factors *= monkey.div_val;
    }

    for _round in 0..rounds {
        // println!("Round: {round}");
        for monkey_idx in 0..monkeys.len() { 
            let monkey = monkeys[monkey_idx].clone();
            let items = monkey.items.clone();
            monkeys[monkey_idx].empty_items();

            for item in items {
                let level = match monkey.op_type {
                    OpType::Mul => item * monkey.op_val,
                    OpType::Sum => item + monkey.op_val,
                    OpType::Square => item * item,
                };

                let new_level = (level % factors) / level_div;
                // println!("{monkey_idx} {level} {new_level}");

                if new_level % monkey.div_val == 0 {
                    monkeys[monkey.div_true].items.push(new_level);
                } else {
                    monkeys[monkey.div_false].items.push(new_level);
                }

                monkeys[monkey_idx].inspection += 1;
            }
        }
    }

    // for monkey in monkeys.iter() {
    //     println!("{monkey:?}");
    // }

    let mut inspections = monkeys.iter().map(|x| x.inspection).collect::<Vec<usize>>();
    inspections.sort_unstable();
    inspections.reverse();

    // println!("{inspections:?}");
    inspections[0] * inspections[1]
}

fn main() {
    let mut monkeys = parse("input.txt");
    let monkey_business = play_rounds(&mut monkeys, 3, 20);

    let mut monkeys = parse("input.txt");
    let monkey_business2 = play_rounds(&mut monkeys, 1, 10000);

    println!("#1 {}", monkey_business);
    println!("#2 {}", monkey_business2);
}

#[test]
fn test() {
    let mut monkeys = parse("input.txt_test");
    let monkey_business = play_rounds(&mut monkeys, 3, 20);
    assert_eq!(10605, monkey_business);

    let mut monkeys = parse("input.txt_test");
    let monkey_business = play_rounds(&mut monkeys, 1, 10000);
    assert_eq!(2713310158, monkey_business);
}