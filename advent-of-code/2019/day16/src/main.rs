use std::fs::read_to_string;

fn str2numbers(s: &str, iterations: usize) -> Vec<usize> {
    let mut res = vec![];
    for _ in 0..iterations {
        let mut tmp = s.chars().map(|x| x.to_digit(10).expect("digit") as usize).collect::<Vec<usize>>();
        res.append(&mut tmp);
    }
    res
}

fn file2numbers(fp: &str, iterations: usize) -> Vec<usize> {
    let contents = read_to_string(fp).expect("file");
    str2numbers(contents.trim(), iterations)
}

fn iteration(numbers: Vec<usize>) -> Vec<usize> {
    let mut signals = vec![];
    let multipliers = [0isize, 1, 0, -1];
    let length = numbers.len();

    for z in 0..length {
        let mut signal : isize = 0;
        for (it, &number) in numbers.iter().enumerate() {
            signal += multipliers[(((it + 1)/(z+1)) % 4)] * number as isize;
        }

        signals.push((signal.abs() % 10) as usize);
    }

    signals
}

fn phases_part2(numbers: Vec<usize>, phase_n: usize) -> Vec<usize> {
    let offset = numbers[0..7].iter().fold(0, |total, i| total * 10 + i);
    let mut numbers = numbers[offset..].to_vec();
    let length = numbers.len();

    for _ in 0..phase_n {
        let mut cusum = 0;
        for idx in (0..length).rev() {
            cusum += numbers[idx];
            numbers[idx] = cusum % 10;
        }
    }

    numbers[0..8].to_vec()
}

fn phases(numbers: Vec<usize>, phase_n: usize, is_step2: bool) -> String {
    let mut numbers = numbers;

    if is_step2 {
        numbers = phases_part2(numbers, phase_n);
    } else {
        for _ in 0..phase_n {
            numbers = iteration(numbers);
        }    
    }

    numbers = numbers[0..8].to_vec();
    numbers.iter().map(|c| format!("{}", c)).collect()
}

fn main() {
    println!("#1: {}", phases(file2numbers("input.txt", 1), 100, false));
    println!("#2: {}", phases(file2numbers("input.txt", 10000), 100, true));
}

#[test]
fn test_0() {
    assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8], file2numbers("input.txt_test0", 1));
    assert_eq!(vec![4, 8, 2, 2, 6, 1, 5, 8], iteration(file2numbers("input.txt_test0", 1)));
    assert_eq!("01029498", phases(file2numbers("input.txt_test0", 1), 4, false));
}

#[test]
fn test_1() {
    assert_eq!("24176176", phases(file2numbers("input.txt_test1", 1), 100, false));
    assert_eq!("73745418", phases(file2numbers("input.txt_test2", 1), 100, false));
    assert_eq!("52432133", phases(file2numbers("input.txt_test3", 1), 100, false));
}

#[test]
fn test_2() {
    assert_eq!("84462026", phases(str2numbers("03036732577212944063491565474664", 10000), 100, true));
}