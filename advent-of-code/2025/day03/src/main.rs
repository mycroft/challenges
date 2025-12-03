use std::fs::read_to_string;

fn find_highest_joltage(batteries: &[i128], size: usize) -> i128 {
    let mut remaining = size;
    let mut result = 0;

    let mut batteries = batteries.to_vec();

    while remaining > 0 {
        result *= 10;
        remaining -= 1;
        let max_value = *batteries.iter().take(batteries.len() - remaining).max().unwrap();
        let max_index = batteries.iter().position(|&c| c == max_value).unwrap();

        // remove the first max_index elements from batteries
        batteries.drain(0..=max_index);

        result += max_value;        
    }

    result
}

fn solve(fp: &str) -> (i128, i128) {
    let contents = read_to_string(fp)
        .expect("Something went wrong reading the file");

    let step1 = contents.lines().fold(0, |acc, line| {
        let batteries = line.chars().map(|c| c.to_digit(10).unwrap() as i128).collect::<Vec<i128>>();

        acc + find_highest_joltage(&batteries, 2)
    });

    let step2 = contents.lines().fold(0, |acc, line| {
        let batteries = line.chars().map(|c| c.to_digit(10).unwrap() as i128).collect::<Vec<i128>>();

        acc + find_highest_joltage(&batteries, 12)
    });

    (step1, step2)
}

fn main() {
    let (step1, step2) = solve("input.txt");
    println!("#1: {}", step1);
    println!("#2: {}", step2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_highest_joltage() {
        let batteries = vec![1, 2, 3, 4, 5];
        assert_eq!(find_highest_joltage(&batteries, 2), 45);
    }

    #[test]
    fn test_solve() {
        let (step1, step2) = solve("input_test.txt");
        assert_eq!(step1, 357);
        assert_eq!(step2, 3121910778619);
    }
}