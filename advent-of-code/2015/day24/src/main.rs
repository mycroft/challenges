use std::fs;
use std::collections::HashSet;
use itertools::Itertools;

#[allow(dead_code)]
fn create_group(grp_size: u32, numbers: &Vec<u32>, index: usize, pick_numbers: &mut Vec<u32>) -> u128 {
    let sum = pick_numbers.iter().sum::<u32>();
    let mut min_qe = 0;

    if sum == grp_size {
        return pick_numbers.iter().fold(1, |x, y| x as u128 * *y as u128);
    }

    if sum > grp_size {
        return 0;
    }

    for idx in index..numbers.len() {
        let number = numbers[idx];

        pick_numbers.push(number);
        let res = create_group(grp_size, numbers, idx + 1, pick_numbers);
        pick_numbers.pop();

        if res > 0 && (res < min_qe || min_qe == 0) {
            min_qe = res;
        }
    }

    min_qe
}

// https://old.reddit.com/r/adventofcode/comments/3y1s7f/day_24_solutions/
fn create_group2(grp_size: u128, numbers: &HashSet<u128>, parts: u8, sub: u8) -> u128 {
    for l in 1..=numbers.len() {
        for combi in numbers
            .into_iter()
            .sorted()
            .copied()
            .combinations(l)
            .filter(|x| x.iter().copied().sum::<u128>() == grp_size) {
            if sub == 2 {
                return 1
            } else {
                let current_set : HashSet<u128> = combi.into_iter().collect();
                let subset : HashSet<u128> = numbers.difference(&current_set).into_iter().copied().collect();

                let res = create_group3(grp_size, &subset, parts, sub - 1);

                if sub < parts {
                    return res;
                } else if res > 0 {
                    return current_set.iter().fold(1, |x , y| x * y);
                }
            }
        }
    }

    0
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();
    let numbers = lines.map(|x| x.parse::<u128>().unwrap()).collect::<Vec<u128>>();
    let total : u128 = numbers.iter().sum();
    let numbers_hs : HashSet<u128> = numbers.into_iter().collect();
    
    let parts = 3;
    let res = create_group2(total / parts, &numbers_hs, parts as u8, parts as u8);
    println!("Part #1: {:?}", res);

    let parts = 4;
    let res = create_group2(total / parts, &numbers_hs, parts as u8, parts as u8);
    println!("Part #2: {:?}", res);

    // Also working, but requires more computation time:
    /*
    println!("Part #1: {:?}", create_group(total / 3, &numbers, 0, &mut vec![]));
    println!("Part #2: {:?}", create_group(total / 4, &numbers, 0, &mut vec![]));
    */
}
