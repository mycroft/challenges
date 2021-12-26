use std::collections::VecDeque;

pub fn decode_ascii85(orig: &String) -> Vec<u8> {
    if !orig.starts_with("<~") {
        panic!("Ascii85 string must start with <~.");
    }

    // if !orig.ends_with("~>")  {
    //     panic!("Ascii85 string must end with ~>.");
    // }

    let mut out = vec![];
    let mut orig: VecDeque<char> = orig
        .trim_start_matches("<~")
        .trim_end_matches("~>")
        .chars()
        .collect();

    let mut count: u32 = 0;
    let mut val: u32 = 0;

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
