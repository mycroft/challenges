use std::fs;

fn main() {
    let contents = fs::read_to_string("p022_names.txt")
        .expect("Something went wrong reading the file");

    let mut names : Vec<String> = contents
        .split(",")
        .map(|x| x.replace("\"", ""))
        .collect();

    names
        .sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    let values = names
        .into_iter()
        .map(|x| x
            .chars()
            .map(|c| c as u32 - 'A' as u32 + 1)
            .sum::<u32>()
        )
        .enumerate()
        .map(|(i, x)| (1+i as u32) * x)
        .sum::<u32>();

    println!("{:?}", values);
}
