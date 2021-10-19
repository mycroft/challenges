extern crate rand;

use rand::Rng;

fn mod_pow(mut base: u128, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 { return 0 }
    let mut result = 1;
    base = base % modulus as u128;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus as u128;
        }
        exp = exp >> 1;
        base = (base * base) % modulus as u128
    }
    result as u64
}


pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2u64..p - 1)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_pow(g as u128, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_pow(b_pub as u128, a, p)
}
