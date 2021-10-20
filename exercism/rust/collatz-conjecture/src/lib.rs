pub fn collatz(n: u64) -> Option<u64> {
    let mut n : u128 = n as u128;

    if n == 0 {
        None
    } else {
        let mut step = 0;

        loop {
            if n == 1 {
                return Some(step)
            } else {
                if n % 2 == 0 {
                    n = n / 2
                } else {
                    n = 3 * n + 1
                }

                step += 1
            }

            if n > u64::MAX as u128 {
                return None
            }
        }
    }
}
