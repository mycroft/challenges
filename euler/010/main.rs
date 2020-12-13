/*
The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.

Find the sum of all the primes below two million.
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
	let primes = simple_sieve(2_000_000);
	let mut sum : u64 = 0;

	for item in primes {
		sum += item as u64;
		// println!("{:?}", item);
	}
	//println!("{:?}", primes);
	println!("{:?}", sum);
}
