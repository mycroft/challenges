pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2];
    let mut current = primes[primes.len() - 1] + 1;

    while n >= primes.len() as u32 {
        let mut prime = true;

        for v in primes.iter() {
            if *v > (current + 1) / 2 {
                break;
            }

            if current % v == 0 {
                prime = false;
                break;
            }
        }

        if prime {
            primes.push(current);
        }

        current += 2;
    }

    primes[n as usize]
}
