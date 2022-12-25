use std::fs::read_to_string;

fn parse(fp: &str) -> Vec<String> {
    let contents = read_to_string(fp).unwrap();

    contents.lines().map(|x| x.to_string()).collect()
}

fn str_to_snafu(s: String) -> i128 {
    let mut res = 0;

    for n in 0..s.len() {
        res += (5i128.pow(n as u32)) * match s.chars().nth(s.len() - n - 1).unwrap() {
            '=' => -2i128,
            '-' => -1,
            x => x.to_digit(10).unwrap() as i128
        };
    }

    res
}

fn snafu_to_str(n: i128) -> String {
    let mut n = n;
    let mut reminder = 0;
    let mut result = String::new();

    while n > 0 {
        let mut r = n % 5;
        n -= r;

        r += reminder;
        reminder = 0;

        let c = match r {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => {
                reminder += 1;
                '='
            },
            4 => {
                reminder += 1;
                '-'
            },
            5 => {
                reminder += 1;
                '0'
            }
            _ => unreachable!()
        };

        // println!("r:{} n:{}", r, n);

        n /= 5;
        result.push(c);
    }

    if reminder != 0 {
        result.push('1');
    }

    result.chars().rev().collect()
}

fn input_to_result(fp: &str) -> String {
    let lines = parse(fp);
    let mut result = 0;

    for line in lines {
        result += str_to_snafu(line);
    }

    snafu_to_str(result)
}

fn main() {
    println!("#1 {}", input_to_result("input.txt")); // 20-==01-2-=1-2---1-0
}

#[test]
fn test_sample() {
    let samples = [
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
    ];

    for sample in samples {
        assert_eq!(
            sample.0,
            str_to_snafu(sample.1.to_string())
        );
    
        assert_eq!(
            sample.1.to_string(),
            snafu_to_str(sample.0)
        );
    }
}

#[test]
fn test_sample_input() {
    assert_eq!(
        "2=-1=0".to_string(),
        input_to_result("input.txt_test")
    );
}