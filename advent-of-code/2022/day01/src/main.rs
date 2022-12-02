use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Need a file");
    let lines : Vec<&str> = contents.split("\n").collect();

    let mut current_cal = 0;
    let mut all_cals : Vec<i32> = Vec::new();

    for line in lines {
        if line.is_empty() {
            all_cals.push(current_cal);
            current_cal = 0;
        } else {
            current_cal += line.parse::<i32>().expect("need a mumber");
        }
    }

    all_cals.push(current_cal);

    all_cals.sort();
    all_cals.reverse();

    println!("#1: {}", all_cals[0]);
    println!("#2: {}", all_cals.iter().take(3).sum::<i32>());
}
