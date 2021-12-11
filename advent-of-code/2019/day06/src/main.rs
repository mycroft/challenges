use scan_fmt::scan_fmt;

extern crate scan_fmt;

use std::collections::HashMap;

fn solve(hm: &HashMap<String, String>, x: &str) -> Vec<String> {
    let mut x = &x.to_string();
    let mut z = vec![];

    loop {
        if !hm.contains_key(x) {
            return z;
        }

        x = hm.get(x).unwrap();
        z.push(x.clone());
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut hm = HashMap::new();

    for line in lines {
        let (from, to) = scan_fmt!(
            line,
            "{}){}",
            String, String
        ).unwrap();

        hm.insert(to, from);
    }

    let mut total = 0;

    for (k, _v) in hm.iter() {
        let z = solve(&hm, k);
        total += z.len();
    }

    println!("#1 {}", total);

    let mut path1 = solve(&hm, &"YOU".to_string());
    let mut path2 = solve(&hm, &"SAN".to_string());

    loop {
        let a = path1.pop().unwrap();
        let b = path2.pop().unwrap();

        if a != b {
            break;
        }
    }

    println!("#2 {}", path1.len() + path2.len() + 2);
}
