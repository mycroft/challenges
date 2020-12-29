use std::collections::VecDeque;

fn score(containers: &mut VecDeque<i16>, start_idx: usize, remaining: i16, valid_size: &mut Vec<usize>) -> (usize, usize) {
	if remaining == 0 {
		valid_size.push(containers.len());
		return (1, containers.len());
	}

	if remaining < 0 {
		return (0, usize::MAX);
	}

	let mut total = 0;

	let size = containers.len();
	for idx in start_idx..size {
		let val = *containers.get(idx).unwrap();
		containers.remove(idx);
		let (current_score, _) = score(containers, idx, remaining - val, valid_size);

		total += current_score;

		containers.insert(idx, val);
	}

	let max_value = match valid_size.iter().max() {
		None => 0,
		Some(v) => *v
	};

	let count = valid_size.iter().filter(|x| **x == max_value).count();

	(total, count)
}

fn main() {
	let init_containers = vec![
		43, 3, 4, 10, 21, 44, 4, 6, 47, 41, 34, 17, 17, 44, 36, 31, 46, 9, 27, 38
	];
	let init_liquid = 150;

	let mut containers : VecDeque<i16> = VecDeque::new();
	for c in init_containers {
		containers.push_back(c);
	}

	let mut valid_size = vec![];

	let (value, min_containers) = score(&mut containers, 0, init_liquid, &mut valid_size);

    println!("Part #1: {}", value);
    println!("Part #2: {}", min_containers);
}

#[test]
fn example() {
	let init_containers = vec![20, 15, 10, 5, 5];
	let init_liquid = 25;

	let mut containers : VecDeque<i16> = VecDeque::new();
	for c in init_containers {
		containers.push_back(c);
	}

	assert_eq!((4, 3), score(&mut containers, 0, init_liquid, &mut vec![]));
}