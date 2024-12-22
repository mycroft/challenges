use std::collections::HashMap;

fn compute(secret: u64) -> u64 {
    let mut secret = ((secret * 64) ^ secret) % 16777216;
    secret = ((secret / 32) ^ secret) % 16777216;
    ((secret * 2048) ^ secret) % 16777216
}

fn read_input(fp: &str) -> Vec<u64> {
    std::fs::read_to_string(fp)
        .expect("Cannot read file")
        .lines()
        .map(|x| x.parse().unwrap())
        .collect()
}

fn secrets(n: u64) -> Vec<u64> {
    let mut secrets = Vec::new();

    secrets.push(n);

    for i in 0..2000 {
        secrets.push(compute(secrets[i]));
    }

    secrets
}

fn changes(base: &mut HashMap<u32, u64>, n: u64) {
    // let mut result = HashSet::new();
    let mut idx : u32 = 0;
    let mut secret = n;

    let mut done = [false; 1 << 20];

    for i in 0..2000 {
        let old_secret = secret;
        secret = compute(secret);

        let diff = (secret%10) as i32- (old_secret%10) as i32 + 9;
        //println!("diff: {} idx {} idx {}", diff, idx, idx << 5);
        idx <<= 5;
        idx |= diff as u32;
        idx %= 1 << 20;

        if i < 4 || done[idx as usize] {
            continue;
        }

        *base.entry(idx).or_insert(0) += secret % 10;
        done[idx as usize] = true;

        //result.insert(idx);
    }
}

fn solve_step2(numbers: &Vec<u64>) -> u64 {
    let mut base: HashMap<u32, u64> = HashMap::new();

    for number in numbers {
        changes(&mut base, *number);
    }

    base.values().copied().max().unwrap()
}

fn solve_step1(numbers: &Vec<u64>) -> u64 {
    let secrets = numbers.iter().map(|x| secrets(*x)).collect::<Vec<_>>();
    secrets.iter().map(|x| x[2000]).sum::<u64>()
}

fn main() {
    let input = read_input("input.txt");

    let current_time = std::time::Instant::now();

    let result_step1 = solve_step1(&input);
    println!("#1 {}", result_step1); // 13764677935

    println!("Time: {}ms", current_time.elapsed().as_millis());


    let result_step2 = solve_step2(&input);
    println!("#2 {}", result_step2); // 1619

    println!("Time: {}ms", current_time.elapsed().as_millis());
}

#[test]
fn sample() {
    let input = read_input("input_test.txt");
    let result_step1 = solve_step1(&input);
    assert_eq!(result_step1, 37327623);

    let input = read_input("input_test2.txt");
    let result_step2 = solve_step2(&input);
    assert_eq!(result_step2, 23);
}