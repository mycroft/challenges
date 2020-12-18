use std::str;

fn n_lexi_perm(mut xs: &mut [u8], n: usize) {
    let mut n = n;
    while !xs.is_empty() {
        let m: usize = (1 .. xs.len()).product();
        let y = n / m;
        xs[0 .. y + 1].reverse();
        xs[1 .. y + 1].reverse();
        let tmp = xs;
        xs = &mut tmp[1..];
        n %= m;
    }
}

fn main() {
    let mut s = *b"0123456789";
    n_lexi_perm(&mut s, 999_999);
    println!("{}", str::from_utf8(&s).unwrap());
}
