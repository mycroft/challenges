use std::fs;
use pathfinding::prelude::bfs;
use std::collections::HashMap;

fn find(matrix: &Vec<Vec<char>>, tofind: char) -> Result<(usize, usize), String> {
    for (i, line) in matrix.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if *c == tofind {
                return Ok((j, i))
            }
        }
    }

    Err(String::from("could not find character"))
}

fn successors(matrix: &Vec<Vec<char>>, position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut out = vec![];
    let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for delta in deltas.iter() {
        let x = (position.0 as i32 + delta.0) as usize;
        let y = (position.1 as i32 + delta.1) as usize;
        if matrix[y][x] != '#' {
            out.push((x, y));
        }
    }

    out
}

fn reduce(grid: &HashMap<char, Vec<(char, usize)>>, visited: &mut Vec<char>, start_char: char, part2: bool) -> usize {
    visited.push(start_char);

    let mut min_path = 0;

    for m in grid.get(&start_char).unwrap().iter() {
        if visited.iter().any(|x| *x == m.0) {
            continue;
        }

        let path_size = reduce(&grid, visited, m.0, part2);

        if min_path == 0 || min_path > path_size + m.1 {
            min_path = path_size + m.1;
        }
    }

    visited.pop();

    if min_path == 0 && part2 {
        // we are at the end of the queue. Let's add start_char <=> 0.
        min_path = grid.get(&start_char).unwrap().iter().filter(|x| x.0 == '0').map(|x| x.1).nth(0).unwrap();
    }

    min_path
}


fn main() {
    let _contents = fs::read_to_string("input.txt").unwrap();

    let mut matrix : Vec<Vec<char>> = vec![];
    let mut chars : Vec<char> = vec![];

    for line in _contents.lines() {
        matrix.push(line.chars().collect::<Vec<char>>());
        for c in line.chars().filter(|x| *x != '.' && *x != '#').collect::<Vec<char>>() {
            chars.push(c);
        }
    }

    let mut grid : HashMap<char, Vec<(char, usize)>> = HashMap::new();

    for (_i, c1) in chars.iter().enumerate() {
        let mut _successors : Vec<(char, usize)> = vec![];
        for (_j, c2) in chars.iter().enumerate() {
            if c1 == c2 {
                continue;
            }

            let from_position = find(&matrix, *c1).unwrap();
            let to_position = find(&matrix, *c2).unwrap();

            let res_search = bfs(&from_position,
                |p| successors(&matrix, *p),
                |p| *p == to_position,
            ).unwrap();

            // println!("{:?} => {:?} = {:?}", from_position, to_position, res_search.len() - 1);

            _successors.push((*c2, res_search.len() - 1));
        }

        grid.insert(*c1, _successors);
    }

    println!("Part #1: {:?}", reduce(&mut grid, &mut vec![], '0', false));
    println!("Part #2: {:?}", reduce(&mut grid, &mut vec![], '0', true));
}
