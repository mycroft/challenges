/*
A palindromic number reads the same both ways. The largest palindrome made from
the product of two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
*/

fn palindrome(x: i32) -> bool {
	let mut orig = x;
	let mut y = 0;

	while orig > 0 {
		y *= 10;
		y += orig % 10;
		orig /= 10;
	}

	x == y
}

fn main() {
	// println!("{:?}", palindrome(1));
	// println!("{:?}", palindrome(11));
	// println!("{:?}", palindrome(111));
	// println!("{:?}", palindrome(9999));
	// println!("{:?}", palindrome(99099));
	// println!("{:?}", palindrome(99989));
	// println!("{:?}", palindrome(09999));

	let mut max = 0;

	let mut x = 999;
	let mut y;
	let mut z;

	while x > 1 {
		y = x;
		while y > 1 {
			if palindrome(x * y) {

				z = x * y;
				if max < z {
					max = z;
				}
			}
			y -= 1;
		}

		x -= 1;
	}

	println!("{:?}", max);
}