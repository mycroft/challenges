use std::fs;

fn count(letters: &mut Vec<char>) -> usize {
    let mut idx = 0;

    loop {
        if idx + 1 >= letters.len() {
            break;
        }

        if letters[idx] != letters[idx + 1] 
            && letters[idx].to_ascii_lowercase() == letters[idx + 1].to_ascii_lowercase() {
            // remote those two letters
            letters.remove(idx);
            letters.remove(idx);
            if idx > 0 {
                idx -= 1;
            }
            continue;
        }

        idx += 1;
    }

    return letters.len();
}

fn main() {
    let contents : String = fs::read_to_string("input.txt").unwrap();
    let contents = contents.trim();
    let mut letters = contents.chars().collect::<Vec<char>>();

    let cnt = count(&mut letters);
    let mut min_chain = letters.len();

    for z in 'A'..'Z' {
        let mut lttrs : Vec<char> = letters
            .iter()
            .filter(|&c| *c != z && *c != z.to_ascii_lowercase())
            .map(|&c| c)
            .collect();

        let cnt_2 = count(&mut lttrs);
        if cnt_2 < min_chain {
            min_chain = cnt_2;
        }
    }

    println!("#1: {}", cnt);
    println!("#2: {}", min_chain);
}
