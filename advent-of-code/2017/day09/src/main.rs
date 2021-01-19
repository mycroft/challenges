use mkz_aoc::file;

fn compute(bytes: &[u8]) -> (usize, usize) {
    let mut idx = 0;
    let mut level = 0;
    let mut total = 0;

    let mut is_garbage = false;
    let mut total_garbage = 0;

    loop {
        if idx >= bytes.len() {
            break;
        }

        if bytes[idx] == '!' as u8 {
            idx += 2;
            continue;
        }

        if is_garbage {
            if bytes[idx] == '>' as u8 {
                is_garbage = false;
            } else {
                total_garbage += 1;
            }
        } else {
            if bytes[idx] == '{' as u8 {
                level += 1;
            }

            if bytes[idx] == '}' as u8 {
                total += level;
                level -= 1;
            }
        }

        if !is_garbage && bytes[idx] == '<' as u8 {
            is_garbage = true;
        }

        // println!("{:?} garbage:{}", bytes[idx] as char, is_garbage);

        idx += 1;
    }

    return (total, total_garbage);
}

fn main() {
    let line = file::read_to_string("input.txt").unwrap();
    let bytes = line.as_bytes();

    let res = compute(bytes);

    println!("Part #1: {}", res.0);
    println!("Part #2: {}", res.1);
}

#[test]
fn example() {
    assert_eq!(0, compute("<>".as_bytes()).1);
    assert_eq!(17, compute("<random characters>".as_bytes()).1);
    assert_eq!(3, compute("<<<<>".as_bytes()).1);
    assert_eq!(2, compute("<{!>}>".as_bytes()).1);
    assert_eq!(0, compute("<!!>".as_bytes()).1);
    assert_eq!(0, compute("<!!!>>".as_bytes()).1);
    assert_eq!(10, compute("<{ozi!a,<{i<a>".as_bytes()).1);

}