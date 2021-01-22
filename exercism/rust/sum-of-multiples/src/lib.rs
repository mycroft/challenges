pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut res = 0;

    for z in 1..limit {
        for f in factors.into_iter() {
            if &0 != f && z % f == 0 {
                res += z;
                break;
            }
        }
    }

    res
}
