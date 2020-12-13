/*
2^15 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

What is the sum of the digits of the number 2^1000?
*/

use num::bigint::BigUint;

fn main() {
	let n = BigUint::new(vec![2]);

	println!("{:?}",
		n
			.pow(1000)
			.to_str_radix(10)
			.chars()
			.filter_map(|x| x.to_digit(10))
			.sum::<u32>()
	);
}