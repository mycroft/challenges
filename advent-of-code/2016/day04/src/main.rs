use std::io::Read;
use std::fs::File;
use regex::Regex;

use std::collections::HashMap;

fn rotate_letters(_pass: &str, id: u32) -> String {
    let mut letters : Vec<u8> = _pass.as_bytes().to_vec();

    for _idx in 0..id {
        let letters_len = letters.len();
        for letter_id in 0..letters_len {
            if letters[letter_id] < 'a' as u8 || letters[letter_id] > 'z' as u8 {
                continue;
            }

            letters[letter_id] += 1;
            if letters[letter_id] > 'z' as u8 {
                letters[letter_id] = 'a' as u8;
            }
        }
    }

    String::from_utf8(letters).unwrap()
}

fn compute_chk(pass: &str) -> String {
    let mut h : HashMap<char, u32> = HashMap::new();
    let mut chk = String::new();

    for c in pass.chars() {
        if c == '-' {
            continue;
        }
        *h.entry(c).or_insert(0) += 1;
    }

    for _idx in 0..5 {
        let mut letters : Vec<char> = vec![];
        let mut max_value = 0;

        for (&k, &v) in h.iter() {
            if v > max_value {
                max_value = v;
                letters = vec![k];
            } else if v == max_value {
                letters.push(k);
            }
        }

        letters.sort();
        let top_letter = letters[0];

        h.remove(&top_letter);

        chk.push(top_letter);
    }

    chk
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut sector_ids_sum = 0;
    let mut sector_id_northpole = 0;

    let re = Regex::new(r"(.*)-(\d+)\[(.*)\]").unwrap();

    for line in contents.lines() {
        let captures = re.captures(line).unwrap();

        let pass = captures.get(1).unwrap().as_str();
        let sector_id = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let chk = captures.get(3).unwrap().as_str();

        if chk == compute_chk(&pass) {
            // println!("{:?}", compute_chk(&pass));
            sector_ids_sum += sector_id;
        }

        let decoded = rotate_letters(&pass, sector_id);

        if decoded == "northpole-object-storage" {
            sector_id_northpole = sector_id;
        }
    }

    println!("Part #1: {}", sector_ids_sum);
    println!("Part #2: {}", sector_id_northpole);

}
