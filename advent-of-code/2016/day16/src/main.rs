fn step(c : &Vec<bool>) -> Vec<bool> {
    let mut out = c.clone();
    let mut copy = c.clone();

    copy.reverse();
    out.extend(vec![false]);
    
    for idx in 0..copy.len() {
        copy[idx] = !copy[idx];
    }

    out.extend(copy);
    out
}

fn checksum(c : &Vec<bool>) -> Vec<bool> {
    let mut cksum : Vec<bool> = vec![];

    if c.len() % 2 == 1 {
        return c.to_vec();
    };

    let mut idx = 0;

    loop {
        if idx == c.len() {
            break;
        }

        cksum.push(c[idx] == c[idx + 1]);
        idx += 2;
    }

    if cksum.len() % 2 == 0 {
        checksum(&cksum)
    } else {
        cksum
    }
}

fn str_to_bool(s: &str) -> Vec<bool> {
    s.chars().map(|c| c == '1').collect::<Vec<bool>>()
}

fn bool_to_str(z: &Vec<bool>) -> String {
    let mut out : String = String::from("");

    for v in z {
        if *v {
            out.push('1');    
        } else {
            out.push('0');
        }
    }

    out
}

fn find(input: &str, len: usize) -> Vec<bool> {
    let mut current = str_to_bool(input);

    loop {
        if current.len() >= len {
            break;
        }

        current = step(&current);        
    }

    checksum(&current[0..len].to_vec())
}

fn main() {
    let input = "10111100110001111";

    println!("Part #1: {}", bool_to_str(&find(input, 272)));
    println!("Part #2: {}", bool_to_str(&find(input, 35651584)));
}

#[test]
fn name() {
    assert_eq!(vec![true, false, false], step(&vec![true]));

    assert_eq!(vec![true, false, false], str_to_bool("100"));
    assert_eq!("100", bool_to_str(&vec![true, false, false]));
}


/*
For example, suppose we want to fill a disk of length 12, and when we finally generate a string of at least length 12, the first 12 characters are 110010110100. To generate its checksum:

Consider each pair: 11, 00, 10, 11, 01, 00.
These are same, same, different, same, different, same, producing 110101.
The resulting string has length 6, which is even, so we repeat the process.
The pairs are 11 (same), 01 (different), 01 (different).
This produces the checksum 100, which has an odd length, so we stop.


*/

/*
For example, after a single step of this process,

1 becomes 100.
0 becomes 001.
11111 becomes 11111000000.
111100001010 becomes 1111000010100101011110000.

*/