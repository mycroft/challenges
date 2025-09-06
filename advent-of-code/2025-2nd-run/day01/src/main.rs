use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("input.txt").expect("Should have been able to read the file");

    let mut floor : i64 = 0;
    let mut idx_basement = None;

    for (idx, c) in contents.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }

        if floor == -1 && idx_basement.is_none() {
            idx_basement = Some(idx + 1); // +1 to convert from 0-based to 1-based index
        }
    }

    println!("#1 {}", floor);
    if let Some(idx) = idx_basement {
        println!("#2 {}", idx);
    } else {
        println!("Never entered the basement.");
    }
}
