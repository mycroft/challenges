use std::fs;
use regex::Regex;
use std::collections::HashMap;

macro_rules! extract {
    ($x:expr, $y:expr, $z:expr) => { $x.captures($y).unwrap().get($z).unwrap().as_str().parse::<usize>().unwrap() }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut lines = contents.lines().collect::<Vec<&str>>();
    let mut sleeps : HashMap<usize, Vec<usize>> = HashMap::new();

    let r_guard = Regex::new(r"^\[(.*) (\d+):(\d+)\] Guard #(\d+) begins shift$").unwrap();
    let r_asleep = Regex::new(r"^\[(.*) (\d+):(\d+)\] falls asleep$").unwrap();
    let r_wakesup = Regex::new(r"^\[(.*) (\d+):(\d+)\] wakes up$").unwrap();

    lines.sort();

    let mut start_at = 0;
    let mut current_guard = 0;

    for line in lines.iter() {
        if r_guard.is_match(&line) {
            let hour = extract!(r_guard, &line, 2);
            let minute = extract!(r_guard, &line, 3);
            current_guard = extract!(r_guard, &line, 4);

            start_at = if hour > 0 {
                0
            } else {
                minute
            };
        }
        
        if r_asleep.is_match(&line) {
            let minute = extract!(r_asleep, &line, 3);

            start_at = minute;
        }

        if r_wakesup.is_match(&line) {
            let minute = extract!(r_wakesup, &line, 3);

            for n in start_at..minute {
                sleeps.entry(current_guard).or_insert(vec![0; 60])[n] += 1;
            }
        }
    }

    let mut max_guard_id = 0;
    let mut max_guard_time = 0;
    let mut max_minute = 0;

    for (k,v) in &sleeps {
        if v.iter().sum::<usize>() > max_guard_time {
            max_guard_time = v.iter().sum::<usize>();
            max_guard_id = *k;

            max_minute = 0;

            for i in 0..60 {
                if v[max_minute] < v[i] {
                    max_minute = i;
                }
            }
        }
    }

    println!("Part #1: {}", max_guard_id * max_minute);


    let mut max_guard_id = 0;
    let mut max_guard_minute_max = 0;
    let mut max_guard_minute = 0;

    for (k,v) in sleeps {
        for i in 0..60 {
            if v[i] > max_guard_minute {
                max_guard_id = k;
                max_guard_minute = v[i];
                max_guard_minute_max = i;
            }
        }
    }

    println!("Part #2: {}", max_guard_id * max_guard_minute_max);
}
