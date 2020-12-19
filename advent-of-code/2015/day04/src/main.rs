
fn main() {
	let s = "iwrupvqb".to_string();
	let mut n = 0;

	loop {
		let c = s.clone() + &n.to_string();

		let digest = md5::compute(c);
		let s : String = format!("{:?}", digest);

		// Change to 5 & "00000" to get first part.
		if &s[..6] == "000000" {
			println!("{:?}", n);
			break;
		}

		n += 1;
	}
}

// Part 1: 346386
// Part 2: 9958218

/*
let digest = md5::compute(b"abcdefghijklmnopqrstuvwxyz");
assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
*/
