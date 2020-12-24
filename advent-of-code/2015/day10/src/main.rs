fn read(s : Vec<u8>) -> Vec<u8> {
	let mut index = 0;

	let mut current_letter = s[0];
	let mut iteration = 0;

	let mut out : Vec<u8> = Vec::new();

	loop {
		if index == s.len() {
			out.push(iteration);
			out.push(current_letter);
			break;
		}

		if current_letter != s[index] {
			out.push(iteration);
			out.push(current_letter);
			iteration = 0;
			current_letter = s[index];
		}

		iteration += 1;
		index += 1;
	}

	out
}

fn main() {
	let mut iteration = 0;
	let mut input : Vec<u8> = "1113222113".chars().map(|x| x.to_digit(10).unwrap() as u8).collect();

	loop {
		input = read(input);
		iteration += 1;

		if iteration == 40 {
			println!("40: {}", input.len());
		}

		if iteration == 50 {
			println!("50: {}", input.len());
			break;
		}
	}
}
