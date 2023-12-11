use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};

/*
| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum PipeType {
    NorthSouth,
    WestEast,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Ground,
    StartingPoint,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Location {
    x: isize,
    y: isize,
}

fn get_dirs_for_type(pipe_type: &PipeType) -> Vec<(isize, isize)> {
    match pipe_type {
        PipeType::Ground => [].into(),
        PipeType::StartingPoint => [(-1, 0), (1, 0), (0, 1), (0, -1)].into(),
        PipeType::NorthSouth => [(0, -1), (0, 1)].into(),
        PipeType::NorthWest => [(0, -1), (-1, 0)].into(),
        PipeType::NorthEast => [(0, -1), (1, 0)].into(),
        PipeType::SouthEast => [(0, 1), (1, 0)].into(),
        PipeType::SouthWest => [(0, 1), (-1, 0)].into(),
        PipeType::WestEast => [(-1, 0), (1, 0)].into(),
    } 
}

fn find_neighbours(pipe_map: &HashMap<Location, PipeType>, current_location: &Location) -> Vec<Location> {
    let mut results = Vec::new();

    if !pipe_map.contains_key(current_location) {
        return Vec::new();
    }

    let current_pipe_type = pipe_map.get(current_location).unwrap();
    let dirs: Vec<(isize, isize)> = get_dirs_for_type(current_pipe_type);

    for dir in dirs {
        let neighbours_locations = Location{
            x: current_location.x + dir.0,
            y: current_location.y + dir.1,
        };
        if !pipe_map.contains_key(&neighbours_locations) {
            continue
        }

        let neighbour_pipe_type = pipe_map.get(&neighbours_locations).unwrap();
        for neighbour_dirs in get_dirs_for_type(neighbour_pipe_type) {
            if -neighbour_dirs.0== dir.0 && -neighbour_dirs.1 == dir.1 {
                results.push(neighbours_locations);
            }
        }
    }

    results
}

fn find_starting_location(pipe_map: &HashMap<Location, PipeType>) -> Location {
    *pipe_map.iter().find(|(_, &pipe_type)| pipe_type == PipeType::StartingPoint).unwrap().0
}

// visited contains the starting point; current_location on the starting point neighbour
fn find_loop(pipe_map: &HashMap<Location, PipeType>, visited: &mut HashSet<Location>, current_location: &Location) -> bool {
    let mut to_visit = VecDeque::new();

    to_visit.push_back(*current_location);

    loop {
        if to_visit.is_empty() {
            break;
        }

        let current_location = to_visit.pop_front().unwrap();
        visited.insert(current_location);

        let neighbours = find_neighbours(pipe_map, &current_location);
        for neighbour_location in neighbours {
            let neighbour_type: &PipeType = pipe_map.get(&neighbour_location).unwrap();
    
            // ok, we finished the loop
            if *neighbour_type == PipeType::StartingPoint && visited.len() > 2 {
                return true;
            }

            // we skip pipe we already visited
            if visited.contains(&neighbour_location) {
                continue;
            }

            to_visit.push_back(neighbour_location);    
        }
    }

    false
}

fn process_file(filename: &str, step1: bool) -> usize {
    let contents = fs::read_to_string(filename).expect("a file to open");
    let lines = contents.lines(). collect::<Vec<&str>>();

    let mut max_x = 0_isize;
    let max_y = lines.len() as isize;

    let mut pipe_map : HashMap<Location, PipeType> = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        max_x = line.len() as isize;
        for (x, c) in line.chars().enumerate() {
            let pipe_type = match c {
                '.' => PipeType::Ground,
                'S' => PipeType::StartingPoint,
                '|' => PipeType::NorthSouth,
                '-' => PipeType::WestEast,
                'L' => PipeType::NorthEast,
                'J' => PipeType::NorthWest,
                '7' => PipeType::SouthWest,
                'F' => PipeType::SouthEast,
                _ => unreachable!(),
            };

            pipe_map.insert(Location{x: x as isize, y: y as isize}, pipe_type);
        }
    }

    let starting_location = find_starting_location(&pipe_map);
    let mut only_pipes = HashSet::new();
    let mut max_loop_size = 0;
    let mut best_dirs = Vec::new();
    
    // starting point: We look the 4 possible directions
    for dir in get_dirs_for_type(pipe_map.get(&starting_location).unwrap()) {
        let mut visited = HashSet::new();
        visited.insert(starting_location);

        let next_location = Location{
            x: starting_location.x + dir.0,
            y: starting_location.y + dir.1,
        };

        let res = find_loop(&pipe_map, &mut visited, &next_location);

        if ! res {
            continue;
        }

        if visited.len() >= max_loop_size {

            if visited.len() > max_loop_size {
                best_dirs = Vec::new();
            }

            only_pipes = visited.clone();
            max_loop_size = visited.len();
            best_dirs.push(dir);
        }
    }

    let pipe_type = if best_dirs.contains(&(0, -1)) {
        if best_dirs.contains(&(-1, 0)) {
            PipeType::NorthWest
        } else {
            PipeType::NorthEast
        }
    } else if best_dirs.contains(&(-1, 0)) {
        PipeType::SouthWest
    } else {
        PipeType::SouthEast
    };

    // Modify starting point
    pipe_map.insert(starting_location, pipe_type);

    if step1 {
        only_pipes.len() / 2
    } else {
        compute_step2(max_x, max_y, &only_pipes, &pipe_map)
    }
}

// scan each line, each entry, for each entry not in loop_pipes, check how many | or LJ/L7/?---? we have. If odd, then enclosed, if not, then ignore.
fn compute_step2(max_x: isize, max_y: isize, loop_pipes: &HashSet<Location>, pipe_map: &HashMap<Location, PipeType>) -> usize {
    let mut res = 0;

    //for lp in loop_pipes {
    //    println!("{:?}", lp);
    //}

    for x in 0..max_x {
        for y in 0..max_y {
            let loc = Location{x, y};
            if loop_pipes.contains(&loc) {
                continue;
            }

            let mut a = 0;
            let mut b = 0;
            let mut currently_in_section : Option<isize> = None;

            for pipe_x in 0..max_x {
                if pipe_x == x {
                    // if currently_in_section.is_some() {
                    //     println!("{:?} at {:?}", currently_in_section, loc);
                    // }
                    
                    //currently_in_section = None;
                    continue;
                }
                let loc_temp = Location{x: pipe_x, y};
                if loop_pipes.contains(&loc_temp) {
                    let pipe_type = *pipe_map.get(&loc_temp).unwrap();
                    if pipe_type == PipeType::NorthSouth {
                        if pipe_x > x {
                            a += 1;
                        } else {
                            b += 1;
                        }
                    } else if pipe_type == PipeType::NorthEast || pipe_type == PipeType::NorthWest || pipe_type == PipeType::SouthEast || pipe_type == PipeType::SouthWest {
                        if currently_in_section.is_none() { // oh just kill me
                            if pipe_type == PipeType::NorthEast || pipe_type == PipeType::NorthWest {
                                // going down!
                                currently_in_section = Some(1);
                            } else {
                                currently_in_section = Some(-1);
                            }
                        } else {
                            if (currently_in_section == Some(1) && (pipe_type == PipeType::SouthEast || pipe_type == PipeType::SouthWest)) || (currently_in_section == Some(-1) && (pipe_type == PipeType::NorthEast || pipe_type == PipeType::NorthWest)) {
                                if pipe_x > x {
                                    a += 1;
                                } else {
                                    b += 1;
                                }
                            }

                            currently_in_section = None;
                        }
                    }
                }
            }

            if a % 2 == 1 && b % 2 == 1 {
                // println!("> {:?} (a:{}/b:{})", loc, a, b);
                res += 1
            }
        }
    }

    res
}

fn main() {
    println!("#1 {:?}", process_file("input.txt", true)); // 7066
    println!("#2 {:?}", process_file("input.txt", false)); // 401
}

#[test]
fn step1() {
    assert_eq!(8, process_file("input_test.txt", true));
    assert_eq!(4, process_file("input_test3.txt", true));
    assert_eq!(7066, process_file("input.txt", true));
}

#[test]
fn step2() {
    assert_eq!(1, process_file("input_test3.txt", false));
    assert_eq!(4, process_file("input_test2.txt", false));
    assert_eq!(8, process_file("input_test4.txt", false));
    assert_eq!(10, process_file("input_test5.txt", false));
}