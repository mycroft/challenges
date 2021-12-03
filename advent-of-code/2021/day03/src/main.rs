fn get_rating(values: &Vec<Vec<bool>>, revert: bool) -> i32 {
    let mut values = values.clone();
    let bits_number = values[0].len();

    for i in 0..bits_number {
        // find most common bit for bit i
        let t_count = values.iter().filter(|&a| a[i]).count();
        let f_count = values.iter().filter(|&a| !a[i]).count();

        let value = if revert {
            t_count >= f_count
        } else {
            !(f_count <= t_count)
        };

        // filter all entries with 
        values = values.iter().filter(|&a| a[i] == value).cloned().collect::<Vec<Vec<bool>>>();

        if values.len() == 1 {
            break;
        }
    }

    values[0].iter().fold(0, |x, &v| x * 2 + v as i32)
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file to read");
    let lines = contents.lines().collect::<Vec<&str>>();

    let lines_num = lines.len();

    let mut bits_lines : Vec<Vec<bool>> = vec![];
    let mut values = vec![0i32;lines[0].len()];

    for line in &lines {
        let bit_line : Vec<bool> = line.chars().map(|c| c == '1').collect();
        bits_lines.push(bit_line);

        line.chars().enumerate().map(|(i, x)| {
            values[i] += (x == '1') as i32
        }).count();
    }

    let bits = values.iter().map(|&x| x > (lines_num/2) as i32).collect::<Vec<bool>>();
    let ibits = values.iter().map(|&x| x <= (lines_num/2) as i32).collect::<Vec<bool>>();

    let gamma = bits.iter().fold(0, |x, &v| x * 2 + v as i32);
    let epsilon = ibits.iter().fold(0, |x, &v| x * 2 + v as i32);

    println!("#1: {:?}", gamma * epsilon);


    let generator_rating = get_rating(&bits_lines, true);
    let scrubber_rating = get_rating(&bits_lines, false);

    println!("#2: {} {} {}", generator_rating, scrubber_rating, scrubber_rating * generator_rating);
}
