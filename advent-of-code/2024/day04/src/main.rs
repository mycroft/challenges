use std::fs;

fn read_input() -> Vec<Vec<char>> {
    fs::read_to_string("input.txt").expect("Error reading the file")
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn count_pattern(input: &Vec<Vec<char>>, pattern: &str) -> usize {
    let mut count = 0;
    
    let pattern: Vec<char> = pattern.chars().collect();

    // horizontal
    for line_index in 0..input.len() {
        for column_index in 0..=input[line_index].len() - pattern.len() {
            let pattern_line_index = line_index;
            let mut pattern_column_index = column_index;
            let mut pattern_index = 0;

            while input[pattern_line_index][pattern_column_index] == pattern[pattern_index] {
                if pattern_index == pattern.len() - 1 {
                    count += 1;
                    break;
                }

                pattern_column_index += 1;
                pattern_index += 1;

                if pattern_column_index > input[line_index].len() {
                    break;
                }

                if pattern_line_index > input.len() {
                    break;
                }
            }
        }
    }

    // vertical
    for line_index in 0..=input.len() - pattern.len() {
        for column_index in 0..input[line_index].len() {
            let mut pattern_line_index = line_index;
            let pattern_column_index = column_index;
            let mut pattern_index = 0;

            while input[pattern_line_index][pattern_column_index] == pattern[pattern_index] {
                if pattern_index == pattern.len() - 1 {
                    count += 1;
                    break;
                }

                pattern_line_index += 1;
                pattern_index += 1;

                if pattern_column_index > input[line_index].len() {
                    break;
                }

                if pattern_line_index > input.len() {
                    break;
                }
            }

        }
    }

    // diagonal
    for line_index in 0..=input.len() - pattern.len() {
        for column_index in 0..=input[line_index].len() - pattern.len() {
            let mut pattern_line_index = line_index;
            let mut pattern_column_index = column_index;
            let mut pattern_index = 0;

            while input[pattern_line_index][pattern_column_index] == pattern[pattern_index] {
                if pattern_index == pattern.len() - 1 {
                    count += 1;
                    break;
                }

                pattern_line_index += 1;
                pattern_column_index += 1;
                pattern_index += 1;

                if pattern_column_index > input[line_index].len() {
                    break;
                }

                if pattern_line_index > input.len() {
                    break;
                }
            }
        }
    }

    // reverse diagonal
    for line_index in 0..=input.len() - pattern.len() {
        for column_index in (pattern.len() - 1)..input[line_index].len() {
            let mut pattern_line_index = line_index;
            let mut pattern_column_index = column_index;
            let mut pattern_index: usize = 0;

            while input[pattern_line_index][pattern_column_index] == pattern[pattern_index] {
                if pattern_index == pattern.len() - 1 {
                    count += 1;
                    break;
                }

                pattern_line_index += 1;
                pattern_column_index -= 1;
                pattern_index += 1;

                if pattern_column_index >= input.len() {
                    break;
                }

                if pattern_line_index >= input[line_index].len() {
                    break;
                }
            }
        }
    }

    count
}

fn count_x_mas(input: &Vec<Vec<char>>) -> usize {
    let mut count = 0;
    for line_index in 1..input.len() - 1 {
        for column_index in 1..input[line_index].len() - 1 {
            if input[line_index][column_index] != 'A' {
                continue;
            }

            let mut count_m = 0;
            let mut count_s = 0;

            if input[line_index - 1][column_index - 1] == 'M' {
                count_m += 1;
            }
            if input[line_index - 1][column_index + 1] == 'M' {
                count_m += 1;
            }
            if input[line_index + 1][column_index - 1] == 'M' {
                count_m += 1;
            }
            if input[line_index + 1][column_index + 1] == 'M' {
                count_m += 1;
            }

            if input[line_index - 1][column_index - 1] == 'S' {
                count_s += 1;
            }
            if input[line_index - 1][column_index + 1] == 'S' {
                count_s += 1;
            }
            if input[line_index + 1][column_index - 1] == 'S' {
                count_s += 1;
            }
            if input[line_index + 1][column_index + 1] == 'S' {
                count_s += 1;
            }

            if count_m != 2 || count_s != 2 {
                continue;
            }

            if input[line_index - 1][column_index - 1] == input[line_index + 1][column_index + 1] {
                continue;
            }

            count += 1;
        }
    }

    count
}

fn main() {
    let input = read_input();

    let mut result_step1 = count_pattern(&input, "XMAS");
    result_step1 += count_pattern(&input, "SAMX");

    let result_step2 = count_x_mas(&input);

    println!("#1: {}", result_step1); // 2536
    println!("#2: {}", result_step2); // 1875
}
