#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];

    for input in inputs {
        match input {
            CalculatorInput::Value(x) => stack.push(*x),
            op => {
                if stack.len() < 2 {
                    return None;
                }

                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();

                let res = match op {
                    CalculatorInput::Add => l + r,
                    CalculatorInput::Subtract => l - r,
                    CalculatorInput::Multiply => l * r,
                    CalculatorInput::Divide => l / r,
                    _ => return None
                };

                stack.push(res);

                // println!("{} {:?} {}", l, op, r);
            }
        }
    }

    if stack.len() != 1 {
        None
    } else {
        Some(stack[0])
    }
}
