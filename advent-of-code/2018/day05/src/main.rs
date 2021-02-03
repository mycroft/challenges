use std::fs;

fn purge(v: &Vec<char>, c: Option<char>) -> usize {
    let mut index = 0;

    let mut v = v.clone();

    loop {
        if index >= v.len() - 1 {
            break;
        }

        if c == Some(v[index]) || c == Some(v[index].to_ascii_lowercase()) {
            v.remove(index);

            if index != 0 {
                index -= 1;    
            }
            
            continue;
        }

        if v[index] != v[index + 1] && (v[index] == v[index + 1].to_ascii_lowercase() || v[index].to_ascii_lowercase() == v[index + 1]) {
            v.remove(index);
            v.remove(index);

            if index != 0 {
                index -= 1;    
            }

            continue;
        }

        index += 1
    }

    v.len()
}

fn main() {
    let mut contents = fs::read_to_string("input.txt").unwrap();
    contents = contents.trim().to_string();

    let array = contents.chars().collect::<Vec<char>>();

    // 10450
    let l = purge(&array, None);

    println!("Part #1: {}", l);

    let mut min_l : Option<usize> = None;

    for c in 'a'..='z' {
        let l = purge(&array, Some(c));

        if min_l == None {
            min_l = Some(l);
        } else if l < min_l.unwrap() {
            min_l = Some(l);
        }
    }

    println!("Part #2: {}", min_l.unwrap());
}
