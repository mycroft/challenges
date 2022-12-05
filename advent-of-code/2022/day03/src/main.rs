use std::fs;
use std::collections::HashSet;

fn badge_to_number(b: char) -> u32 {
    match b {
        'a'..='z' => {
            b as u32 - 'a' as u32 + 1
        },
        'A'..='Z' => {
            b as u32 - 'A' as u32 + 1 + 26
        },
        _ => { panic!("invalid") },
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut score_part1 = 0;
    let mut score_part2 = 0;
    let mut groups: Vec<HashSet<char>> = Vec::new();

    for line in lines {
        let l = line.len() / 2;

        let hs1: HashSet<char> = HashSet::from_iter(line[..l].chars());
        let hs2: HashSet<char> = HashSet::from_iter(line[l..].chars());

        let inter: HashSet<char> = hs1.intersection(&hs2).cloned().collect();

        for c in inter.iter() {
            score_part1 += badge_to_number(*c);
        }

        groups.push(HashSet::from_iter(line.chars()));
    }

    while groups.len() > 0 {
        let inter = groups.pop().unwrap().intersection(&groups.pop().unwrap()).cloned().collect();
        let inter: HashSet<char> = groups.pop().unwrap().intersection(&inter).cloned().collect();

        for c in inter.iter() {
            score_part2 += badge_to_number(*c);
        }
    }

    println!("#1 {score_part1}");
    println!("#2 {score_part2}")
}
