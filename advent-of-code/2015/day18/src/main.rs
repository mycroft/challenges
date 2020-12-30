use std::fs;
use std::iter::FromIterator;

fn neighbour(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> u8 {
    let mut score = 0;
    let i : i32 = i as i32;
    let j : i32 = j as i32;

    for n_i in i-1..=i+1 {
        for n_j in j-1..=j+1 {
            if (n_i == i && n_j == j) || n_i < 0 || n_j < 0 || n_i >= matrix.len() as i32 || n_j >= matrix[i as usize].len() as i32 {
                continue;
            }

            if matrix[n_i as usize][n_j as usize] == '#' {
                score += 1;
            }
        }
    }

    score
}

/*
A light which is on stays on when 2 or 3 neighbors are on, and turns off otherwise.
A light which is off turns on if exactly 3 neighbors are on, and stays off otherwise.
*/

fn iteration(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_matrix : Vec<Vec<char>> = vec![];

    for i in 0..matrix.len() {
        let mut current_row = vec![];
        for j in 0.. matrix[i].len() {
            let neighbour = neighbour(&matrix, i, j);
            // println!("i:{:?} j:{:?} n:{:?}", i, j, neighbour);
            let new_char = match matrix[i][j] {
                '.' => if neighbour == 3 { '#' } else { '.' },
                '#' => if neighbour == 2 || neighbour == 3 { '#' } else { '.' },
                _ => { '0' }
            };

            current_row.push(new_char);
        }
        new_matrix.push(current_row);
    }

    new_matrix
}

#[allow(dead_code)]
fn dump(matrix: &Vec<Vec<char>>) {
    for i in 0..matrix.len() {
        println!("{}", String::from_iter(matrix[i].iter()));
    }

    println!("--");
}

fn count(matrix: &Vec<Vec<char>>) -> u32 {
    let mut lights = 0;
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == '#' {
                lights += 1
            }
        }
    }

    lights
}

fn init_matrix(filename: &str) -> Vec<Vec<char>> {
    let content = fs::read_to_string(filename).unwrap();
    let lines = content.lines();

    let mut matrix : Vec<Vec<char>> = vec![];
    for line in lines {
        matrix.push(line.chars().collect::<Vec<char>>());
    }

    matrix
}

fn main() {
    let mut matrix = init_matrix("input.txt");
    let rounds = 100;

    for _it in 1..=rounds {
        matrix = iteration(matrix);

        // println!("After {:?} step(s)", _it);
        // dump(&matrix);
    }

    println!("Part #1: {:?}", count(&matrix));

    let mut matrix = init_matrix("input.txt");
    let l1 = matrix.len();

    for _it in 1..=rounds {
        matrix = iteration(matrix);

        matrix[0][0] = '#';
        matrix[0][l1-1] = '#';
        matrix[l1-1][0] = '#';
        matrix[l1-1][l1-1] = '#';

        // println!("After {:?} step(s)", _it);
        // dump(&matrix);
    }

    println!("Part #2: {:?}", count(&matrix));

}

#[test]
fn example() {
    let mut matrix = init_matrix("input_test.txt");
    let rounds = 4;

    for _it in 1..=rounds {
        matrix = iteration(matrix);
    }

    assert_eq!(4, count(&matrix));
}
