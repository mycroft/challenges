/*
The prime factors of 13195 are 5, 7, 13 and 29.

What is the largest prime factor of the number 600851475143 ?
*/

fn main() {
	let mut numb : i64 = 600851475143;
	let mut div = 2;

	while div < numb {
		if numb % div == 0 {
			numb = numb / div;
			continue
		}

		div += 1;
	}

	println!("div: {}", div);
}