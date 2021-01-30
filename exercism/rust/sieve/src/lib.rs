pub fn primes_up_to(limit: u64) -> Vec<u64> { 
    let mut is_prime = vec![true; (limit+1) as usize];

    is_prime[0] = false;
    if limit >= 1 { is_prime[1] = false }
 
    for num in 2..limit+1 {
        if is_prime[num as usize] {
            let mut multiple = num*num;
            while multiple <= limit {
                is_prime[multiple as usize] = false;
                multiple += num;
            }
        }
    }
 
    is_prime.iter().enumerate()
        .filter_map(|(pr, &is_pr)| if is_pr {Some(pr as u64)} else {None} )
        .collect()
}
