#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point (usize, usize);

fn find_low_points(matrix: &[Vec<usize>]) -> Vec<Point> {
    let max_x = matrix[0].len();
    let max_y = matrix.len();

    let deltas = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1)
    ];

    let mut low_values = vec![];

    for i in 0..max_x {
        for j in 0..max_y {
            let mut is_low = true;
            let current = matrix[j][i];

            for delta in deltas {
                if (j as i32 + delta.0 < 0) || (j as i32 + delta.0 >= max_y as i32) {
                    continue;
                }
                if (i as i32 + delta.1 < 0) || (i as i32 + delta.1 >= max_x as i32) {
                    continue;
                }

                let other = matrix[(j as i32 + delta.0) as usize][(i as i32 + delta.1) as usize];
                if other < current {
                    is_low = false;
                }
            }

            if is_low {
                low_values.push(Point(i, j));
            }
        }
    }

    low_values
}

fn basin_size(matrix: &[Vec<usize>], starting_point: Point) -> Vec<Point> {
    let max_x = matrix[0].len();
    let max_y = matrix.len();

    let deltas = [
        (-1, 0), (0, -1), (0, 1), (1, 0)
    ];

    let mut basin_points = vec![];
    let mut to_scan = vec![starting_point];

    while !to_scan.is_empty() {
        let current = to_scan.pop().unwrap();
        if !basin_points.contains(&current) {
            basin_points.push(current);
        }

        let i = current.0;
        let j = current.1;

        for delta in deltas {
            if (j as i32 + delta.0 < 0) || (j as i32 + delta.0 >= max_y as i32) {
                continue;
            }
            if (i as i32 + delta.1 < 0) || (i as i32 + delta.1 >= max_x as i32) {
                continue;
            }

            let new_x = (i as i32 + delta.1) as usize;
            let new_y = (j as i32 + delta.0) as usize;

            if basin_points.contains(&Point(new_x, new_y)) {
                continue;
            }

            let other = matrix[new_y][new_x];
            if other == 9 {
                continue;
            }

            to_scan.push(Point(new_x, new_y));
        }
    }

    basin_points
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file to read");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut matrix = vec![];

    for line in lines {
        let line = line
            .chars()
            .map(|c| c.to_digit(10).expect("number") as usize)
            .collect::<Vec<usize>>();
        matrix.push(line);
    }

    let low_values = find_low_points(&matrix);

    let res_1 = low_values.iter().map(|x| (matrix[x.1][x.0] + 1) as i32).sum::<i32>();

    println!("#1 {}", res_1);

    let mut sizes = vec![];

    for p in low_values {
        let s = basin_size(&matrix, p);
        sizes.push(s.len());
    }

    sizes.sort_unstable();

    println!("#2 {}", &sizes[sizes.len()-3..sizes.len()].iter().product::<usize>());
}
