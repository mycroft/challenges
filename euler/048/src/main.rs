/*
The series, 1^1 + 2^2 + 3^3 + ... + 10^10 = 10405071317.

Find the last ten digits of the series, 1^1 + 2^2 + 3^3 + ... + 1000^1000.
*/

use num::bigint::BigUint;

fn ppp(n : u128) -> u128 {
	let mut r = 1;
	let mut i = 0;

	while i < n {
		r = (r * n) % 100000000000;

		i += 1;
	}

	return r;
}

fn main() {
	let mut b = BigUint::new(vec![0]);
	let mut n : u128 = 1;

	while n < 1000 {
		b += ppp(n);

		n += 1;
	}

	let s = b.to_str_radix(10);

	println!("{}", &s[s.len() - 10..]);
}