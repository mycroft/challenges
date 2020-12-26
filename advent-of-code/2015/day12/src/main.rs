use std::fs;

fn get_next_num(content: &Vec<u8>, consumed: usize) -> (i64, usize) {
	let mut idx : usize = consumed;
	let mut is_negative = false;

	while idx < content.len() && !(content[idx] as char).is_numeric() {
		is_negative = content[idx] == '-' as u8;
		idx += 1;
	}

	if content.len() == idx {
		return (0, idx);
	}

	let mut num : i64 = 0;

	loop {
		if idx == content.len() {
			break;
		}

		if !(content[idx] as char).is_numeric() {
			break;
		}

		num *= 10;
		num += (content[idx] as char).to_digit(10).unwrap() as i64;
		idx += 1;
	}

	if is_negative {
		num *= -1;
	}

	return (num, idx);
}

trait ComputeScore {
	fn compute(&self) -> i64;
}

impl ComputeScore for json::JsonValue {
	fn compute(&self) -> i64 {
		let mut sum : i64 = 0;
		// println!("{:#}", self);

		if self.is_object() {
			if self.entries().any(|(_x, y)| y == "red") {
				return 0;
			}
		}

		if self.is_number() {
			return self.as_number().unwrap().as_fixed_point_i64(0).unwrap();
		}

		if self.is_array() {
			for n in 0..self.len() {
				sum += self[n].compute();
			}
		}

		if self.is_object() {
			sum += self.entries().map(|(_x, y)| y.compute()).sum::<i64>();
		}

		sum
	}
}

fn main() {
	let content = fs::read_to_string("input.txt")
		.expect("Something went wrong reading the file");
	// let content = "{\"a\":\"red\"}".to_string();
	let mut consumed : usize = 0;

	let bytes : Vec<u8> = content.as_bytes().to_vec();
	let mut numbers : Vec<i64> = Vec::new();

	loop {
		let (num, got) = get_next_num(&bytes, consumed);
		numbers.push(num);
		consumed = got;

		if consumed == bytes.len() {
			break;
		}
	}

	println!("Part #1: {:?}", numbers.iter().fold(0, |x, y| x + y));

	let parsed = json::parse(&content);

	let total = match &parsed {
		Ok(ref n) => n.compute(),
		_ => panic!("failed to parse json.")
	};

	println!("Part #2: {:?}", total);
}
