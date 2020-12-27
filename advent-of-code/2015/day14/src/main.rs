use std::fs;

use std::cmp::min;
use regex::Regex;

use std::collections::HashMap;

// Vixen can fly 19 km/s for 7 seconds, but then must rest for 124 seconds.

#[derive(Debug)]
struct Reinder<'a> {
	name: &'a str,
	speed: u32,
	duration: u32,
	rest: u32,
	distance: u32,
}

fn compute(time : u32, speed : u32, duration : u32, rest : u32) -> u32 {
	let cycle_duration = duration + rest;
	let _cycle_start = time - time % cycle_duration;
	let cycle_remainder = time % cycle_duration;

	let cycle_number = (time / cycle_duration) as u32;

	cycle_number * (speed * duration) + min(duration, cycle_remainder) * speed
}

fn get_max<'a>(reinders: &'a Vec<Reinder>, time : u32) -> (Vec<&'a str>, u32) {
	let mut max_distance = None;
	let mut max_reinders = Vec::new();

	for reinder in reinders {
		let value = compute(time, reinder.speed, reinder.duration, reinder.rest);
		max_distance = match max_distance {
			None => {
				max_reinders.push(reinder.name);
				Some(value)
			},
			Some(current_max) => {
				if value > current_max {
					max_reinders.clear();
					max_reinders.push(reinder.name);
					Some(value)
				} else if value == current_max {
					max_reinders.push(reinder.name);
					Some(value)
				} else {
					Some(current_max)
				}
			}
		};
	}

	(max_reinders, max_distance.unwrap())
}

fn main() {
	let content = fs::read_to_string("input.txt")
		.expect("Something went wrong reading the file");

	let re = Regex::new(r"^(.*) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.$").unwrap();

	let lines = content.lines();

	let mut reinders = Vec::new();

	for line in lines {
		let cap = re.captures(line).unwrap();

		let reinder = Reinder{
			name: 		cap.get(1).unwrap().as_str(),
			speed: 		cap.get(2).unwrap().as_str().parse::<u32>().unwrap(),
			duration: 	cap.get(3).unwrap().as_str().parse::<u32>().unwrap(),
			rest:		cap.get(4).unwrap().as_str().parse::<u32>().unwrap(),
			distance:   0,
		};

		reinders.push(reinder);
	}

	let time = 2503;
	let (names, max_distance) = get_max(&reinders, time);

	println!("Part #1: {} (by {})", max_distance, names.get(0).unwrap());

	let mut winners = HashMap::new();

	for time in 1..=time {
		let (names, _max_distance) = get_max(&reinders, time);

		for winner in names {
			let value = match winners.get(winner) {
				None => 1, 
				Some(val) => val + 1,
			};

			winners.insert(winner, value);
		}
	}

	let mut max_value = 0;
	let mut max_winner = "";

	for (winner, value) in winners {
		if max_value > value {
			continue;
		}

		max_value = value;
		max_winner = winner;
	}

	println!("Part #2: {} (by {})",
		max_value, max_winner
	);
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn test_compute() {
    	assert_eq!(0, compute(0, 1, 1, 1));
    	assert_eq!(5, compute(1, 5, 2, 10));
    	assert_eq!(10, compute(2, 5, 2, 10));
    	assert_eq!(10, compute(3, 5, 2, 10));
    	assert_eq!(10, compute(10, 5, 2, 10));

    	assert_eq!(1120, compute(1000, 14, 10, 127));
    	assert_eq!(1056, compute(1000, 16, 11, 162));
    }
}
