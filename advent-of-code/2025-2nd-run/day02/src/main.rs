use std::fs::read_to_string;
use std::cmp::{min, max};

fn main() {
    let contents = read_to_string("input.txt").expect("Failed to read input file");
    let lines = contents.lines();

    let mut total_surface = 0;
    let mut total_ribbon = 0;

    for line in lines {
        let (x, y, z) = {
            let mut nums = line.split('x')
                .map(|s| s.parse::<i32>().expect("Failed to parse number"))
                .collect::<Vec<i32>>();
            nums.sort();
            (nums[0], nums[1], nums[2])
        };

        let surface_area = 2 * (x * y + y * z + z * x);
        let smaller_side = min(min(x * y, y * z), z * x);
        total_surface += surface_area + smaller_side;

        let ribbon = 2 * x + 2 * y + 2 * z - 2 * max(max(x, y), z) + x * y * z;
        total_ribbon += ribbon;
    }

    println!("#1 {}", total_surface);
    println!("#2 {}", total_ribbon);
}
