use std::fs;
use regex::Regex;
use std::collections::HashMap;

fn dump(input: &Vec<Vec<bool>>) {
    for x in 0..input.len() {
        let s : String = input[x].iter().map(|b| if *b { '#' } else { '.' }).collect();

        println!("{}", s);
    }

    println!("");
}

fn rotate(input: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut out = input.clone();

    for x in 0..input.len() {
        let new_y = input.len() - x - 1;

        for y in 0..input.len() {
            let new_x = y;

            out[new_x][new_y] = input[x][y];
        }
    }

    out
}

fn flip(input: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut out = input.clone();

    for x in 0..input.len() {
        out[x][0] = input[x][input.len() - 1];
    }

    for x in 0..input.len() {
        out[x][input.len() - 1] = input[x][0];
    }

    out
}

fn possibles(input: &Vec<Vec<bool>>) -> Vec<Vec<Vec<bool>>> {
    let mut possibles : Vec<Vec<Vec<bool>>> = vec![];

    let mut current = input.clone();

    for _i in 0..4 {
        current = rotate(&current);

        if possibles.iter().all(|x| *x != current) {
            possibles.push(current.clone());
        }

        let flipped = flip(&current);

        if possibles.iter().all(|x| *x != flipped) {
            possibles.push(flipped);
        }
    }

    possibles
}

fn str_to_vec(input: &str) -> Vec<Vec<bool>> {
    input.split("/").map(|x| x.chars().map(|b| b == '#').collect::<Vec<bool>>()).collect()
}


fn find_match(rules: &HashMap<Vec<Vec<bool>>, Vec<Vec<bool>>>, input: Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut rules_out : Vec<Vec<bool>> = vec![];
    let input_possibles = possibles(&input);

    for input_possible in input_possibles {
        if rules.contains_key(&input_possible) {
            rules_out = rules.get(&input_possible).unwrap().to_vec();
            break;
        }
    }

    rules_out
}

fn split(input: &Vec<Vec<bool>>) -> Vec<Vec<Vec<bool>>> {
    let mut out : Vec<Vec<Vec<bool>>> = vec![];

    for s in 2..=3 {
        if input.len() % s != 0 {
            continue;
        }

        for x in 0..(input.len() / s) {
            for y in 0..(input.len() / s) {
                let mut current = vec![];
                for x1 in 0..s {
                    let mut current_line = vec![];
                    for y1 in 0..s {
                        current_line.push(input[x * s + x1][y * s + y1]);
                    }

                    current.push(current_line);
                }

                out.push(current);
            }
        }

        break;
    }

    out
}

fn unsplit(splitted: &Vec<Vec<Vec<bool>>>) -> Vec<Vec<bool>> {
    let mut out : Vec<Vec<bool>>;

    let s = splitted.len() * splitted[0].len() * splitted[0][0].len();
    let s = (s as f64).sqrt() as usize;

    let sub_size = splitted[0][0].len();
    let num = (splitted.len() as f64).sqrt() as usize;

    out = vec![vec![false; s]; s];

    let mut current_line = 0;
    let mut current = 0;

    for sub_array in splitted {
        for (line_n, line) in sub_array.iter().enumerate() {
            for (el_n, el) in line.iter().enumerate() {
                // println!("[{}][{}]",
                //     line_n + current_line * sub_size,
                //     current * sub_size + el_n
                // );

                out[line_n + current_line * sub_size][current * sub_size + el_n] = *el;
            }
        }

        current += 1;

        if current == num {
            current = 0;
            current_line += 1;
        }
    }

    out
}

fn main() {
    let _contents = fs::read_to_string("input.txt").unwrap();
    let re = Regex::new(r"(.*) => (.*)$").unwrap();
    let lines = _contents.lines();

    let mut rules = HashMap::new();

    let input = ".#./..#/###";
    let mut input : Vec<Vec<bool>> = str_to_vec(&input);

    for line in lines {
        let caps = re.captures(line).unwrap();

        let src = caps.get(1).unwrap().as_str();
        let dst = caps.get(2).unwrap().as_str();

        rules.insert(str_to_vec(&src), str_to_vec(&dst));
    }

    for it in 0..18 {
        if input.len() < 4 {
            input = find_match(&rules, input);

            // dump(&input, it);

            continue;
        }

        let splitted = split(&input);

        let mut new_sub = vec![];
        for s in splitted {
            new_sub.push(find_match(&rules, s));
        }

        input = unsplit(&new_sub);

        //dump(&input, it);

        if it+1 == 5 || it+1 == 18 {
            println!("Count: {}", 
                input.iter().map(|x| x.iter().filter(|x| **x).count()).collect::<Vec<usize>>().iter().sum::<usize>()
            );
        }
    }
}
