/*
n! means n × (n − 1) × ... × 3 × 2 × 1

For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

Find the sum of the digits in the number 100!
*/

use num::bigint::BigUint;

fn main() {
	let mut n : u32 = 100;
	let mut f = BigUint::parse_bytes(b"1", 10).unwrap();

	while n > 1 {
		f *= n;
		n -= 1;
	}

	let sum : u32 = f
		.to_str_radix(10)
		.chars()
		.map(|x| x.to_digit(10).unwrap())
		.sum();

	println!("{:?}", sum);
}
