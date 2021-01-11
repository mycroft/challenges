
fn gethash(hashes: &mut Vec<String>, salt: &str, idx: usize) -> String {
    if hashes.len() > idx {
        return hashes[idx].clone();
    }

    for i in hashes.len()..=idx {
        let c = salt.clone().to_owned() + &i.to_string();
        let digest = md5::compute(c);
        let s : String = format!("{:?}", digest);

        hashes.push(s);
    }

    hashes[idx].clone()
}

fn getstreshedhash(hashes: &mut Vec<String>, salt: &str, idx: usize) -> String {
    if hashes.len() > idx {
        return hashes[idx].clone();
    }

    for i in hashes.len()..=idx {
        let c = salt.clone().to_owned() + &i.to_string();
        let mut digest = md5::compute(c);

        for _j in 0..2016 {
            let s : String = format!("{:?}", digest);
            digest = md5::compute(&s);
        }

        let s : String = format!("{:?}", digest);

        hashes.push(s);
    }

    hashes[idx].clone()
}

fn hasreps(_hash: &str, reps: usize, want_letter: Option<char>) -> Result<char, ()> {
    let letters : &[u8] = _hash.as_bytes();

    for (idx, letter) in letters.iter().enumerate() {
        if idx == _hash.len() - reps + 1{
            break;
        }

        let mut isvalid = true;
        for i in 1..reps {
            if want_letter != None && want_letter != Some(*letter as char) {
                continue;
            }

            if *letter != letters[idx + i] {
                isvalid = false;
                break;
            }
        }

        if isvalid && (want_letter == None || want_letter == Some(*letter as char)) {
            return Ok(*letter as char);
        }
    }

    Err(())
}

fn find(salt: &str, hash_func: fn(&mut Vec<String>, &str, usize) -> String) -> usize {
    let mut hashes : Vec<String> = vec![];
    let mut number = 0;
    let mut key_idx = 1;

    loop {
        let hash = hash_func(&mut hashes, salt, number);

        let mut key = 0;

        if let Ok(c) = hasreps(&hash, 3, None) {
            for i in 1..1000 {
                let sub_hash = hash_func(&mut hashes, salt, number + i);
                if let Ok(cs) = hasreps(&sub_hash, 5, Some(c)) {
                    if cs == c {
                        key = number + i;
                    }
                }
            }
        }

        if key != 0 {
            // println!("key_idx:{} number:{}", key_idx, number);
            if key_idx == 64 {
                break;
            }
            key_idx += 1;
        }

        number += 1;
    }

    number
}

fn main() {
    let salt = "cuanljph";

    println!("Part #1: {}", find(&salt, gethash));
    println!("Part #2: {}", find(&salt, getstreshedhash));
}

#[test]
fn example() {
    let mut hashes = vec![];
    assert_eq!(String::from("0034e0923cc38887a57bd7b1d4f953df"), gethash(&mut hashes, &"abc", 18));
    assert_eq!(Ok('8'), hasreps("0034e0923cc38887a57bd7b1d4f953df", 3, None));
    assert_eq!(Err(()), hasreps("0034e0923cc38787a57bd7b1d4f953df", 3, None));
    assert_eq!(Ok('8'), hasreps("0034e0923cc38887a57bd7b1d4f953df", 3, Some('8')));
    assert_eq!(Ok('8'), hasreps("0034e0923cc38987a57bd7b1d4f95888", 3, Some('8')));

    let mut hashes = vec![];
    assert_eq!(String::from("a107ff634856bb300138cac6568c0f24"), getstreshedhash(&mut hashes, &"abc", 0));
}

/*
Takes 3min40 to resolve
*/