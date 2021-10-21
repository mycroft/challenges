use std::fs;
use std::collections::HashSet;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let lines = content.lines().collect::<Vec<&str>>();
    let mut score = 0;
    let mut final_score = 0;
    let mut frequencies : HashSet<i64> = HashSet::new();
    let mut repeated : Option<i64> = None;
    let mut first_iteration_done = false;

    loop {
        for item in &lines {
            let value : i64 = item.parse().unwrap();
            score += value;

            if frequencies.contains(&score) && repeated == None {
                repeated = Some(score);
            }
            
            frequencies.insert(score);
        }

        if !first_iteration_done {
            first_iteration_done = true;
            final_score = score;
        }

        if repeated != None {
            break;
        }
    }

    println!("#1: {}", final_score);
    println!("#2: {}", repeated.unwrap());    
}
