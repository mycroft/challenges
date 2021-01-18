use mkz_aoc::file;

fn run(instructions: &mut Vec<i128>, apply: impl Fn(i128) -> i128) -> usize {
    let mut c = 0;
    let mut index = 0;

    loop {
        if index >= instructions.len() {
            break;
        }

        let old_value = instructions[index];
        instructions[index] = apply(instructions[index]);
        index = (index as i128 + old_value) as usize;

        c += 1;
    }

    c
}

fn solve1(instructions: &mut Vec<i128>) -> usize {
    run(instructions, |x| x + 1)
}

fn solve2(instructions: &mut Vec<i128>) -> usize {
    run(instructions, |x| if x >= 3 { x - 1} else { x + 1 })
}

fn main() {
    let lines = file::read_to_numbers("input.txt").unwrap();

    println!("Part #1: {}", solve1(&mut lines.clone()));
    println!("Part #2: {}", solve2(&mut lines.clone()));
}

#[test]
fn example() {
    assert_eq!(5, solve1(&mut vec![0, 3, 0, 1, -3]));
}
