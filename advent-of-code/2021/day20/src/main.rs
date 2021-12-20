use std::collections::HashSet;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point(i32, i32);

fn main() {
    let values = parse_file("input.txt");

    let samples = values.0;
    let mut picture = values.1;

    let orig_w = picture.iter().map(|p| p.0).max().unwrap();
    let orig_h = picture.iter().map(|p| p.1).max().unwrap();

    for step in 0..50 {
        picture = compute_step(&picture, &samples, step, orig_w, orig_h);
        if step == 1 {
            println!("#1 {}", picture.len());
        }
    }

    println!("#2 {}", picture.len());
}

fn draw(matrix: &HashSet<Point>) {
    let min_x = matrix.iter().map(|p| p.0).min().unwrap();
    let max_x = matrix.iter().map(|p| p.0).max().unwrap();
    let min_y = matrix.iter().map(|p| p.1).min().unwrap();
    let max_y = matrix.iter().map(|p| p.1).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if matrix.contains(&Point(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn compute_step(matrix: &HashSet<Point>, samples: &[bool], step: usize, h: i32, w: i32) -> HashSet<Point> {
    let mut res = HashSet::new();

    let border = 50;

    for y in -border..=h+border {
        for x in -border..=w+border {
            let mut binaries = vec![];

            // for surrounding points
            for inner_y in y-1..=y+1 {
                for inner_x in x-1..=x+1 {

                    // out of limits: apply sample[0]
                    let is_true = if inner_x < -border || inner_x > w+border-1 || inner_y < -border || inner_y > h+border-1 {
                        step%2 == 1 && samples[0]
                    } else {
                        matrix.contains(&Point(inner_x, inner_y))
                    };

                    binaries.push(is_true);
                }
            }

            let point_idx = binaries.iter()
                .fold(0usize, |acc, &v| (acc << 1) + v as usize);

            if samples[point_idx] {
                res.insert(Point(x, y));
            }
        }
    }

    res
}

fn parse_file(fp: &str) -> (Vec<bool>, HashSet<Point>) {
    let contents = std::fs::read_to_string(fp).expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut is_sample = true;
    let mut sample_vec = vec![];
    let mut matrix = HashSet::new();
    let mut line_num = 0;

    for line in lines {
        let line = line.trim_end();

        if line.is_empty() {
            is_sample = false;
            continue;
        }
        let mut v = line.chars()
            .map(|c| c == '#')
            .collect::<Vec<bool>>();

        if is_sample {
            sample_vec.append(&mut v);
        } else {
            // Image part
            for p in v.iter().enumerate() {
                if *p.1 {
                    matrix.insert(Point(p.0 as i32, line_num));
                }
            }

            line_num += 1;
        }
    }

    (sample_vec, matrix)
}