use std::collections::HashSet;
use num::bigint::BigUint;
use num::pow::pow;

fn main() {
    let mut number = HashSet::new();

    for n in 2..=100 {
        for e in 2..=100 {
            let num = pow(BigUint::from(n as u32), e);

            if !number.contains(&num) {
                number.insert(num);
            }
        }
    }

    println!("{:?}", number.len());
}
