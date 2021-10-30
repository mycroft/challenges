/* AOC 2018 - 18 */
use std::{env, fs};
use std::collections::HashMap;

#[derive(Copy,Clone,Debug, PartialEq, Eq, Hash)]
enum State {
    Open,
    Tree,
    Lumberyard,
}

#[derive(Debug)]
struct MatrixResult(usize, usize);

fn display(matrix: &Vec<Vec<State>>) {
    for (_y, line) in matrix.iter().enumerate() {
        let mut line_str = String::from("");
        
        for (_x, state) in line.iter().enumerate() {
            let c = match state {
                State::Open => '.',
                State::Tree => '|',
                State::Lumberyard => '#',
            };

            line_str.push(c);
        }

        println!("{}", line_str);
    }

    println!("");
}

fn get_adj_things(matrix: &Vec<Vec<State>>, x: usize, y: usize) -> (usize, usize, usize) {
    let deltas = [
        (-1, -1), (-1,  0), (-1,  1),
        ( 0, -1),           ( 0,  1),
        ( 1, -1), ( 1,  0), ( 1,  1),
    ];

    let mut res = (0, 0, 0);

    for delta in deltas {
        if x as i32 + delta.0 < 0 || 
           x as i32 + delta.0 >= matrix[0].len() as i32 ||
           y as i32 + delta.1 < 0 ||
           y as i32 + delta.1 >= matrix.len() as i32
        {
            continue;
        }

        match matrix[(y as i32 + delta.1) as usize][(x as i32 + delta.0) as usize] {
            State::Open => res.0 += 1,
            State::Tree => res.1 += 1,
            State::Lumberyard => res.2 += 1,
        };
    }

    res
}

/*  
* An open acre will become filled with trees if three or more adjacent acres contained trees.
  Otherwise, nothing happens.

* An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards.
  Otherwise, nothing happens.

* An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one 
  other lumberyard and at least one acre containing trees. Otherwise, it becomes open.
*/

fn step(matrix: &Vec<Vec<State>>) -> Vec<Vec<State>> {
    let mut new_matrix = vec![];

    for (y, matrix_line) in matrix.iter().enumerate() {
        let mut state_line = vec![];

        for (x, state) in matrix_line.iter().enumerate() {
            let things = get_adj_things(&matrix, x, y);

            let next_state = match state {
                State::Open => if things.1 >= 3 { State::Tree } else { State::Open },
                State::Tree => if things.2 >= 3 { State::Lumberyard } else { State::Tree },
                State::Lumberyard => if things.1 >= 1 && things.2 >= 1 { State::Lumberyard } else { State::Open }
            };

            state_line.push(next_state);

        }

        new_matrix.push(state_line);
    }

    new_matrix
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut filepath = "input.txt";

    if args.len() > 1 {
        filepath = args[1].as_str();
    }

    let contents = fs::read_to_string(filepath).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut matrix : Vec<Vec<State>> = vec![];
    let mut known_matrices: HashMap<Vec<Vec<State>>, MatrixResult> = HashMap::new();

    for line in &lines {
        let matrix_line = line
            .chars()
            .map(|c| match c {
                '.' => State::Open,
                '|' => State::Tree,
                '#' => State::Lumberyard,
                _ => unreachable!()
            })
            .collect::<Vec<State>>();

        matrix.push(matrix_line);
    }

    let max_step = 1000000000;
    let mut jumped_in_the_future = false;
    let mut s = 1;
    loop {
        if s > max_step {
            break;
        }

        matrix = step(&matrix);

        let res = matrix
            .iter()
            .flatten()
            .fold((0,0), |res, &v| match v {
                State::Tree => (res.0 + 1, res.1),
                State::Lumberyard => (res.0, res.1 + 1),
                State::Open => (res.0, res.1),
            });

        if s == 10 { println!("#1: {}", res.0 * res.1); }
        if s == max_step { println!("#2: {}", res.0 * res.1); }

        if !jumped_in_the_future && known_matrices.contains_key(&matrix) {
            let former_result = known_matrices.get(&matrix).unwrap();
            let cycle_length = s - former_result.0;

            jumped_in_the_future = true;
            s += max_step - s - (max_step - s) % cycle_length;
        } else {
            known_matrices.insert(matrix.clone(), MatrixResult(s, res.0 * res.1));
        }

        s += 1;
    }
}
