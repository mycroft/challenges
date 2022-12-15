use std::fs::read_to_string;
use std::cmp::Ordering;

fn get_elements_in_list(l: &String) -> Vec<String> {
    let mut elements = Vec::new();
    let letters = l.chars().collect::<Vec<char>>();

    let mut current_letters = String::new();
    let mut list_level = 0;

    for idx in 1..letters.len() - 1 {
        match letters[idx] {
            '[' => {
                list_level += 1;
                current_letters.push('[');
            },
            ']' => {
                list_level -= 1;
                current_letters.push(']');
            },
            '0'..='9' => {
                current_letters.push(letters[idx]);
            },
            ',' => {
                if list_level == 0 {
                    elements.push(current_letters);
                    current_letters = String::new();
                } else {
                    current_letters.push(',');
                }
            },
            _ => unreachable!()
        }

        // println!("{idx}, {}", letters[idx]);
    }

    if current_letters.len() > 0 {
        elements.push(current_letters);
    }

    elements
}

fn is_list(s: &String) -> bool {
    s.len() > 0 && s.chars().nth(0).unwrap() == '['
}

fn compare(s0: &String, s1: &String) -> Ordering {
    // println!("Compare {s0}");
    // println!("     vs {s1}");

    if !is_list(&s0) && !is_list(&s1) {
        return if s0.parse::<u32>().unwrap() == s1.parse::<u32>().unwrap() {
            Ordering::Equal
        } else if s0.parse::<u32>().unwrap() < s1.parse::<u32>().unwrap() {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    } else if is_list(&s0) && is_list(&s1) {
        let s0_elements = get_elements_in_list(&s0);
        let s1_elements = get_elements_in_list(&s1);

        // println!("Compare list:{s0} vs list:{s1}");
        // println!("{} {}", s0_elements.len(), s1_elements.len());

        // if s0_elements.len() == 0 && s1_elements.len() > 0 {
        //     return true;
        // }

        for idx in 0..s0_elements.len() {
            // before doing any comparaison, we check if elements exists.
            if s1_elements.len() <= idx {
                // woops, no more element in right side
                return Ordering::Greater;
            }

            // if elements are similar, just continue.
            // xxx if 5 & [5] they are still equal
            // if s0_elements[idx] == s1_elements[idx] {
            //     continue;
            // }

            match compare(&s0_elements[idx], &s1_elements[idx]) {
                Ordering::Equal => {
                    continue;
                },
                x => {
                    return x;
                }
            }
        }

        if s0_elements.len() < s1_elements.len() {
            return Ordering::Less;
        }

        //  no more element and all elements were checked: returns true
        return Ordering::Equal;
    } else {
        if is_list(&s0) {
            // s1 must become a list
            let mut enlisted_s1 = String::new();
            enlisted_s1.push('[');
            enlisted_s1.push_str(&s1);
            enlisted_s1.push(']');

            return compare(s0, &enlisted_s1);
        } else {
            // s0 must become a list
            let mut enlisted_s0 = String::new();
            enlisted_s0.push('[');
            enlisted_s0.push_str(&s0);
            enlisted_s0.push(']');

            return compare(&enlisted_s0, s1);
        }
    }
}

fn parse(fp: &str) -> Vec<(String, String)> {
    let mut signals = Vec::new();
    let contents = read_to_string(fp).unwrap();

    let lines = contents.lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let mut idx = 0;
    let mut line0 = String::new();
    loop {
        if idx >= lines.len() {
            break;
        }

        if idx % 3 == 0 {
            line0 = lines[idx].clone();
        } else if (idx % 3) == 1 {
            signals.push(
                (line0.clone(), lines[idx].clone())
            );
        }

        idx += 1;
    }

    signals
}

fn all_signals(signals: &Vec<(String, String)>) -> Vec<String> {
    let mut res = Vec::new();

    for sig in signals {
        res.push(sig.0.clone());
        res.push(sig.1.clone());
    }

    res
}

fn compute_indices_sum(fp: &str) -> u32 {
    let mut result = 0;
    let mut idx = 1;

    let signals = parse(fp);

    for signal in signals {
        match compare(&signal.0, &signal.1) {
            Ordering::Less => {
                result += idx;
                // println!("{idx} => true");
            }
            _ => {
                // println!("{idx} => false");
            },
        }

        idx += 1;
    }

    result
}

fn sort_signals_and_compute(fp: &str) -> u32 {
    let signals = parse(fp);
    let mut signals = all_signals(&signals);

    signals.push("[[2]]".to_string());
    signals.push("[[6]]".to_string());

    // sort signals
    signals.sort_by(compare);

    // find indices for [[2]] & [[6]]
    let i0 = 1 + signals.iter().position(|x| x == &"[[2]]".to_string()).unwrap() as u32;
    let i1 = 1 + signals.iter().position(|x| x == &"[[6]]".to_string()).unwrap() as u32;

    i0 * i1
}

fn main() {
    println!("#1 {}", compute_indices_sum("input.txt"));
    println!("#2 {}", sort_signals_and_compute("input.txt"));
}

#[test]
fn test_get_elements_in_list() {
    assert_eq!(
        ["1", "2", "3"].to_vec(),
        get_elements_in_list(&"[1,2,3]".to_string())
    );
    assert_eq!(
        ["[1]", "[2,3,4]"].to_vec(),
        get_elements_in_list(&"[[1],[2,3,4]]".to_string())
    );
}

#[test]
fn test_is_list() {
    assert!(is_list(&"[1,2,3]".to_string()));
    assert!(!is_list(&"0".to_string()));
}

#[test]
fn test_compare() {
    assert_eq!(
        Ordering::Less,
        compare(
            &"[1]".to_string(),
            &"[2]".to_string(),
        )
    );

    assert_eq!(
        Ordering::Less,
        compare(
            &"[1]".to_string(),
            &"2".to_string(),
        )
    );

    assert_eq!(
        Ordering::Less,
        compare(
            &"1".to_string(),
            &"2".to_string(),
        )
    );

    assert_eq!(
        Ordering::Greater,
        compare(
            &"[2]".to_string(),
            &"[1]".to_string(),
        )
    );

    assert_eq!(
        Ordering::Equal,
        compare(
            &"[1]".to_string(),
            &"1".to_string(),
        )
    );

    assert_eq!(
        Ordering::Greater,
        compare(
            &"2".to_string(),
            &"1".to_string(),
        )
    );

    assert_eq!(
        Ordering::Less,
        compare(
            &"[1,1,3,1,1]".to_string(),
            &"[1,1,5,1,1]".to_string(),
        )
    );
}

#[test]
fn test_compute() {
    assert_eq!(
        13,
        compute_indices_sum("input.txt_test")
    );
}

#[test]
fn test_sort_compute() {
    assert_eq!(
        140,
        sort_signals_and_compute("input.txt_test")
    );
}

#[test]
fn test_debug() {
    assert_eq!(
        1,
        compute_indices_sum("input.txt_test2")
    );
}