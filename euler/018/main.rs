use std::fs::File;
use std::io::prelude::*;
use std::cmp::max;


fn main() -> std::io::Result<()> {
    let mut file = File::open("input.txt")?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    let mut lines : Vec<Vec<u32>> = contents
        .split("\n")
        .map(|x| x
            .split(" ")
            .map(|y| y
                .parse::<u32>().unwrap()
            )
            .collect()
        )
        .collect();

    let mut last_line : Vec<u32> = vec![0];

    for (line_num, line) in lines.iter_mut().enumerate() {

        if line_num == 0 {
            last_line = line.to_vec();
            continue;
        }

        let borne_max = line.len() - 1;

        for (pos, el) in line.iter_mut().enumerate() {
            if pos == 0 {
                *el = *el + last_line[0];
            } else if pos < borne_max {
                *el = *el + max(last_line[pos - 1], last_line[pos]);
            } else {
                *el = *el + last_line[pos - 1];
            }
        }

        last_line = line.to_vec();
    }

    println!("{:?}", lines.last().unwrap().iter().max().unwrap());

    Ok(())
}

