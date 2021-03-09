/*
 * AOC 2018 day 14.
 */
fn main() {
    let mut recipes = vec![3, 7];
    let mut lutin1_index = 0;
    let mut lutin2_index = 1;

    let mut part1_completed = false;
    let mut part2_completed = false;

    let after = 919901;
    let after_a = after
        .to_string()
        .chars()
        .map(|x| x as usize - '0' as usize)
        .collect::<Vec<usize>>();

    loop {
        let mut shift = 0;

        let score = recipes[lutin1_index] + recipes[lutin2_index];

        if score >= 10 {
            recipes.push(1);
            shift = 1;
        }

        recipes.push(score % 10);

        lutin1_index = (lutin1_index + 1 + recipes[lutin1_index]) % recipes.len();
        lutin2_index = (lutin2_index + 1 + recipes[lutin2_index]) % recipes.len();

        if part1_completed == false && recipes.len() >= after + 10 {
            let s = &recipes[after..after+10].iter().map(|x| '0' as u8 + *x as u8).collect::<Vec<u8>>();
            println!("Part #1: {}", String::from_utf8(s.to_vec()).unwrap());

            part1_completed = true;
        }

        if part2_completed == false && recipes.len() > after_a.len() {
            let v = &recipes[(recipes.len() - after_a.len() - shift)..(recipes.len() - shift)];

            if after_a == v {
                println!("Part #2: {:?}", recipes.len() - after_a.len() - shift);
                part2_completed = true;
            }
        }

        if part1_completed && part2_completed {
            break;
        }
    }

    // done.
}
