use std::fs;
use std::cmp::{max,min};

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let z : Vec<&str> = contents
        .split("\n")
        .collect();

    let mut total : u32 = 0;
    let mut total_ribbon : u32 = 0;

    for v in z {
        if v == "" {
            continue;
        }

        let dim : Vec<u32> = v
            .split('x')
            .filter_map(|x| x.parse::<u32>().ok())
            .collect();

        let wrapping = 2 * (dim[0] * dim[1] + dim[0] * dim[2] + dim[1] * dim[2]);
        let slack = min(min(dim[0] * dim[1], dim[0] * dim[2]), dim[1] * dim[2]);

        total += wrapping;
        total += slack;

        let ribbon = dim[0] * dim[1] * dim[2];
        let bow = 2 * (min(min(dim[0], dim[1]), dim[2]) + ribbon / (max(max(dim[0], dim[1]), dim[2]) * min(min(dim[0], dim[1]), dim[2])));

        total_ribbon += bow;
        total_ribbon += ribbon;

    }

    println!("{:?} {:?}", total, total_ribbon);
}