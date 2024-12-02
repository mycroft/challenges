
fn read_input(fp: &str) -> Vec<Vec<i32>> {
    let contents: String = std::fs::read_to_string(fp).expect("a file to open");

    let mut result = Vec::new();

    for line in contents.lines() {
        let numbers = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        result.push(numbers);
    }

    result
}

fn check(line: &Vec<i32>) -> bool {
    let is_growing = line[1] > line[0];

    for i in 0..line.len() - 1 {
        let diff = line[i + 1] - line[i];

        if is_growing && diff < 0 {
            return false;
        } else if !is_growing && diff > 0 {
            return false;
        }

        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
    }

    true
}

fn get_subset(line: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();

    for i in 0..line.len() {
        let mut subset = line.clone();
        subset.remove(i);
        result.push(subset);
    }

    result
}

fn solve_step1(input: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for line in input {
        if check(line) {
            result += 1;
        }
    }

    result
}

fn solve_step2(input: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for line in input {
        if check(line) {
            result += 1;
            continue;
        }

        for subsets in get_subset(line) {
            if check(&subsets) {
                result += 1;
                break;
            }
        }
    }

    result
}


fn main() {
    let input = read_input("input.txt");
    let result_step1 = solve_step1(&input);
    let result_step2 = solve_step2(&input);

    println!("#1 {:?}", result_step1);
    println!("#2 {:?}", result_step2);
}
