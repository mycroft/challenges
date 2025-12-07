use std::fs::read_to_string;

fn parse(fp: &str) -> Vec<Vec<String>> {
    let contents = read_to_string(fp).expect("Failed to read file");
    let lines = contents.lines();

    let mut vec = Vec::new();

    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if vec.len() <= parts.len() {
            vec.resize(parts.len(), Vec::new());
        }

        for (i, part) in parts.iter().enumerate() {
            if vec.get(i).is_none() {
                vec.push(Vec::new());
            }
            vec[i].push(part.to_string());
        }
    }

    vec
}

fn parse_step2(fp: &str) -> Vec<Vec<String>> {
    let contents = read_to_string(fp).expect("Failed to read file");
    let lines = contents.lines();

    let mut line_len = lines.clone().next().unwrap().len();

    let mut current_number = String::new();
    let mut current_op = Vec::new();
    let mut ops = Vec::new();
    let mut current_line = 0;
    while line_len != 0 {
        let current_char = lines.clone().nth(current_line).unwrap().chars().nth(line_len - 1).unwrap();

        if current_char.is_ascii_digit() {
            current_number.push(current_char);
        }

        if (current_char == ' ' || !current_char.is_ascii_digit()) && !current_number.is_empty() {
            current_op.push(current_number.clone());
            current_number.clear();
        }

        if !current_char.is_ascii_digit() && current_char != ' ' {
            current_op.push(current_char.to_string());
            ops.push(current_op.clone());
            current_op.clear();
        }

        current_line += 1;
        if current_line >= lines.clone().count() {
            current_line = 0;
            line_len -= 1;
            continue;
        }
    }

    ops
}

fn compute(data: Vec<Vec<String>>) -> u128 {
    let mut result = 0;

    for line in data {
        // Placeholder for computation logic
        let mut line = line.clone();
        let op = line.pop().unwrap();
        let first = line.pop().unwrap().parse::<u128>().unwrap_or(0);

        result += line.into_iter().fold(first, |acc, item| {
            match op.as_str() {
                "+" => acc + item.parse::<u128>().unwrap_or(0),
                "*" => acc * item.parse::<u128>().unwrap_or(0),
                _ => acc,
            }
        });
    }
    result
}


fn main() {
    let parsed_data = parse("input.txt");
    println!("#1: {}", compute(parsed_data));

    let parsed_data_step2 = parse_step2("input.txt");
    println!("#2: {}", compute(parsed_data_step2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute() {
        let data = parse("input_test.txt");
        assert_eq!(compute(data), 4277556);
    }
}