use mkz_aoc::{file,parse};

fn solve1(input: String) -> usize {
    let mut list = parse::string_to_usize_vec(input);
    list.push(list[0]);

    list.iter().enumerate().filter(|(i, x)| *i != list.len() - 1 && *x == &list[i + 1]).map(|(_i, x)| x).sum()
}

fn solve2(input: String) -> usize {
    let list = parse::string_to_usize_vec(input);

    list[0..list.len()/2]
        .iter()
        .enumerate()
        .filter(|(i, x)| *x == &list[list.len()/2 + i])
        .map(|(_i, x)| x)
        .sum::<usize>() * 2
}

fn main() {
    let contents = file::read_to_string("input.txt").unwrap();

    println!("Part #1: {}", solve1(contents.to_string()));
    println!("Part #2: {}", solve2(contents.to_string()));
}

#[test]
fn test_solve1() {
    assert_eq!(3, solve1(String::from("1122")));
    assert_eq!(4, solve1(String::from("1111")));
    assert_eq!(0, solve1(String::from("1234")));
    assert_eq!(9, solve1(String::from("91212129")));
}

#[test]
fn test_solve2() {
    assert_eq!(6, solve2(String::from("1212")));
    assert_eq!(0, solve2(String::from("1221")));
    assert_eq!(4, solve2(String::from("123425")));
    assert_eq!(12, solve2(String::from("123123")));
    assert_eq!(4, solve2(String::from("12131415")));
}
