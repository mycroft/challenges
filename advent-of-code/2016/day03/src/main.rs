use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();
    let mut count = 0;
    let mut all : Vec<Vec<u32>> = Vec::new();

    for line in lines {
        let values = line
            .split(" ")
            .filter(|x| *x != "")
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if values[0] + values[1] > values[2] && values[0] + values[2] > values[1] && values[2] + values[1] > values[0] {
            count += 1;
        }

        all.push(values);
    }

    println!("Part #1: {:?}", count);

    let mut idx = 0;
    let mut count = 0;

    loop {
        if idx == all.len() {
            break;
        }

        for n in 0..3 {
            let v0 = all[idx][n];
            let v1 = all[idx + 1][n];
            let v2 = all[idx + 2][n];

            if v0 + v1 > v2 && v0 + v2 > v1 && v2 + v1 > v0 {
                count += 1;
            }
        }

        idx += 3;
    }

    println!("Part #2: {:?}", count);
}
