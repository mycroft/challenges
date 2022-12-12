use std::fs::read_to_string;
use std::collections::HashSet;

fn parse(fp: &str) -> Vec<Vec<usize>> {
    let contents = read_to_string(fp).unwrap();

    contents.lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn run_step1(map: &Vec<Vec<usize>>) -> usize {
    let mut count_viewable = 4; // all four corners

    let mut hs: HashSet<(usize, usize)> = HashSet::new();

    // for each line
    for line_idx in 1..map.len()-1 {
        let mut current_size = map[line_idx][0];
        count_viewable += 1;

        for col_idx in 1..map[line_idx].len() - 1 {
            if map[line_idx][col_idx] > current_size {
                // println!("1. Can see {} at line:{line_idx},col:{col_idx}", map[line_idx][col_idx]);
                if !hs.contains(&(line_idx, col_idx)) {
                    count_viewable += 1;
                    hs.insert((line_idx, col_idx));
                }

                current_size = map[line_idx][col_idx];
            }
        }

        current_size = map[line_idx][map[line_idx].len() - 1];
        count_viewable += 1;

        for col_idx in 1..(map[line_idx].len() - 1) {
            let col_idx = map[line_idx].len() - 1 - col_idx;
            if map[line_idx][col_idx] > current_size {
                // println!("2. Can see {} at line:{line_idx},col:{col_idx}", map[line_idx][col_idx]);
                if !hs.contains(&(line_idx, col_idx)) {
                    count_viewable += 1;
                    hs.insert((line_idx, col_idx));
                }
                current_size = map[line_idx][col_idx];
            }
        }
    }

    // for each column
    for col_idx in 1..map[0].len()-1 {
        let mut current_size = map[0][col_idx];
        count_viewable += 1;

        for line_idx in 1..map.len() - 1 {
            if map[line_idx][col_idx] > current_size {
                // println!("3. Can see {} at line:{line_idx},col:{col_idx}", map[line_idx][col_idx]);
                if !hs.contains(&(line_idx, col_idx)) {
                    count_viewable += 1;
                    hs.insert((line_idx, col_idx));
                }
                current_size = map[line_idx][col_idx];
            }
        }

        current_size = map[map.len() - 1][col_idx];
        count_viewable += 1;
        for line_idx in 1..(map.len() - 1) {
            let line_idx = map.len() - 1 - line_idx;
            if map[line_idx][col_idx] > current_size {
                // println!("4. Can see {} at line:{line_idx},col:{col_idx}", map[line_idx][col_idx]);
                if !hs.contains(&(line_idx, col_idx)) {
                    count_viewable += 1;
                    hs.insert((line_idx, col_idx));
                }
                current_size = map[line_idx][col_idx];
            }
        }
 
    }

    count_viewable
}

fn run_step2(map: &Vec<Vec<usize>>) -> usize {
    let mut score = 0;
    let max_line_idx = map.len() - 1;
    let max_col_idx = map[0].len() - 1;

    for line_idx in 1..max_line_idx {
        for col_idx in 1..max_col_idx {
            // for each tree...

            let dirs: [(i32, i32); 4] = [
                (0, 1),
                (1, 0),
                (0, -1),
                (-1, 0)
            ];

            let local_score = dirs.iter()
                .map(|dir| {
                    // We got a position (line_idx, col_idx), and a direction: dir.
                    let current_tree_size = map[line_idx][col_idx];
                    let mut viewable_trees = 0;

                    let mut current_line_idx = line_idx;
                    let mut current_col_idx = col_idx;

                    loop {
                        if current_line_idx == 0 || current_line_idx == max_line_idx {
                            break;
                        }
                        if current_col_idx == 0 || current_col_idx == max_col_idx {
                            break;
                        }
                        
                        // move
                        current_line_idx = (current_line_idx as i32 + dir.0) as usize;
                        current_col_idx = (current_col_idx as i32 + dir.1) as usize;

                        viewable_trees += 1;

                        if map[current_line_idx][current_col_idx] >= current_tree_size {
                            break;
                        }
                    }

                    // println!("{line_idx},{col_idx} can see {viewable_trees} on dir {dir:?}");
                    viewable_trees as usize
                })
                .fold(1, |acc: usize, x: usize| x * acc);

            if local_score > score {
                score = local_score;
            }
        }
    }

    score
}

fn main() {
    let map = parse("input.txt");

    println!("#1 {}", run_step1(&map));
    println!("#2 {}", run_step2(&map));
}

#[test]
fn test() {
    assert_eq!(21, run_step1(&parse("input_test.txt")));
    assert_eq!(8, run_step2(&parse("input_test.txt")));
}