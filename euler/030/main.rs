fn main() {
	let mut total = 0;

	for n in 2..10000000 {
		let m : u32 = n
			.to_string()
			.chars()
			.map(|x| x.to_digit(10).unwrap().pow(5))
			.sum()
		;

		if n == m {
			// println!("{:?} {:?}", m, n);
			total += m;
		}
	}

	println!("{:?}", total);
}