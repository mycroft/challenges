use std::fs::read_to_string;
use pathfinding::prelude::bfs;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
struct Pos {
    x: usize,
    y: usize,
    c: char,
}

#[derive(Debug, Clone)]
struct NotFoundError;

fn parse(fp: &str) -> Vec<Vec<char>> {
    let contents = read_to_string(fp).unwrap();
    let lines: Vec<&str> = contents.lines().collect();

    let mut m = Vec::new();

    for line in lines {
        if line.len() == 0 {
            break;
        }
        m.push(
            line.chars().collect()
        );
    }

    m
}

fn find_in_map(m: &Vec<Vec<char>>, c: char) -> Result<Pos, NotFoundError> {
    for y in 0..m.len() {
        for x in 0..m[y].len() {
            if m[y][x] == c {
                return Ok(Pos{x, y, c});
            }
        }
    }

    return Err(NotFoundError)
}

fn find_all_in_map(m: &Vec<Vec<char>>, c: char) -> Vec<Pos> {
    let mut v = Vec::new();

    for y in 0..m.len() {
        for x in 0..m[y].len() {
            if m[y][x] == c {
                v.push(
                    Pos{x, y, c}
                );
            }
        }
    }

    v
}

fn successors(m: &Vec<Vec<char>>, p: Pos) -> Vec<Pos> {
    let mut succ = Vec::new();
    let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let c = m[p.y][p.x];

    for dir in dirs {
        if ((p.x as i32) + dir.0) < 0 || ((p.y as i32) + dir.1) < 0 || ((p.x as i32) + dir.0) >= m[0].len() as i32 || ((p.y as i32) + dir.1) >= m.len() as i32 {
            continue;
        }
    
        let np = Pos{
            x: ((p.x as i32) + dir.0) as usize,
            y: ((p.y as i32) + dir.1) as usize,
            c: m[((p.y as i32) + dir.1) as usize][((p.x as i32) + dir.0) as usize],
        };
        
        let nc = m[np.y][np.x];

        if c == 'S' && m[np.y][np.x] == 'a' {
            succ.push(np);
        } else if nc == 'E' && c == 'z' {
            succ.push(np);
        } else if nc as u32 <= c as u32 + 1 && nc != 'S' && nc != 'E' {
            succ.push(np);
        }
    }

    succ
}

fn run(m: &Vec<Vec<char>>, start_pos: Pos) -> usize {
    let to_pos = find_in_map(m, 'E').unwrap();

    if let Some(res) = bfs(
        &start_pos,
        |p| successors(&m, *p),
        |p| *p == to_pos
    ) {
        res.len() - 1
    } else {
        99999
    }
}

fn find_smallest_run_from(m: &Vec<Vec<char>>, c: char) -> usize {
    find_all_in_map(&m, c).iter()
        .map(|from_pos| run(&m, *from_pos))
        .min()
        .unwrap()
}

fn main() {
    let m = parse("input.txt");
    let from_pos = find_in_map(&m, 'S').unwrap();
    println!("#1 {}", run(&m, from_pos));        
    println!("#2 {}", find_smallest_run_from(&m, 'a'));
}

#[test]
fn test() {
    let m = parse("input.txt_test");
    assert_eq!(31, run(&m, find_in_map(&m, 'S').unwrap()));
    assert_eq!(29, find_smallest_run_from(&m, 'a'));
}