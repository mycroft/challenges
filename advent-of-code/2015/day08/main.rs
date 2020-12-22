use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");
    let words : Vec<&str> = contents.split("\n").filter(|x| x != &"").collect();

    let mut total_size : i32 = 0;
    let mut total_real : i32 = 0;

    for word in &words {
        let mut c = 0;
        let mut real_n = -2;

        while c < word.len() {
            real_n += 1;
            if word.chars().into_iter().nth(c).unwrap() == '\\' {
                let next = word.chars().into_iter().nth(c+1).unwrap();
                if next == 'x' {
                    c += 4;
                    continue;
                } else {
                    c += 2;
                    continue;
                }
            }
            c += 1;
        }

        total_size += word.len() as i32;
        total_real += real_n;
    }

    println!("{:?} - {:?} = {:?}", total_size, total_real, total_size - total_real);

    let mut total_size : i32 = 0;
    let mut total_real : i32 = 0;

    for word in &words {
        let orig_count = word.len() as i32;
        let new_count = word
            .chars()
            .into_iter()
            .map(|x| match x { '\\' | '"' => 2, _ => 1 })
            .sum::<i32>() + 2;

        total_size += orig_count;
        total_real += new_count;
    }

    println!("{:?} - {:?} = {:?}", total_size, total_real, total_real - total_size);
}
