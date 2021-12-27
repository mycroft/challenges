use std::collections::VecDeque;

pub fn decode_ascii85(orig: &str) -> Vec<u8> {
    if !orig.starts_with("<~") {
        panic!("Ascii85 string must start with <~.");
    }

    let mut out = vec![];
    let mut orig: VecDeque<char> = orig
        .trim_start()
        .trim_start_matches("<~")
        .trim_end()
        .trim_end_matches("~>")
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .collect();

    let mut count: u32 = 0;
    let mut val: u32 = 0;

    let mut added = 0;

    // Adding missing 'u' at the end.
    while orig.len() % 5 != 0 {
        orig.push_back('u');
        added += 1;
    }

    loop {
        if orig.is_empty() {
            // no more char to ingest.
            break;
        }

        // Consome one char & convert it to bits
        let c = orig.pop_front().unwrap();
        let c_bits = c as u8 - 33;

        val += c_bits as u32 * 85u32.pow(4 - count);

        if count != 4 {
            count += 1;
            continue;
        }

        let mut c: u8 = 0;

        for idx in 0..32 {
            if idx != 0 && idx % 8 == 0 {
                out.push(c);

                c = 0;
            }

            let idx = 32 - idx - 1;

            c <<= 1;
            c += (val & 1 << idx == 1 << idx) as u8;
        }

        out.push(c);

        count = 0;
        val = 0;
    }

    // Removing last 'added'.
    for _ in 0..added {
        out.pop();
    }

    out
}

#[test]
fn test_ascii85() {
    let orig = String::from("<~6Y.B[~>");
    let res = decode_ascii85(&orig);

    assert_eq!(String::from("Ce q"), String::from_utf8(res).unwrap());

    let orig = String::from("<~6Y.B[/cYkO~>");
    let res = decode_ascii85(&orig);

    assert_eq!(String::from("Ce q.\0\0\0"), String::from_utf8(res).unwrap());
}
