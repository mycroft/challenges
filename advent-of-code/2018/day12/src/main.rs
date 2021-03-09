/*
 * AOC 2018 day 12.
 */
use anyhow::Result;
use std::fs;
use std::collections::HashMap;

fn step(mutations: &HashMap<Vec<char>, char>, index: isize, state: &str) -> Result<(String, isize)> {
    // We're adding 5 '.' on front of state
    let mut index = index - 5 + 2;
    let mut initial = true;

    let mut out : Vec<char> = vec![];

    let mut state = state.chars().collect::<Vec<char>>();
    for _ in 0..5 {
        state.insert(0, '.');
    }

    for _ in 0.. 5 {
        state.push('.');
    }

    for s in 0..state.len() - 4 {
        let sub_array = &state[s..s+5];

        let n_char = *mutations.get(sub_array).unwrap();

        if initial && n_char == '.' {
            index += 1;
        } else {
            initial = false;
            out.push(n_char);
        }
    }

    loop {
        let l = out.len();

        if out[l - 1] != '.' {
            break;
        }

        out.pop();
    }

    Ok((out.iter().collect(), index))
}

fn score(index: isize, state: &str) -> isize {
    let state = state.chars().collect::<Vec<char>>();
    let mut result = 0;

    for (no, &c) in state.iter().enumerate() {
        if c == '#' {
            result += index + no as isize;
        }
    }

    result
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("input.txt")?;
    let lines = contents.lines().collect::<Vec<&str>>();

    let initial_index : isize = 0;
    let initial_state = lines[0].split(" ").collect::<Vec<&str>>()[2];

    // println!("initial_state: {}", initial_state);

    let mut mutations : HashMap<Vec<char>, char>= HashMap::new();

    for (lineno, line) in lines.iter().enumerate() {
        if lineno < 2 {
            continue;
        }

        let fields = line.split(" ").collect::<Vec<&str>>();

        mutations.insert(fields[0].chars().collect::<Vec<char>>(), fields[2].chars().nth(0).unwrap());
    }

    // println!("mutations: {:?}", mutations);

    let mut index = initial_index;
    let mut state = initial_state.to_string();

    for _ in 0..20 {
        let res = step(&mutations, index, &state)?;

        // println!("{} // {}", res.0, score(res.1, &res.0));

        state = res.0;
        index = res.1;
    }

    println!("Part #1: {}", score(index, &state));

    let mut index = initial_index;
    let mut state = initial_state.to_string();

    let final_step = 50000000000i64;

    for it in 0..final_step {
        let res = step(&mutations, index, &state)?;

        if res.0 == state {
            // println!("Pattern is the same at iteration {}!", it);
            // println!("Index is {}", res.1);

            index += (final_step - it) as isize;

            break;
        }

        state = res.0;
        index = res.1;
    }

    println!("Part #2: {}", score(index, &state));

    Ok(())
}
