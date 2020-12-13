/*
By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.

What is the 10 001st prime number?
*/

// https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
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
	let primes = simple_sieve(1000000);
	println!("{:?}", primes[10000]);
}