use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut nums : Vec<i32> = vec![];
    let mut val = 0;

    for line in contents.lines() {
        nums.push(line.parse::<i32>().unwrap());
    }

    for num in &nums {
        val += num;
    }

    println!("Part #1: {}", val);

    let mut seen : HashSet<i32> = HashSet::new();
    let mut val = 0;
    let mut result = None;

    loop {
        for num in &nums {
            val += num;
            if seen.contains(&val) {
                result = Some(val);
                break;
            } else {
                seen.insert(val);
            }
        }

        if result != None {
            break;
        }
    }

    println!("Part #2: {}", result.unwrap());
}
