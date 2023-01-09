use std::fs::read_to_string;

fn compute(input: &[isize], offset: usize) -> Vec<isize> {
    let pattern = [0, 1, 0, -1];
    let mut result = Vec::new();

    for idx0 in 0..input.len() {
        let mut r = 0;
        for (idx1, el) in input.iter().enumerate() {
            let b = ((offset + idx1) / (idx0 + 1)) % pattern.len();
            // println!("{idx0} {idx1} / el:{el} b:{b}");

            r += el * pattern[b];
        }

        result.push(r.abs() % 10);
        // println!();
    }

    result
}

fn phases(input: &[isize], offset: usize, phases: usize) -> Vec<isize> {
    let mut result = input.to_owned();
    for _ in 0..phases {
        result = compute(&result, offset);
    }

    result
}

fn bignum_to_vec(s: &str) -> Vec<isize> {
    s.chars().map(|x| x.to_digit(10).unwrap() as isize).collect()
}

fn phase2(input: &Vec<isize>) -> Vec<isize> {
    let start = input[..7].iter().fold(0, |acc, x| acc * 10 + x) as usize;
    let end = input.len() * 10_000;

    // all numbers are after the middle, which means the coefficients will be only one.
    let mut numbers = Vec::new();
    for idx in start..end {
        numbers.push(input[idx % input.len()]);
    }

    for _ in 0..100 {
        let mut cumsum = 0;
        // we're going reverse as the last number is itself, the number before equals the last + itself, etc.
        for idx in (0..numbers.len()).rev() {
            cumsum += numbers[idx];
            numbers[idx] = cumsum % 10;
        }
    }

    numbers[..8].to_vec()
}

fn arr_to_num(nums: &[isize]) -> i64 {
    nums.iter().fold(0, |acc, &x| acc * 10 + x as i64)
}

fn main() {
    let contents = read_to_string("input.txt").expect("a file to open");
    let contents = contents.trim();
    let nums = bignum_to_vec(contents);
    let result = phases(&nums, 1, 100)[..8].to_vec();
    println!("#1 {}", arr_to_num(&result)); // 37153056
    println!("#2 {}", arr_to_num(&phase2(&nums))); // 60592199
}

#[test]
fn test_sample() {
    assert_eq!(
        [0, 1, 0, 2, 9, 4, 9, 8].to_vec(),
        phases(
            &[1, 2, 3, 4, 5, 6, 7, 8].to_vec(),
            1,
            4,
        )
    );

    assert_eq!(
        bignum_to_vec("24176176"),
        phases(
            &bignum_to_vec("80871224585914546619083218645595"),
            1,
            100,
        )[..8]
    );

    assert_eq!(
        bignum_to_vec("73745418"),
        phases(
            &bignum_to_vec("19617804207202209144916044189917"),
            1,
            100,
        )[..8]
    );

    assert_eq!(
        bignum_to_vec("52432133"),
        phases(
            &bignum_to_vec("69317163492948606335995924319873"),
            1,
            100,
        )[..8]
    );
}

#[test]
fn test_sample_step2() {
    assert_eq!(
        bignum_to_vec("84462026"),
        phase2(
            &bignum_to_vec("03036732577212944063491565474664"),
        )
    );

    assert_eq!(
        bignum_to_vec("78725270"),
        phase2(
            &bignum_to_vec("02935109699940807407585447034323"),
        )
    );

    assert_eq!(
        bignum_to_vec("53553731"),
        phase2(
            &bignum_to_vec("03081770884921959731165446850517"),
        )
    );
}
