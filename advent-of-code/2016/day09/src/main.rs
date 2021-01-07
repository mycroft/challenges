use std::fs;

fn take_while(accept: impl Fn(char) -> bool, s: &str) -> (&str, &str) {
    let extracted_end = s
        .char_indices()
        .find_map(|(idx, c)| if accept(c) { None } else { Some(idx) })
        .unwrap_or_else(|| s.len());

    let extracted = &s[..extracted_end];
    let remainder = &s[extracted_end..];
    (remainder, extracted)
}

fn expand(_contents : &String, use_recursive: bool) -> usize {
    let mut size = 0;
    let mut _idx = 0;

    let mut s = _contents.as_str();

    while s.len() > 0 {
        let (sn, prefix) = take_while(|c| c.is_alphabetic(), &s);
        size += prefix.len();

        if sn.len() == 0 || sn.chars().nth(0).unwrap() != '(' {
            break;
        }

        let (sn, _) = take_while(|c| c == '(', &sn);
        let (sn, x) = take_while(|c| c.is_numeric(), &sn);
        let (sn, _) = take_while(|c| c == 'x', &sn);
        let (sn, y) = take_while(|c| c.is_numeric(), &sn);
        let (sn, _) = take_while(|c| c == ')', &sn);

        let x = x.parse::<usize>().unwrap();
        let y = y.parse::<usize>().unwrap();

        if use_recursive {
            size += y * expand(&sn[0..x].to_string(), true);
        } else {
            size += y * x
        }

        s = &sn[x..];
    }

    size
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    println!("Part #1: {:?}", expand(&contents, false));
    println!("Part #2: {:?}", expand(&contents, true));
}

#[test]
fn example() {
    assert_eq!(expand(&String::from("ADVENT".to_string()), false), 6);
    assert_eq!(expand(&String::from("A(1x5)BC".to_string()), false), 7);
    assert_eq!(expand(&String::from("(3x3)XYZ".to_string()), false), 9);
    assert_eq!(expand(&String::from("A(2x2)BCD(2x2)EFG".to_string()), false), 11);
    assert_eq!(expand(&String::from("(6x1)(1x3)A".to_string()), false), 6);
    assert_eq!(expand(&String::from("X(8x2)(3x3)ABCY".to_string()), false), 18);

    assert_eq!(expand(&String::from("(3x3)XYZ".to_string()), true), 9);
    assert_eq!(expand(&String::from("X(8x2)(3x3)ABCY".to_string()), true), 20);
    assert_eq!(expand(&String::from("(27x12)(20x12)(13x14)(7x10)(1x12)A".to_string()), true), 241920);
    assert_eq!(expand(&String::from("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN".to_string()), true), 445);
}
