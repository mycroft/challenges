use std::fs;
use std::collections::{HashMap,HashSet};
use pathfinding::prelude::astar;
use pathfinding::prelude::dfs;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    pos: Pos,
    current: Vec<Pos>
}

impl Pos {
    fn neighbours(&self, map: &HashMap<Pos, usize>) -> Vec<Pos> {
        let mut neighbours = Vec::new();
        let current_height = *map.get(&self).unwrap();

        let moves = vec![(0, -1), (0, 1), (-1, 0), (1, 0)];

        for possible_move in moves {
            let npos = Pos{x: self.x + possible_move.0, y: self.y + possible_move.1};

            if npos != *self && map.contains_key(&npos) && *map.get(&npos).unwrap() == current_height + 1 {
                neighbours.push(npos);
            }
        }

        neighbours
    }
}

impl State {
    fn next(&self, map: &HashMap<Pos, usize>) -> Vec<State> {
        let mut states = Vec::new();

        for neighbour in self.pos.neighbours(map) {
            let mut path = self.current.clone();
            path.push(self.pos.clone());

            states.push(
                State{
                    pos: neighbour,
                    current: path,
                },
            );
        }


        states
    }
}

fn read_input(fp: &str) -> (HashMap<Pos, usize>, HashSet<Pos>, HashSet<Pos>) {
    let content = fs::read_to_string(fp).expect("Cannot read file");
    let mut map = HashMap::new();
    let mut starting_points = HashSet::new();
    let mut ending_points = HashSet::new();

    for (i, line) in content.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let height = c.to_digit(10).unwrap() as usize;
            map.insert(Pos{x: j as isize, y: i as isize}, height);

            if height == 0 {
                starting_points.insert(Pos{x: j as isize, y: i as isize});
            } else if height == 9 {
                ending_points.insert(Pos{x: j as isize, y: i as isize});
            }
        }
    }

    (map, starting_points, ending_points)
}

fn finding_pathes(map: &HashMap<Pos, usize>, starting_points: &HashSet<Pos>, ending_points: &HashSet<Pos>) -> usize {
    let mut pathes = Vec::new();

    for starting_point in starting_points {
        for ending_point in ending_points {
            let res = astar(
                starting_point,
                |p| p.neighbours(map).into_iter().map(|p| (p, 1)),
                |_p| 0,
                |p| *p == *ending_point
            );

            if let Some((path, _cost)) = res {
                pathes.push(path);
            }
        }
    }

    pathes.len()
}

fn is_known(known: &Vec<Vec<Pos>>, path: &Vec<Pos>, ending_point: Pos) -> bool {

    let mut path = path.clone();
    path.push(ending_point);

    for k in known.iter() {
        if k.len() != path.len() {
            continue;
        }

        let mut is_same = true;
        for i in 0..k.len() {
            if k[i] != path[i] {
                is_same = false;
                break;
            }
        }

        if is_same {
            return true;
        }
    }

    false
}

fn find_all_pathes(map: &HashMap<Pos, usize>, starting_points: &HashSet<Pos>, ending_points: &HashSet<Pos>) -> usize {
    let mut pathes = Vec::new();

    for starting_point in starting_points {
        for ending_point in ending_points {
            let initial_state = State{pos: *starting_point, current: Vec::new()};
            let mut known = Vec::new();

            loop {
                let res = dfs(
                    initial_state.clone(),
                    |s| s.next(map),
                    |s| s.pos == *ending_point && !is_known(&known, &s.current, s.pos),
                );

                if let Some(path) = res {
                    let mut path_vec = Vec::new();
                    for state in path.iter() {
                        path_vec.push(state.pos);
                    }
                    known.push(path_vec.clone());
                    pathes.push(path_vec);
                } else {
                    break;
                }
            }
        }
    }

    pathes.len()
}


fn main() {
    let (map, starting_points, ending_points) = read_input("input.txt");

    let _pathes = finding_pathes(&map, &starting_points, &ending_points);
    println!("#1 {}", _pathes);

    let _all_pathes = find_all_pathes(&map, &starting_points, &ending_points);
    println!("#2 {}", _all_pathes);
}
