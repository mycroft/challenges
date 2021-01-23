use mkz_aoc::file;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let lines = file::read_to_lines("input.txt").unwrap();
    let re = Regex::new(r"(\d+): (\d+)").unwrap();
    let mut weights : HashMap<usize, usize> = HashMap::new();

    for line in lines {
        let caps = re.captures(&line).unwrap();

        weights.insert(
            caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
        );
    }

    let _res : usize = weights
        .iter()
        .filter(|(k, v)| *k % ((*v-1) * 2) == 0)
        .map(|(k, v)| k * v)
        .sum()
    ;

    println!("Part #1: {:?}", _res);

    for delay in 0.. {
        let caught = weights
            .iter()
            .any(|(k, v)| (*k + delay) % ((*v-1) * 2) == 0)
        ;

        if !caught {
            println!("Part #2: {}", delay);
            break;
        }
    }
}
