/*
The following iterative sequence is defined for the set of positive integers:

n → n/2 (n is even)
n → 3n + 1 (n is odd)

Using the rule above and starting with 13, we generate the following sequence:

13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

Which starting number, under one million, produces the longest chain?

NOTE: Once the chain starts the terms are allowed to go above one million.
*/

use std::collections::HashMap;

fn get(n : u64, cache : &mut HashMap<u64, u64>) -> u64 {
	let ret;

	match cache.get(&n) {
		Some(number) => {
			return *number
		},
		_ => {
		}
	}

	if n % 2 == 0 {
		ret = 1 + get(n / 2, cache);
	} else {
		ret = 1 + get(3 * n + 1, cache)
	}

	cache.insert(n, ret);

	ret
}

fn main() {

	let mut cache = HashMap::new();

	cache.insert(0, 0);
	cache.insert(1, 1);

	let mut n = 1_000_000;
	let mut max_path_size = 0;
	while n > 1 {
		let path_size = get(n, &mut cache);

		if path_size > max_path_size {
			max_path_size = path_size;
			println!("n: {} has path size {}", n, path_size);
		}

		n -= 1;

	}
}