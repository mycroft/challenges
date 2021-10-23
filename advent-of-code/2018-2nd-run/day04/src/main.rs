use std::fs;
use regex::Regex;
use chrono::{Duration, NaiveDateTime};

use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let mut lines = contents.lines().collect::<Vec<&str>>();
    let re = Regex::new(r"^\[(.*)\] (.*)$").unwrap();
    let re_g = Regex::new(r"Guard #(.*) begins shift").unwrap();

    let mut asleep_durations : HashMap<usize, i64> = HashMap::new();
    let mut asleep_minutes : HashMap<usize, Vec<usize>> = HashMap::new();

    lines.sort();

    let mut current_guard_id = 0;
    let mut current_datetime = NaiveDateTime::from_timestamp(0, 0);

    for line in lines {
        // println!("line: {}", line);
        // println!("{:?}", re.captures(&line));
        let caps = re.captures(&line).unwrap();

        let datetime = match NaiveDateTime::parse_from_str(&caps[1], "%Y-%m-%d %H:%M") {
            Ok(x) => x,
            Err(_) => {
                panic!("failed parsing date time field.");
            }
        };

        match &caps[2] {
            "wakes up" => {
                let diff : Duration = datetime - current_datetime;
                // println!("Adding {} minutes to {}.", diff.num_minutes(), current_guard_id);
                *asleep_durations.entry(current_guard_id).or_insert(0) += diff.num_minutes();
                let minutes = asleep_minutes.entry(current_guard_id).or_insert([0; 60].to_vec());

                // Adding 1 from current_datetime's minute to datetime's minute
                for i in 0..diff.num_minutes() {
                    minutes[(i as usize + current_datetime.format("%M").to_string().parse::<usize>().unwrap()) % 60] += 1;
                }

                current_datetime = datetime;
            },
            "falls asleep" => {
                current_datetime = datetime;
            },
            x => {
                let caps_shift = re_g.captures(&x).unwrap();
                let guard_id = caps_shift[1].parse::<usize>().unwrap();

                current_guard_id = guard_id;
                current_datetime = datetime;
            }
        }
    }

    let mut guard_max_sleep : Option<usize> = None;
    let mut guard_max_sleep_duration : Option<i64> = None;

    for (k, v) in asleep_durations.iter() {
        if guard_max_sleep_duration == None || guard_max_sleep_duration.unwrap() < *v {
            guard_max_sleep = Some(*k);
            guard_max_sleep_duration = Some(*v);
        }
    }

    let mut sleep_max_duration : Option<usize> = None;
    let mut sleep_max_minute : Option<usize> = None;

    // println!("Guard choosen: {}", guard_max_sleep.unwrap());

    for (i, m) in asleep_minutes.entry(guard_max_sleep.unwrap()).or_insert(Vec::new()).iter().enumerate() {
        if sleep_max_duration == None || sleep_max_duration.unwrap() < *m {
            sleep_max_minute = Some(i);
            sleep_max_duration = Some(*m);
        }
    }

    // println!("{:?}", asleep_durations);
    // println!("{:?}", asleep_minutes);

    let mut minute_most_slept : Option<usize> = None;
    let mut minute_most_slept_duration : Option<usize> = None;
    let mut minute_most_slept_guard : Option<usize> = None;

    for (guard_id, _) in asleep_durations.iter() {
        for (i, m) in asleep_minutes.entry(*guard_id).or_insert(Vec::new()).iter().enumerate() {
            if minute_most_slept == None || minute_most_slept_duration.unwrap() < *m {
                minute_most_slept = Some(i);
                minute_most_slept_duration = Some(*m);
                minute_most_slept_guard = Some(*guard_id);
            }
        }
    }

    println!("#1: {}", sleep_max_minute.unwrap() * guard_max_sleep.unwrap());
    println!("#2: {}", minute_most_slept_guard.unwrap() * minute_most_slept.unwrap());
}
