fn main() {
    let input = 919901;

    let mut recipes : Vec<i8> = vec![];
    recipes.push(3);
    recipes.push(7);

    let mut indexes = (0, 1);
    let mut round = 1;

    let mut input_arr : Vec<i8> = vec![];
    let mut input_tmp = input;

    loop {
        input_arr.insert(0, ((input_tmp as u32) % 10) as i8);
        input_tmp = (input_tmp - (input_tmp % 10)) / 10;

        if input_tmp == 0 {
            break;
        }
    }

    loop {
        let total = recipes[indexes.0] + recipes[indexes.1];
        if total >= 10 {
            recipes.push(total / 10);
        }
        recipes.push(total % 10);

        indexes.0 = (indexes.0 + recipes[indexes.0] as usize + 1) % recipes.len();
        indexes.1 = (indexes.1 + recipes[indexes.1] as usize + 1) % recipes.len();

        if round > input_arr.len() && recipes[round - input_arr.len()..round] == input_arr {
            break;
        }

        round += 1;
    }

    println!(
        "#1: {}",
        recipes[input..input + 10]
            .to_vec()
            .iter()
            .map(|&c| char::from_digit(c as u32, 10).unwrap())
            .collect::<String>()
    );

    println!("#2: {}", round - input_arr.len());
}
