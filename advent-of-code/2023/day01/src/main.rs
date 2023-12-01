use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Need a file");
    let lines : Vec<&str> = contents.split('\n').collect();
    
    let hm: HashMap<String, u32> = [
        ("one", 1), ("two", 2), ("three", 3),
        ("four", 4), ("five", 5), ("six", 6),
        ("seven", 7), ("eight", 8), ("nine", 9)
    ]
    .iter()
    .map(|&(s, i)| (s.to_string(), i))
    .collect();

    let mut ret1 = 0;
    let mut ret2 = 0;

    for line in lines {
        let digits1 : Vec<u32> = line.chars().filter(|c| c.is_numeric()).map(|c| c.to_digit(10).unwrap()).collect();
        let mut digits2 : Vec<u32> = Vec::new();
    
        let s = String::from(line);

        for n in 0..line.len() {
            if s.chars().nth(n).unwrap().is_ascii_digit() {
                let v = s.chars().nth(n).unwrap().to_digit(10).unwrap();
                digits2.push(v);
            } else {
                for (k, v) in hm.iter() {
                    if s[n..].starts_with(k) {
                        digits2.push(*v);
                    }
                }
            }
        }

        if !digits1.is_empty() {
            ret1 += 10 * digits1[0] + digits1[digits1.len() - 1];
        }
        if !digits2.is_empty() {
            ret2 += 10 * digits2[0] + digits2[digits2.len() - 1];
        }
    }

    println!("#1 {}", ret1);
    println!("#2 {}", ret2);
}
