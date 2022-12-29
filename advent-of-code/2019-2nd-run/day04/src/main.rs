use std::ops::RangeInclusive;

fn is_matching(r: &RangeInclusive<u32>, n: u32, step2: bool) -> bool {
    if !r.contains(&n) {
        return false;
    }

    let mut n = n;
    let mut v = [0; 10];
    let mut current = 10;

    while n > 0 {
        let r = n % 10;
        if r > current {
            return false;
        }
        v[r as usize] += 1;
        current = r;
        n = (n - r) / 10;
    }

    let has_group = v.iter().filter(|&&x| x >= 2).count() >= 1;
    let has_double = v.iter().filter(|&&x| x == 2).count() >= 1;

    if step2 {
        has_double 
    } else {
        has_group
    }
}

fn input_to_range(s: &str) -> RangeInclusive<u32> {
    let parts = s.split("-").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

    RangeInclusive::new(
        parts[0],
        parts[1],
    )
}

fn main() {
    let puzzle_input = "130254-678275";
    let range = input_to_range(puzzle_input);
    let mut result_step1 = 0;
    let mut result_step2 = 0;

    for idx in range.clone() {
        if is_matching(&range, idx, false) {
            result_step1 += 1;
        }
        if is_matching(&range, idx, true) {
            result_step2 += 1;
        }
    }

    println!("#1 {result_step1}"); // 2090
    println!("#2 {result_step2}"); // 1419
}

#[test]
fn test_sample_step1() {
    let range = input_to_range("100000-999999");

    assert!(is_matching(&range, 111111, false));
    assert!(!is_matching(&range, 223450, false));
    assert!(!is_matching(&range, 123789, false));
    assert!(is_matching(&range, 123444, false));
}

#[test]
fn test_sample_step2() {
    let range = input_to_range("100000-999999");

    assert!(is_matching(&range, 111111, true));
    assert!(!is_matching(&range, 223450, true));
    assert!(!is_matching(&range, 123789, true));
    assert!(!is_matching(&range, 123444, true));

    assert!(is_matching(&range, 112233, true));
    assert!(!is_matching(&range, 123444, true));
    assert!(!is_matching(&range, 123789, true));
    assert!(is_matching(&range, 111122, true));

    assert!(!is_matching(&range, 566999, true));
}