fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("input file");
    let numbers = contents
        .trim_end()
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();
    
    let x_size = 25;
    let y_size = 6;

    let mut idx: usize = 0;

    let mut min_0 = x_size * y_size;
    let mut res = 0;

    let mut final_image: Vec<u32> = vec![2; min_0];

    while idx < numbers.len() {
        let rng = &numbers[idx..idx+(x_size * y_size)];

        for (idx, &el) in rng.iter().enumerate() {
            if final_image[idx] == 2 {
                final_image[idx] = el;
            }
        }

        let num_0 = rng.iter().filter(|&&x| x == 0).count();

        if num_0 < min_0 {
            min_0 = num_0;
            res = rng.iter().filter(|&&x| x == 1).count() * rng.iter().filter(|&&x| x == 2).count();
        }

        idx += x_size * y_size;
    }

    println!("#1 {}", res);

    for y in 0..y_size {
        for x in 0..x_size {
            let c = match final_image[x + x_size * y] {
                1 => '#',
                0 => ' ',
                _ => unreachable!()
            };
            print!("{}", c);
        }
        println!();
    }
}
