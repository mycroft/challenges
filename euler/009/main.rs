/*
A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,

a^2 + b^2 = c^2

For example, 32 + 42 = 9 + 16 = 25 = 52.

There exists exactly one Pythagorean triplet for which a + b + c = 1000.
Find the product abc.
*/

fn main() {
	for i in 1..997 {
		for j in i..(1000 - i - 1) {
			let k = 1000 - i - j;

			if i * i + j * j == k * k {
				println!("i:{} j:{} k:{} i*j*k:{}", i, j, k, i * j * k);
			}
		}
	}
}