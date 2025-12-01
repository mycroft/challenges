use std::fs::read_to_string;

fn solve(fp: &str) -> (i32, i32) {
    let contents = read_to_string(fp)
        .expect("Something went wrong reading the file");

    let mut position = 50;
    let mut step1 = 0;
    let mut step2: i32 = 0;

    let lines = contents.lines();
    for line in lines {
        let direction = line.chars().next().unwrap();
        let mut value: i32 = line[1..].parse().unwrap();

        step2 += value / 100;
        value %= 100;


        match direction {
            'L' => {
                if position != 0 && value > position {
                    step2 += 1;
                }
                position -= value;
            },
            'R' => {
                if value + position > 100{
                    step2 += 1;
                }
                position += value;
            },
            _ => panic!("Unknown direction"),
        }

        position = position.rem_euclid(100);

        if position == 0 {
            step1 += 1;
            step2 += 1;
        }
    }

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
    fn test_solve() {
        let (step1, step2) = solve("input_test.txt");
        assert_eq!(step1, 3);
        assert_eq!(step2, 6);
    }

    #[test]
    fn test_solve0() {
        let (step1, step2) = solve("input_test0.txt");
        assert_eq!(step1, 0);
        assert_eq!(step2, 10);
    }
}