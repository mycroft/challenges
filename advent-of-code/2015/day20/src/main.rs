use divisors::get_divisors;

fn compute(n : u32) -> u32 {
	let mut divisors = get_divisors(n);
	divisors.push(1);
	if !divisors.iter().any(|x| *x == n) {
		divisors.push(n);
	}

	divisors.iter().map(|x| x * 10).sum()
}

fn compute2(n : u32) -> u32 {
	let mut divisors = get_divisors(n);
	divisors.push(1);
	if !divisors.iter().any(|x| *x == n) {
		divisors.push(n);
	}

	divisors.iter().filter(|x| *x * 50 >= n).map(|x| x * 11).sum()
}

fn main() {
	let mut house_number = 1;
	let mut gifts;
	let limit = 33100000;

	compute2(500);

	loop {
		gifts = compute(house_number);
		if gifts >= limit {
			break;
		}

		house_number += 1;
	}

	println!("{:?} => {}", house_number, gifts);

	house_number = 1;

	loop {
		gifts = compute2(house_number);
		if gifts >= limit {
			break;
		}

		house_number += 1;
	}

	println!("{:?} => {}", house_number, gifts);
}

#[test]
fn example() {
	assert_eq!(compute(1), 10);
	assert_eq!(compute(2), 30);
	assert_eq!(compute(3), 40);
	assert_eq!(compute(4), 70);
	assert_eq!(compute(5), 60);
	assert_eq!(compute(6), 120);
	assert_eq!(compute(7), 80);
	assert_eq!(compute(8), 150);
	assert_eq!(compute(9), 130);
}
