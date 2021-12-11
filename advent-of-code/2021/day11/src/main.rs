fn matrix_has_more_than_9(matrix: &Vec<Vec<u32>>) -> bool {
    matrix.iter().any(|l|
        l.iter().any(|x| x > &9))
}



fn matrix_step(matrix: &Vec<Vec<u32>>) -> (Vec<Vec<u32>>, usize, bool) {
    let deltas = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];

    let mut matrix = matrix.clone();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            matrix[i][j] += 1;
        }
    }

    let mut flashed: Vec<(usize, usize)> = vec![];

    loop {
        if !matrix_has_more_than_9(&matrix) {
            break;
        }

        let mut flashings = vec![];

        // find flashing
        for y in 0..matrix.len() {
            for x in 0..matrix[y].len() {
                if matrix[y][x] > 9 && !flashed.contains(&(x,y)) {
                    flashings.push((x, y));
                    matrix[y][x] = 0;
                }
            }
        }

        // run flashing
        for flashing in &flashings {
            for delta in deltas {
                let x = flashing.0 as i32 + delta.0;
                let y = flashing.1 as i32 + delta.1;

                if x < 0 || y < 0 || x as usize >= matrix[0].len() || y as usize >= matrix.len() {
                    continue;
                }

                matrix[y as usize][x as usize] += 1;
            }

            flashed.push(flashing.clone());
        }
    }

    // clean up flashed
    for flashing in &flashed {
        matrix[flashing.1][flashing.0] = 0;
    }

    let all_flashed = flashed.len() == matrix[0].len() * matrix.len();
    
    (matrix, flashed.len(), all_flashed)
}

fn dump(matrix: &Vec<Vec<u32>>) {
    for y in 0..matrix.len() {
        for x in 0..matrix[0].len() {
            print!("{}", matrix[y][x]);
        }
        println!("");
    }
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut matrix = vec![];
    for line in lines {
        let line = line
            .chars()
            .map(|x| x.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        matrix.push(line);
    }

    let mut step = 0;
    let mut total_flashed = 0;

    dump(&matrix);

    loop {
        step += 1;

        let (matrix_res, flashed_num, all_flashed) = matrix_step(&matrix);
        matrix = matrix_res;
        if step <= 100 {
            total_flashed += flashed_num;
        }

        if all_flashed {
            break;
        }
    }
    
    println!("#1 {}", total_flashed);
    println!("#2 {}", step);
}
