use pathfinding::prelude::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn successors(&self, matrix: &[Vec<usize>], max_x: usize, max_y: usize) -> Vec<(Pos, u32)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|(x,y)| {
                (self.0 + x, self.1 + y)
            })
            .filter(|(x, y)|
                *x >= 0 && *y >= 0 && *x < max_x as i32 && *y < max_y as i32
            )
            .map(|(x, y)| (Pos(x, y), get_matrix_value_at(matrix, x as usize, y as usize) as u32))
            .collect::<Vec<(Pos, u32)>>()
    }
}

fn get_matrix_value_at(matrix: &[Vec<usize>], x: usize, y: usize) -> usize {
    let res = matrix[y % matrix.len()][x % matrix[0].len()];

    let mut r = res + x / matrix[0].len() + y / matrix.len();

    while r > 9 {
        r -= 9;
    }

    r
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file");
    let matrix = contents
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|l| {
            l
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    println!("#1: {}", solve(&matrix, 1));
    println!("#2: {}", solve(&matrix, 5));
}

fn solve(matrix: &[Vec<usize>], factor: usize) -> usize {
    let goal = Pos((factor * matrix[matrix.len() - 1].len() - 1) as i32, (factor * matrix.len() - 1) as i32);

    let res = astar(
        &Pos(0, 0),
        |p| p.successors(matrix, matrix[0].len() * factor, matrix.len() * factor),
        |_p| 0,
        |p| *p == goal
    ).expect("value");
  
    let mut distance = 0;

    for p in res.0 {
        distance += get_matrix_value_at(matrix, p.0 as usize, p.1 as usize);
    }

    // Remove first.
    distance -= matrix[0][0];
    
    distance
}