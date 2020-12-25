fn check(p1 : &Vec<String>, p2 : &Vec<String>, password : &String) -> bool {

	if !p1.iter().any(|x| password.contains(x)) {
		return false;
	}


	if password.chars().any(|x| x == 'i' || x == 'o' || x == 'l') {
		return false;
	}

	let matches : Vec<&String> = p2
		.iter()
		.filter(|x| password.contains(*x))
		.collect()
		;

	if matches.len() != 2 {
		return false;
	}

	true
}

fn next(password : &String) -> String {
	let mut z : Vec<char> = password.chars().collect();
	let mut idx = z.len() - 1;

	loop {
		if z[idx] == 'z' {
			z[idx] = 'a';
			idx -= 1;
		} else {
			z[idx] = (z[idx] as u8 + 1) as char;
			break;
		}
	}

	z.iter().collect::<String>()
}

fn main() {
	let mut input = "hepxcrrq".to_string();

	let mut pattern1 : Vec<String> = vec![];
	for i in 0..24 {
		let mut s = "".to_string();
		let c = 'a' as u8 + i;
		s.push(c as char);
		s.push((c+1) as char);
		s.push((c+2) as char);

		pattern1.push(s);
	}

	let mut pattern2 : Vec<String> = vec![];
	for i in 0..26 {
		let mut s = "".to_string();
		let c = 'a' as u8 + i;
		s.push(c as char);
		s.push(c as char);

		pattern2.push(s);
	}

	loop {
		input = next(&input);

		if check(&pattern1, &pattern2, &input) {
			break;
		}
	}

	println!("password is valid: {:?}", input);

	loop {
		input = next(&input);

		if check(&pattern1, &pattern2, &input) {
			break;
		}
	}

	println!("password is valid: {:?}", input);
}
