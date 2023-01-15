use std::fs::read_to_string;
use std::collections::VecDeque;
use modinverse::modinverse;
use num_bigint::ToBigInt;
use num_traits::cast::ToPrimitive;

#[derive(Debug)]
enum Kind {
    DealIncrement(isize),
    Cut(isize),
    DealNewStack
}

fn parse(fp: &str) -> Vec<Kind> {
    let contents = read_to_string(fp).expect("a file to open");
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut result = Vec::new();

    for line in &lines {
        let instr = if line.starts_with("deal with increment ") {
            let parts = line.split(' ').collect::<Vec<&str>>();
            Kind::DealIncrement(parts[3].parse::<isize>().unwrap())
        } else if line.starts_with("cut") {
            let parts = line.split(' ').collect::<Vec<&str>>();
            Kind::Cut(parts[1].parse::<isize>().unwrap())
        } else if line.starts_with("deal into new stack") {
            Kind::DealNewStack
        } else {
            unreachable!()
        };

        result.push(instr);
    }

    result
}

fn deal_new_stack(game: &VecDeque<isize>) -> VecDeque<isize> {
    let mut result = VecDeque::new();

    for card in game {
        result.push_front(*card);
    }

    result
}

fn deal_increment(game: &VecDeque<isize>, n: isize) -> VecDeque<isize> {
    let mut game = game.to_owned();
    let mut result: VecDeque<isize>  = (0..game.len() as isize).collect();

    let result_len = game.len();
    let mut idx: isize = 0;

    while !game.is_empty() {
        result[(idx as usize) % result_len] = game.pop_front().unwrap();
        idx += n;
    }

    result
}

fn cut(game: &VecDeque<isize>, n: isize) -> VecDeque<isize> {
    let mut result = VecDeque::new();
    let mut game = game.to_owned();

    let n = if n < 0 {
        game.len() as isize + n
    } else {
        n
    };

    let mut p2 = game.split_off(n as usize);

    result.append(&mut p2);
    result.append(&mut game);

    result
}

fn play_step1(fp: &str, s: isize) -> VecDeque<isize> {
    let instrs = parse(fp);

    let mut game = (0..s).collect::<VecDeque<isize>>();
    for instr in &instrs {
        game = match instr {
            Kind::DealNewStack => deal_new_stack(&game),
            Kind::DealIncrement(n) => deal_increment(&game, *n),
            Kind::Cut(n) => cut(&game, *n)
        }
    }

    game
}

fn play_step2(fp: &str, l: i128) -> (i128, i128) {
    let mut instrs = parse(fp);
    let mut a: i128 = 1;
    let mut b: i128 = 0;

    instrs.reverse();

    for instr in &instrs {
        match instr {
            Kind::DealNewStack => {
                a *= -1;
                b = l - b - 1;
            },
            Kind::DealIncrement(n) => {
                let z = modinverse(*n as i128, l).unwrap();

                a = (a.to_bigint().unwrap() * z.to_bigint().unwrap()).modpow(&1.to_bigint().unwrap(), &l.to_bigint().unwrap()).to_i128().unwrap();
                b = (b.to_bigint().unwrap() * z.to_bigint().unwrap()).modpow(&1.to_bigint().unwrap(), &l.to_bigint().unwrap()).to_i128().unwrap();

            },
            Kind::Cut(n) => {
                b = (b + *n as i128) % l;
            }
        };
    }

    (a, b)
}

fn polypow(a: i128, b: i128, m: i128, n: i128) -> (i128, i128) {
    if m == 0 {
        (1, 0)
    } else if m % 2 == 0 {
        polypow(a*a%n, (a*b+b)%n, m / 2, n)
    } else {
        let (c, d) = polypow(a, b, m-1, n);
        (a*c%n, (a*d+b)%n)
    }
}

fn shuffle2(fp: &str, l: i128, n: i128, pos: i128) -> i128 {
    let (a, b) = play_step2(fp, l);
    let (a, b) = polypow(a, b, n, l);
    (pos * a + b) % l
}

fn main() {
    let res = play_step1("input.txt", 10007);
    println!("#1 {}", res.iter().position(|card| *card == 2019).unwrap()); // 1867

    let l = 119315717514047;
    let n = 101741582076661;
    let pos = 2020;
    let res2 = shuffle2("input.txt", l, n, pos);

    println!("#2 {res2}"); // 71047285772808
}

#[test]
fn test_sample() {
    assert_eq!(
        [0, 3, 6, 9, 2, 5, 8, 1, 4, 7].to_vec(),
        play_step1("input.txt_test0", 10).into_iter().collect::<Vec<isize>>(),
    );

    assert_eq!(
        [3, 0, 7, 4, 1, 8, 5, 2, 9, 6].to_vec(),
        play_step1("input.txt_test1", 10).into_iter().collect::<Vec<isize>>(),
    );

    assert_eq!(
        [6, 3, 0, 7, 4, 1, 8, 5, 2, 9].to_vec(),
        play_step1("input.txt_test2", 10).into_iter().collect::<Vec<isize>>(),
    );

    assert_eq!(
        [9, 2, 5, 8, 1, 4, 7, 0, 3, 6].to_vec(),
        play_step1("input.txt_test3", 10).into_iter().collect::<Vec<isize>>(),
    );
}
