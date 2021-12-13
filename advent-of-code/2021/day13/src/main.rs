#[macro_use] extern crate scan_fmt;

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut matrix = vec![];
    let mut folds = vec![];

    for line in lines {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if line.starts_with("fold along ") {
            let (fold_at, fold_by) = scan_fmt!(
                line,
                "fold along {}={}",
                char, i32
            ).unwrap();

            folds.push((fold_at, fold_by));
        } else {
            let parts = line
                .split(',')
                .collect::<Vec<&str>>()
                .iter()
                .map(|f| f.parse::<i32>().expect("number"))
                .collect::<Vec<i32>>();

            matrix.push((parts[0], parts[1]));
        }
    }

    for (idx, fold_order) in folds.iter().enumerate() {
        matrix = fold(&matrix, fold_order.0, fold_order.1);
        if idx == 0 {
            println!("#1 {}", matrix.len());
        }
    }

    println!("#2:");
    dump(&matrix);
}

fn fold(matrix: &[(i32, i32)], at: char, by: i32) -> Vec<(i32, i32)> {
    let mut matrix_res: Vec<(i32, i32)> = vec![];

    for p in matrix.iter() {
        let mut p_coord = if at == 'x' { p.0 } else { p .1 };
        let mut p = *p;
        if p_coord > by {
            p_coord -= 2 * (p_coord - by);

            if at == 'x' {
                p.0 = p_coord;
            } else {
                p.1 = p_coord;
            }    
        }

        if !matrix_res.contains(&p) {
            matrix_res.push(p);
        }
    }

    // remove duplicatas
    matrix_res
}

fn dump(points: &[(i32, i32)]) {
    let max_x = points.iter().map(|x| x.0).max().unwrap();
    let max_y = points.iter().map(|x| x.1).max().unwrap();

    let mut matrix = vec![vec![false; 1+max_x as usize]; 1+max_y as usize];

    for point in points {
        matrix[point.1 as usize][point.0 as usize] = true;
    }

    for l in matrix {
        for p in l {
            if p {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}
