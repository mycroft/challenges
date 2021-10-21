use std::fs;
use std::collections::HashMap;

fn count(s: String) -> (usize, usize) {
    let mut hm : HashMap<char, usize> = HashMap::new();
    let mut res = (0, 0);

    s.chars().map(|c| *hm.entry(c).or_insert(0) += 1).count();

    for (_, v) in hm {
        if v == 2 {
            res.0 = 1;
        } else if v == 3 {
            res.1 = 1;
        }
    }
    res
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines()
        .collect::<Vec<&str>>();

    let mut res = (0, 0);
    let mut found = false;
    let mut final_word = String::from("");

    for line in &lines {
        let count_res = count(String::from(*line));
        res.0 += count_res.0;
        res.1 += count_res.1;
    }

    for (i, line) in lines.iter().enumerate() {
        for (j, line2) in lines.iter().enumerate() {
            if line == line2 || i > j {
                continue;
            }

            let diff = line.chars()
                .enumerate()
                .map(|(i, c)| c != line2.chars().nth(i).unwrap())
                .filter(|x| *x)
                .count();

            if diff == 1 {
                found = true;

                line
                    .chars()
                    .enumerate()
                    .map(|(i, c)| {
                        if c == line2.chars().nth(i).unwrap() {
                            final_word.push(c)
                        } 
                    })
                    .count();

                break;
            }
        }

        if found {
            break;
        }
    }

    println!("#1: {}", res.0 * res.1);
    println!("#2: {}", final_word);
}

#[test]
fn basic_count() {
    assert_eq!((0, 0), count(String::from("abcdef")));
    assert_eq!((1, 1), count(String::from("bababc")));
}