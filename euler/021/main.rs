/*
Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide
evenly into n).
If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b
are called amicable numbers.

For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110;
therefore d(220) = 284.
The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

Evaluate the sum of all the amicable numbers under 10000.

*/
fn simple_sieve(limit: usize) -> Vec<usize> {
 
    let mut is_prime = vec![true; limit+1];
    is_prime[0] = false;
    if limit >= 1 { is_prime[1] = false }
 
    for num in 2..limit+1 {
        if is_prime[num] {
            let mut multiple = num*num;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += num;
            }
        }
    }
 
    is_prime.iter().enumerate()
        .filter_map(|(pr, &is_pr)| if is_pr {Some(pr)} else {None} )
        .collect()
}


fn main() {
	let primes = simple_sieve(10000);
	let mut num = 1;
	let mut amicable : Vec<usize> = vec![0];

	while num < 10_000 {
		let mut n = num;
		let mut div : Vec<usize> = vec![1];

		for prime in &primes {
			while n % prime == 0 {
				let mut new_div : Vec<usize> = vec![1];

				for d in &div {
					new_div.push(d * *prime);
				}

				n = n / prime;

				for d in new_div.into_iter() {
					if !div.contains(&d) && d < num {
						div.push(d);
					}
				}
			}

			if n == 1 {
				break;
			}
		}

		amicable.push(div.iter().sum::<usize>());

		num += 1;
	}

	num = 1;
	let mut total = 0;

	while num < 10_000 {
		if amicable[num] < 10_000 && amicable[amicable[num]] == num && num != amicable[num] {
			println!("{:?} {:?} {:?}", num, amicable[num], amicable[amicable[num] as usize]);
			total += num;
		}

		num += 1;
	}

	println!("total:{}", total);
}