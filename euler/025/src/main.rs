use num_bigint::BigUint;
use num_traits::One;

fn main() {
	let mut m : Vec<BigUint> = Vec::new();
	let mut rank = 2;

	m.push(One::one());
	m.push(One::one());

	loop {
		let f = &m[m.len() - 1] + &m[m.len() - 2];
		rank += 1;

		if f.to_str_radix(10).len() >= 1000 {
			println!("{:?}", f.to_str_radix(10));
			println!("{:?}", rank);
			break;
		}

		m.push(f);
	}
}
