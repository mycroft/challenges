/*
The sum of the squares of the first ten natural numbers is,

1^2 + 2^2 + ... + 10^2 = 385

The square of the sum of the first ten natural numbers is,

(1 + 2 + ... + 10)^2 = 55^2 = 3025

Hence the difference between the sum of the squares of the first ten natural numbers
and the square of the sum is

3025 - 385 = 2640

Find the difference between the sum of the squares of the first one hundred natural number
and the square of the sum.
*/

fn main() {
	let mut s : u128 = 0;
	let mut s2 : u128 = 0;
	let mut i : u128 = 1;

	while i <= 100 {
		s += i.pow(2);
		s2 += i;

		i += 1;
	}

	s2 = s2.pow(2);

	println!("{:?} - {:?} = {:?}", s2, s, s2 - s);
}