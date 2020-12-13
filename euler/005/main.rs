/*
2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.

What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
*/

fn valid(x: i64) -> bool {
	let mut n = 20;

	while n > 1 {
		if x % n != 0 {
			return false;
		}

		n -= 1;
	}

	true
}

fn main() {
	let mut s = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;

	// solution is 5 * 7 * 9 * 11 * 11 * 13 * 16 * 19

	loop {
		if valid(s) {
			break
		}

		s += 19 * 2;
	}

	println!("{:?}", s);
}
