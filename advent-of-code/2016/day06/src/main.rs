use std::fs;
use std::collections::HashMap;

fn main() -> Result<(), std::io::Error> {
    let _contents = fs::read_to_string("input.txt")?;
    let _lines = _contents.lines();
    let mut _hm : Vec<HashMap<char, usize>> = vec![];
    let mut s = String::new();
    let mut s2 = String::new();

    for _ in 0..8 {
        _hm.push(HashMap::new());
    }

    for line in _lines {
        for (i, c) in line.chars().enumerate() {
            *_hm[i].entry(c).or_insert(0) += 1;
        }
    }

    for i in 0..8 {
        let top_char = _hm[i].iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        let least_char = _hm[i].iter().max_by(|b, a| a.1.cmp(&b.1)).unwrap();
        s.push(*top_char.0);
        s2.push(*least_char.0);

    }

    println!("Part #1: {}", s);
    println!("Part #2: {}", s2);

    Ok(())
}
