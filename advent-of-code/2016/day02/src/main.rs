use std::fs;

fn is_valid(valid_chars: &Vec<Vec<char>>, idx_i: i8, idx_j: i8) -> bool{

    idx_i >= 0 && idx_i <= 4 && idx_j >= 0 && idx_j <= 4 && valid_chars[idx_j as usize][idx_i as usize] != '0'
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut code = String::from("");

    let mut idx_i = 1;
    let mut idx_j = 1;

    for line in contents.lines() {
        for c in line.chars() {
            match c {
                'U' => { idx_j -= 1; },
                'D' => { idx_j += 1; },
                'L' => { idx_i -= 1; },
                'R' => { idx_i += 1; },
                _ => {
                    unreachable!()
                }
            };

            if idx_i < 0 {
                idx_i = 0;
            }
            if idx_i > 2 {
                idx_i = 2;
            }
            if idx_j < 0 {
                idx_j = 0;
            }
            if idx_j > 2 {
                idx_j = 2;
            }
        }
        // println!("{} {} {}", idx_i, idx_j, 3 * idx_j + idx_i + 1);
        let code_part = 3 * idx_j + idx_i + 1;
        code.push_str(&code_part.to_string());
    }

    println!("Part #1: {}", code);

    let valid = vec![
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '2', '3', '4', '0'],
        vec!['5', '6', '7', '8', '9'],
        vec!['0', 'A', 'B', 'C', '0'],
        vec!['0', '0', 'D', '0', '0'],
    ];

    let mut idx_i : i8 = 1;
    let mut idx_j : i8 = 1;
    let mut code = String::new();

    for line in contents.lines() {
        for c in line.chars() {
            match c {
                'U' => { if is_valid(&valid, idx_i, idx_j - 1) { idx_j -= 1; } },
                'D' => { if is_valid(&valid, idx_i, idx_j + 1) { idx_j += 1; } },
                'L' => { if is_valid(&valid, idx_i - 1, idx_j) { idx_i -= 1; } },
                'R' => { if is_valid(&valid, idx_i + 1, idx_j) { idx_i += 1; } },
                _ => {
                    unreachable!()
                }
            };
        }

        // println!("i:{} j:{} c:{}", idx_i, idx_j, valid[idx_j as usize][idx_i as usize]);
        code.push(valid[idx_j as usize][idx_i as usize]);
    }

    println!("Part #2: {}", code);
}
