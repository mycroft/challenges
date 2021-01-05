use std::fs;

fn has_abba_pattern(string : &str) -> bool {
    let stringb = string.as_bytes();
    let stringlen = stringb.len();

    for i in 0..(stringlen - 3) {
        if stringb[i] == stringb[i + 3] && stringb[i + 1] == stringb[i + 2] && stringb[i] != stringb[i + 1] {
            return true;
        }
    }

    false
}

fn get_aba_patterns(string : &str) -> Vec<&str> {
    let stringb = string.as_bytes();
    let stringlen = stringb.len();

    let mut ret : Vec<&str> = vec![];

    for i in 0..(stringlen - 2) {
        if stringb[i] == stringb[i + 2] && stringb[i] != stringb[i + 1] {
            ret.push(&string[i..=i+2]);
        }
    }

    ret

}

fn is_tls_valid(string : &str) -> bool {
    let v: Vec<&str> = string.rsplit(|c| c == '[' || c == ']').collect();

    let mut is_abba_valid = false;
    let mut is_hypernet_valid = true;

    for (i, p) in v.iter().enumerate() {
        if i % 2 == 0 {
            if has_abba_pattern(&p) {
                is_abba_valid = true;
            }
        } else {
            if has_abba_pattern(&p) {
                is_hypernet_valid = false;
            }
        }
    }

    is_hypernet_valid && is_abba_valid
}

fn invert(_string: &str) -> String {
    format!("{}{}{}",
        _string.chars().nth(1).unwrap(),
        _string.chars().nth(0).unwrap(),
        _string.chars().nth(1).unwrap()
    )
}


fn is_ssl_valid(string : &str) -> bool {
    let v: Vec<&str> = string.rsplit(|c| c == '[' || c == ']').collect();

    let mut v1 : Vec<&str> = vec![];
    let mut v2 : Vec<&str> = vec![];

    for (i, p) in v.iter().enumerate() {
        let v_tmp = get_aba_patterns(&p);
        if i % 2 == 0 {
            for v_elem in v_tmp {
                v1.push(v_elem);
            }
        } else {
            for v_elem in v_tmp {
                v2.push(v_elem);
            }
        }
    }

    for v_elem in v2 {
        let v_elem = invert(&v_elem);
        if v1.iter().any(|x| *x == v_elem) {
            return true;
        }
    }

    false
}



fn main() {
    let _contents = fs::read_to_string("input.txt").unwrap();

    let _lines = _contents.lines();

    let mut tls_valid = 0;
    let mut ssl_valid = 0;

    for line in _lines {
        if is_tls_valid(line) {
            tls_valid += 1;
        }

        if is_ssl_valid(line) {
            ssl_valid += 1;
        }
    }

    println!("Part #1: {}", tls_valid);
    println!("Part #2: {}", ssl_valid);
}

#[test]
fn test_has_abba_pattern() {
    assert_eq!(true, has_abba_pattern("abba"));
    assert_eq!(true, has_abba_pattern("ioxxoj"));
}

#[test]
fn test_is_tls_valid() {
    assert_eq!(true, is_tls_valid("abba[mnop]qrst"));
    assert_eq!(false, is_tls_valid("abcd[bddb]xyyx"));
    assert_eq!(false, is_tls_valid("aaaa[qwer]tyui"));
    assert_eq!(true, is_tls_valid("ioxxoj[asdfgh]zxcvbn"));
}

#[test]
fn test_get_aba_patterns() {
    assert_eq!(true, get_aba_patterns("aba").len() > 0);
    assert_eq!(true, get_aba_patterns("xyx").len() > 0);
    assert_eq!(true, get_aba_patterns("zazbz").len() > 0);
    assert_eq!(true, get_aba_patterns("abc").len() == 0);
}

#[test]
fn test_is_ssl_valid() {
    assert_eq!(true, is_ssl_valid("aba[bab]xyz"));
    assert_eq!(true, is_ssl_valid("aaa[kek]eke"));
    assert_eq!(true, is_ssl_valid("zazbz[bzb]cdb"));
    assert_eq!(false, is_ssl_valid("xyx[xyx]xyx"));
}
