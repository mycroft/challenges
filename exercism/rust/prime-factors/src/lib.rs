pub fn factors(n: u64) -> Vec<u64> {
    let mut current = n;
    let mut z = 2;
    let mut v : Vec<u64> = vec![];

    while current != 1 {
        if current % z == 0 {
            v.push(z);
            current = current / z;
        } else {
            z += 1;
        }
    }

    v
}
