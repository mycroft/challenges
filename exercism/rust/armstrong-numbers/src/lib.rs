pub fn is_armstrong_number(num: u32) -> bool {
	let str = num.to_string();
	num == str
		.chars()
		.map(|x| x
			.to_digit(10)
			.unwrap()
			.pow(str.len() as u32)
		)
		.sum::<u32>()
}
