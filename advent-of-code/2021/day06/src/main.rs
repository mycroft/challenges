fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file to open");
    let contents = contents.trim_end();
    let orig_numbers = contents
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| {x.parse::<i32>().expect("number")})
        .collect::<Vec<i32>>();

    let mut numbers = orig_numbers.clone();
    for _day in 0..80 {
        let mut to_add = 0;
        for n in numbers.iter_mut() {
            if n == &0 {
                *n = 6;
                to_add += 1;
            } else {
                *n -= &1;
            }
        }

        for _ in 0..to_add {
            numbers.push(8);
        }
    }

    println!("#1: {}", numbers.len());

    let mut cycles = [0u128; 9];

    for &n in orig_numbers.iter() {
        cycles[n as usize] += 1;
    }

    for day in 0..256 {
        let cycle_number = day % 7;
        let to_add = cycles[7];

        cycles[7] = cycles[8];
        cycles[8] = cycles[cycle_number];

        cycles[cycle_number] += to_add;

    }

    println!("#2: {}", cycles.iter().sum::<u128>());
}
