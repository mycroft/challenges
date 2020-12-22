use std::fs;
use regex::Regex;

fn main() {
	let contents = fs::read_to_string("input.txt")
		.expect("Something went wrong reading the file");

	let lines = contents.split("\n");

	let re = Regex::new(r"(\d+),(\d+) through (\d+),(\d+)$").unwrap();

	let mut grid1 = vec![vec![false; 1000]; 1000];
	let mut grid2 = vec![vec![0; 1000]; 1000];

	for line in lines {
		if line == "" {
			break;
		}

		let coord = re.captures(line).unwrap();
		let x1 = coord[1].parse::<u32>().unwrap() as usize;
		let y1 = coord[2].parse::<u32>().unwrap() as usize;
		let x2 = coord[3].parse::<u32>().unwrap() as usize;
		let y2 = coord[4].parse::<u32>().unwrap() as usize;

    	for i in x1..=x2 {
    		for j in y1..=y2 {
    			if line.starts_with("toggle") {
    				grid1[i][j] = !grid1[i][j];
    				grid2[i][j] += 2;
    			} else if line.starts_with("turn on") {
    				grid1[i][j] = true;
    				grid2[i][j] += 1;
    			} else if line.starts_with("turn off") {
    				grid1[i][j] = false;
    				grid2[i][j] -= 1;
    				if grid2[i][j] < 0 {
    					grid2[i][j] = 0;
    				}
    			}
    		}
    	}
	}

	let mut total1 = 0;
	let mut total2 = 0;

	for i in 0..=999 {
		for j in 0..=999 {
			if grid1[i][j] {
				total1 += 1;
			}
			total2 += grid2[i][j];
		}
	}

	println!("{:?}", total1);
	println!("{:?}", total2);
}