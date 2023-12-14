use std::{fs, collections::{VecDeque, HashMap}};

fn compute(
    cache: &mut HashMap<(VecDeque<char>, VecDeque<usize>, usize), usize>,
    tokens: &VecDeque<char>,
    springs: &VecDeque<usize>,
    current_spring: usize) -> usize
{
    if let Some(entry) = cache.get(&(tokens.clone(), springs.clone(), current_spring)) {
        return *entry;
    }

    let orig_tokens = tokens.clone();
    let orig_springs = springs.clone();

    let mut tokens = tokens.clone();
    let mut springs = springs.clone();

    // no more token
    if tokens.is_empty() {
        // we're in a spring
        if current_spring > 0 {
            let res = (springs.len() == 1 && current_spring == springs[0]) as usize;
            cache.insert((orig_tokens, orig_springs, current_spring), res);
            return res;
        } else {
            let res = springs.is_empty() as usize;
            cache.insert((orig_tokens, orig_springs, current_spring), res);
            return res;
        }
    }

    if current_spring > 0 && (springs.is_empty() || springs[0] < current_spring) {
        cache.insert((orig_tokens, orig_springs, current_spring), 0);
        return 0;
    }

    match tokens.pop_front().unwrap() {
        '.' => {
            if current_spring > 0 {
                // the current spring we're does not fit the group we were supposed to be in
                if !springs.is_empty() && current_spring != springs[0] {
                    cache.insert((orig_tokens, orig_springs, current_spring), 0);
                    return 0;
                } else {
                    springs.pop_front();
                }
            }
            let res = compute(cache, &tokens, &springs, 0);
            cache.insert((orig_tokens, orig_springs, current_spring), res);
            res
        },
        '#' => {
            let res = compute(cache, &tokens, &springs, current_spring + 1);
            cache.insert((orig_tokens, orig_springs, current_spring), res);
            res
        },
        '?' => {
            if springs.is_empty() || current_spring == springs[0] {
                springs.pop_front();
    
                let res = compute(cache, &tokens, &springs, 0);
                cache.insert((orig_tokens, orig_springs, current_spring), res);
                res
            } else if current_spring > 0 {
                let res = compute(cache, &tokens, &springs, current_spring + 1);
                cache.insert((orig_tokens, orig_springs, current_spring), res);
                res
            } else {
                let res = compute(cache, &tokens, &springs, current_spring + 1) + compute(cache, &tokens, &springs, current_spring);
                cache.insert((orig_tokens, orig_springs, current_spring), res);
                res
            }
        },
        _ => unreachable!(),
    }
}

fn check(tokens: &VecDeque<char>, springs: &VecDeque<usize>) -> usize {
    let mut cache: HashMap<(VecDeque<char>, VecDeque<usize>, usize), usize> = HashMap::new();
    compute(&mut cache, tokens, springs, 0)
}

fn check_str(tokens_str: &str, springs_str: &str) -> (usize, usize) {
    let tokens = tokens_str.chars().collect::<VecDeque<char>>();
    let mut springs = VecDeque::new();
    for spring in springs_str.split(',') {
        springs.push_back(spring.parse::<usize>().unwrap());
    }

    let mut tokens_5 = VecDeque::new();
    let mut springs_5 = VecDeque::new();

    for n in 0..5 {
        for c in tokens_str.chars() {
            tokens_5.push_back(c);
        }
        if n != 4 {
            tokens_5.push_back('?');
        }
        for spring in springs_str.split(',') {
            springs_5.push_back(spring.parse::<usize>().unwrap());
        }
    }

    (check(&tokens, &springs), check(&tokens_5, &springs_5))
    // (check(&tokens, &springs), check(&tokens, &springs))
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("file to read");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut p1 = 0;
    let mut p2 = 0;

    for line in lines {
        let parts = line.split(' ').collect::<Vec<&str>>();
        let (res_p1, res_p2) = check_str(parts[0], parts[1]);
        //sprintln!("{} {}", line, res_p1);

        p1 += res_p1;
        p2 += res_p2;

        //break;
    }

    println!("#1 {}", p1); // 7204
    println!("#2 {}", p2); // 7204
}

