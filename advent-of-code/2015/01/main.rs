use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let mut found = false;
    let mut level = 0;

    for (i, c) in contents.chars().enumerate() {
        if c == '(' {
            level += 1;
        } else if c == ')' {
            level -= 1;
        }

        if level == -1 && found == false {
            println!("Santa entered basement on iteration {}", i + 1);
            found = true
        }
    }

    println!("Final level: {}", level)
}
