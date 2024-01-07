use std::collections::{HashMap, HashSet};
use std::{fs, fmt::Debug};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AreaType {
    Ground,
    Wall,
    Slope(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Neighbour {
    position: Position,
    weight: usize,
}

impl Position {
    fn successors(&self, map: &HashMap<Position, AreaType>, seen: &HashSet<Position>, use_slopes: bool) -> Vec<Position> {
        let mut result = Vec::new();

        let dirs = [(0isize, 1isize), (1isize, 0isize), (0isize, -1isize), (-1, 0)];
        let allowed_slopes = ['v', '>', '<', '^'];

        for (idx_dir, dir) in dirs.iter().enumerate() {
            let mut new_position = *self;
            new_position.x += dir.0;
            new_position.y += dir.1;

            if seen.contains(&new_position) || !map.contains_key(&new_position){
                continue;
            }

            let allowed_slope = allowed_slopes[idx_dir];

            if use_slopes {
                if *map.get(&new_position).unwrap() != AreaType::Ground && *map.get(&new_position).unwrap() != AreaType::Slope(allowed_slope) {
                    continue;
                }
            } else if *map.get(&new_position).unwrap() == AreaType::Wall {
                    continue;
            }

            result.push(new_position);
        }

        result
    }
}

fn read(fp: &str) -> (HashMap<Position, AreaType>, Position, Position) {
    let contents = fs::read_to_string(fp).expect("A file to open");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut map = HashMap::new();
    let mut starting_position = None;
    let mut ending_position = None;
    
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = Position{x: x as isize, y: y as isize};

            let area_type = match c {
                '.' => AreaType::Ground,
                '#' => AreaType::Wall,
                '>' | 'v' | '^' | '<' => AreaType::Slope(c),
                _ => unreachable!(),
            };

            if area_type == AreaType::Wall {
                continue;
            }

            if area_type == AreaType::Ground {
                if starting_position.is_none() {
                    starting_position = Some(pos);
                }
                ending_position = Some(pos);
            }

            map.insert(pos, area_type);
        }
    }

    (map, starting_position.unwrap(), ending_position.unwrap())
}

fn compute_neighbours(map: &HashMap<Position, AreaType>, use_slopes: bool) -> HashMap<Position, HashSet<Neighbour>> {
    let mut neighbours = HashMap::new();

    for position in map.keys() {
        let neighbour_positions = position.successors(map, &HashSet::new(), use_slopes);
        let mut neighbours_set = HashSet::new();

        for neighbour_position in neighbour_positions {
            neighbours_set.insert(Neighbour{
                position: neighbour_position,
                weight: 1,
            });
        }

        neighbours.insert(*position, neighbours_set);
    }

    neighbours
}

// removes all positions with 2 neighbours
fn contract_neighbours(orig_neighbours: &HashMap<Position, HashSet<Neighbour>>) -> HashMap<Position, HashSet<Neighbour>> {
    let mut final_neighbours = orig_neighbours.clone();
    let mut ignored_elements = HashSet::new();
    
    loop {
        let neighbours = final_neighbours.clone();
        let removable_neighbour = neighbours.iter().find(|&n| n.1.len() == 2 && !ignored_elements.contains(n.0));
        if removable_neighbour.is_none() {
            break;
        }

        // println!();
        // / println!("removing {:?}", removable_neighbour);

        let removable_neighbour = removable_neighbour.unwrap();
        let mut neighbours_iter = removable_neighbour.1.iter();
        let node0 = neighbours_iter.next().unwrap();
        let node1 = neighbours_iter.next().unwrap();

        if final_neighbours.get(&node0.position).unwrap().len() != 2 {
            ignored_elements.insert(*removable_neighbour.0);
            continue;
        }

        if final_neighbours.get(&node1.position).unwrap().len() != 2 {
            ignored_elements.insert(*removable_neighbour.0);
            continue;
        }

        // println!("node0:{:?} node1:{:?}", node0, node1);

        let node0_nb_set = final_neighbours.get_mut(&node0.position).unwrap();
        // println!("node0 set:{:?}", node0_nb_set);
        let node0_nb_set_element = node0_nb_set.iter().cloned().find(|nb| nb.position == *removable_neighbour.0).unwrap();

        node0_nb_set.remove(&node0_nb_set_element);
        node0_nb_set.insert(Neighbour{position: node1.position, weight: node1.weight + node0_nb_set_element.weight});

        let node1_nb_set = final_neighbours.get_mut(&node1.position).unwrap();
        // println!("node1 set:{:?}", node1_nb_set);
        let node1_nb_set_element = node1_nb_set.iter().cloned().find(|nb| nb.position == *removable_neighbour.0).unwrap();

        node1_nb_set.remove(&node1_nb_set_element);
        node1_nb_set.insert(Neighbour{position: node0.position, weight: node0.weight + node1_nb_set_element.weight});

        final_neighbours.remove(removable_neighbour.0);

        // println!("final:");
        // for f in &final_neighbours {
        //    println!("- {:?}", f);
        // }
    }

    final_neighbours
}

fn dfs(neighbours_map: &HashMap<Position, HashSet<Neighbour>>, current_position: Position, ending_position: Position, seen: &mut HashMap<Position, usize>) -> usize {
    let mut max_value = 0;
    if current_position == ending_position {
        let res = seen.iter().map(|(_, &p)| p).sum();
        // println!("{:?}", res);
        return res;
    }

    let neighbours = neighbours_map.get(&current_position).unwrap();

    for successor in neighbours {
        if seen.contains_key(&successor.position) {
            continue;
        }

        seen.insert(successor.position, successor.weight);

        max_value = std::cmp::max(dfs(neighbours_map, successor.position, ending_position, seen), max_value);

        seen.remove(&successor.position);
    }

    max_value
}

fn main() {
    let (map, starting_position, ending_position) = read("input.txt");

    let mut seen = HashMap::new();
    let neighbours = compute_neighbours(&map, true);
    println!("#1 {:?}", dfs(&neighbours, starting_position, ending_position, &mut seen));


    let mut seen = HashMap::new();
    let neighbours = compute_neighbours(&map, false);
    let neighbours = contract_neighbours(&neighbours);
    println!("#2 {:?}", dfs(&neighbours, starting_position, ending_position, &mut seen));
}
