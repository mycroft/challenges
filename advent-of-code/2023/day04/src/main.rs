use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("a file to open");
    let mut cards: HashMap<usize, usize> = HashMap::new();

    let mut p1 = 0;

    for (mut card_number, line) in contents.lines().enumerate() {
        card_number += 1;
        if !cards.contains_key(&card_number) {
            cards.insert(card_number, 1);
        } else {
            cards.insert(card_number, 1+ *cards.get(&card_number).unwrap());
        }
    
        let parts = line.split(": ").collect::<Vec<&str>>();

        let winning = parts.get(1).unwrap().split('|').next().unwrap();
        let numbers = parts.get(1).unwrap().split('|').nth(1).unwrap();

        let winning = winning.split(' ').filter(|x| !x.is_empty() && x.is_ascii()).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let numbers = numbers.split(' ').filter(|x| !x.is_empty() && x.is_ascii()).map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        let mut p = 0;
        let mut matches = 0;

        for num in numbers {
            if winning.contains(&num) {
                if p == 0 {
                    p = 1;
                } else {
                    p *= 2;
                }
                matches += 1;
            }
        }

        // adding new cards
        let count = *cards.get(&card_number).unwrap();

        for n in 1..matches+1 {
            cards.entry(n+card_number).or_insert(0);

            let new_count = *cards.get(&(n+card_number)).unwrap() + count;
            cards.insert(n+card_number, new_count);
        }

        p1 += p;
    }

    let mut p2 = 0;
    for (_k, v) in cards {
        p2 += v;
    }

    println!("#1 {:?}", p1);
    println!("#2 {:?}", p2);
}
