
// https://fr.wikipedia.org/wiki/Loi_binomiale

fn partialfact(n : u128, min : u128) -> u128 {
	if n <= min {
		1
	} else {
		n * partialfact(n - 1, min)
	}
}

fn main() {
	let n = 20;
	let res : u128 = partialfact(2 * n, n) / partialfact(n, 1);

	println!("{:?}",
		res
	);
}
