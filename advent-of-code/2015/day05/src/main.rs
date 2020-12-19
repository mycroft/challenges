use std::fs;

fn is_nice(word : &str) -> bool {
    let mut voyelle = 0;
    let mut has_double_pattern = false;
    let mut has_bad_pattern = false;

    if word == "" {
        return false;
    }

    for c in word.chars() {
        if "aeiou".contains(c) {
            voyelle += 1;
        }
    }

    for (i, c) in word.chars().enumerate() {
        if i == word.len() - 1 {
            break;
        }

        if c == word.chars().nth(i + 1).unwrap() {
            has_double_pattern = true;
        }

        for bad_pattern in vec!["ab", "cd", "pq", "xy"] {
            if word.contains(bad_pattern) {
                has_bad_pattern = true;
            }
        }
    }

    voyelle >= 3 && has_double_pattern && false == has_bad_pattern
}

fn is_nice2(word : &str) -> bool {

    let mut first_property = false;
    let mut second_property = false;

    if word == "" {
        return false;
    }

    for (i, _) in word.chars().enumerate() {
        if i >= word.len() - 2 {
            break;
        }

        if word[i+2..].contains(&word[i..i+2]) {
            first_property = true;
        }
    }

    for (i, c) in word.chars().enumerate() {
        if i >= word.len() - 2 {
            break;
        }

        if word.chars().nth(i + 2).unwrap() == c {
            second_property = true;
        }
    }

    first_property && second_property
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let words : Vec<&str> = contents.split("\n").collect();

    let mut nice_words = 0;
    let mut nice_words2 = 0;

    for word in words {
        if is_nice(word) {
            nice_words += 1;
        }

        if is_nice2(word) {
            nice_words2 += 1;
        }
        // println!("{:?} {:?} {:?} {:?}", word, voyelle, has_double_pattern, has_bad_pattern);
    }

    println!("{:?}", nice_words);
    println!("{:?}", nice_words2);
}