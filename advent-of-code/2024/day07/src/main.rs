use std::{collections::{HashMap, HashSet}, fs};

fn read_input(fp: &str) -> HashMap<i128, Vec<i128>> {
    let mut result = HashMap::new();
    let content = fs::read_to_string(fp).expect("Cannot read file");

    for line in content.lines() {
        let parts = line.split(":").collect::<Vec<&str>>();
        let key = parts[0].parse::<i128>().unwrap();
        let value = parts[1].trim().split(" ").map(|x| x.trim().parse::<i128>().unwrap()).collect::<Vec<i128>>();
        result.insert(key, value);
    }

    result
}

fn solve_uniq_step1(result: i128, elements: Vec<i128>) -> bool {
    let mut set = HashSet::new();

    set.insert(elements[0]);

    for el in elements.iter().skip(1) {
        let mut new_set = HashSet::new();

        for &x in set.iter() {
            let add_result = x + el;
            if add_result <= result {
                new_set.insert(add_result);
            }
            let mul_result = x * el;
            if mul_result <= result {
                new_set.insert(x * el);
            }
        }

        set = new_set;
    }

    set.contains(&result)
}

fn solve_uniq_step2(result: i128, elements: Vec<i128>) -> bool {
    let mut set = HashSet::new();

    set.insert(elements[0]);

    for el in elements.iter().skip(1) {
        let mut new_set = HashSet::new();

        for &x in set.iter() {
            let add_result = x + el;
            if add_result <= result {
                new_set.insert(add_result);
            }
            let mul_result = x * el;
            if mul_result <= result {
                new_set.insert(x * el);
            }
            let comb_result = x * 10_i128.pow(el.to_string().len() as u32) + el;
            if comb_result <= result {
                new_set.insert(comb_result);
            }
        }

        set = new_set;
    }

    set.contains(&result)
}

fn solve(input: HashMap<i128, Vec<i128>>) -> (i128, i128) {
    let mut result_step1 = 0;
    let mut result_step2 = 0;
    for (key, value) in input.iter() {
        if solve_uniq_step1(*key, value.clone()) {       
            result_step1 += *key;
        }
        if solve_uniq_step2(*key, value.clone()) {       
            result_step2 += *key;
        }
    }

    (result_step1, result_step2)
}



fn main() {
    let input = read_input("input.txt");

    let (result_step1, result_step2) = solve(input);

    println!("#1 {}", result_step1); // 28730327770375
    println!("#2 {}", result_step2); // 424977609625985
}
