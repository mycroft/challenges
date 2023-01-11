use std::fs::read_to_string;
use std::collections::{BTreeSet, HashSet, HashMap};
use pathfinding::prelude::bfs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

#[derive(Debug)]
struct Map {
    objects: HashMap<Position, char>,
    ground: HashSet<Position>,
    pathes: HashMap<(char, char), (HashSet<char>, usize)>
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct State {
    characters: Vec<char>
}

fn parse(fp: &str, patch: bool) -> Map {
    let contents = read_to_string(fp).expect("file to open");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut ground = HashSet::new();
    let mut objects = HashMap::new();

    for (y, &line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                continue;
            }

            ground.insert(Position(x as isize, y as isize));

            if c == '.' {
                continue;
            }

            objects.insert(Position(x as isize, y as isize), c);
        }
    }

    if patch {
        // find '@'
        let position = objects.iter().find(|(_, &c)| c == '@').unwrap().0.clone();
        let directions = [(0isize, 0isize), (1, 0), (-1, 0), (0, 1), (0, -1)];

        // remove '@' from objects
        objects.remove(&position);

        // remove former grounds
        for direction in directions {
            ground.remove(&Position(position.0 + direction.0, position.1 + direction.1));
        }

        // add new objects: @, #, $, %
        objects.insert(Position(position.0 - 1, position.1 - 1), '@');
        objects.insert(Position(position.0 - 1, position.1 + 1), '#');
        objects.insert(Position(position.0 + 1, position.1 - 1), '$');
        objects.insert(Position(position.0 + 1, position.1 + 1), '%');
    }

    // Compute all pathes
    let mut expected_keys = objects.iter().map(|(_, c)| *c).filter(|&c| c >= 'a' && c <= 'z').collect::<Vec<char>>();
    expected_keys.push('@');
    if patch {
        expected_keys.push('#');
        expected_keys.push('$');
        expected_keys.push('%');
    }

    let mut current_map = Map{ ground, objects, pathes: HashMap::new() };

    for from_idx in 0..(expected_keys.len()-1) {
        for to_idx in from_idx+1..expected_keys.len() {
            // println!("Finding path from {} to {}", expected_keys[from_idx], expected_keys[to_idx]);

            let from_pos = current_map.objects.iter().filter(|(_, &c)| c == expected_keys[from_idx]).next().unwrap().0;
            let to_pos = current_map.objects.iter().filter(|(_, &c)| c == expected_keys[to_idx]).next().unwrap().0;

            let path = bfs(
                from_pos,
                |p| successors(&current_map, p, &HashSet::new(), false), // successors
                |p| p == to_pos
            );

            if path.is_none() {
                continue;
            }

            let path = path.unwrap();

            // find required keys
            let mut required_keys = HashSet::new();
            for p in &path {
                if let Some(&object) = current_map.objects.get(p) {
                    if object >= 'A' && object <= 'Z' {
                        required_keys.insert(object);
                    }
                }
            }

            // println!("{} -> {} length is {} and requires keys:{:?}", expected_keys[from_idx], expected_keys[to_idx], path.len(), &required_keys);

            current_map.pathes.insert((expected_keys[from_idx], expected_keys[to_idx]), (required_keys.clone(), path.len()));
            current_map.pathes.insert((expected_keys[to_idx], expected_keys[from_idx]), (required_keys, path.len()));
        }
    }

    current_map
}

fn successors(m: &Map, position: &Position, owned_keys: &HashSet<char>, force_doors: bool) -> Vec<Position> {
    let directions = [(0isize, 1isize), (1, 0), (0, -1), (-1, 0)];
    let mut result = Vec::new();

    for direction in directions {
        let new_position = Position(position.0 + direction.0, position.1 + direction.1);
        if !m.ground.contains(&new_position) {
            continue;
        }

        if !force_doors {
            result.push(new_position);
            continue;
        }

        if let Some(&object) = m.objects.get(&new_position) {
            if object >= 'A' && object <= 'Z' && !owned_keys.contains(&object.to_ascii_lowercase()) {
                // println!("Found object {} but do not have key (keys: {owned_keys:?}).", object);
                continue;
            }
        }

        result.push(new_position);
    }

    result
}

fn collect_keys(m: &Map, keys: &BTreeSet<char>, current_state: State, cache: &mut HashMap<(State, BTreeSet<char>), usize>) -> usize {
    let expected_keys = m.objects.iter().map(|(_, c)| *c).filter(|&c| c >= 'a' && c <= 'z').collect::<Vec<char>>();
    let mut scores = Vec::new();

    // println!("collect_keys called with state {current_state:?} with keys {keys:?}");

    for idx in 0..current_state.characters.len() {
        for &expected_key in &expected_keys {
            if keys.contains(&expected_key) {
                // Already have this key
                continue;
            }
    
            // from current_key to expected_key
            let path = m.pathes.get(&(current_state.characters[idx], expected_key));
            if path.is_none() {
                continue;
            }
            let path = path.unwrap();
    
            // rewrite this
            let mut have_all_keys = true;
            for required_key in &path.0 {
                if !keys.contains(&required_key.to_ascii_lowercase()) {
                    have_all_keys = false;
                }
            }
            if !have_all_keys {
                // println!("idx:{idx} Did not have all the keys to go to {expected_key}");
                continue;
            }
    
            // println!("idx:{idx} Found a path from {} to key:{expected_key} of size:{} (patches cache size: {})", current_state.characters[idx], path.1, m.pathes.len());

            let mut new_keys = keys.clone();
            new_keys.insert(expected_key);

            let mut new_state = current_state.clone();
            new_state.characters[idx] = expected_key;
    
            let score = if cache.contains_key(&(new_state.clone(), new_keys.clone())) {
                *cache.get(&(new_state, new_keys)).unwrap()
            } else {
                let score = collect_keys(m, &new_keys, new_state.clone(), cache);
                cache.insert((new_state, new_keys.clone()), score);
                
                score
            };
    
            scores.push(path.1 - 1 + score)
        }    
    }

    // we have all the keys. Return the minimal found score.
    *scores.iter().min().unwrap_or(&0)
}

fn collect_all_keys(m: &Map, multiple: bool) -> usize {
    // cache key: (the current character, BtreeSet)
    let mut cache: HashMap<(State, BTreeSet<char>), usize> = HashMap::new();
    let state = if !multiple {
        State{characters: ['@'].to_vec()}
    } else {
        State{characters: [
            '@', '#', '$', '%'
        ].to_vec()}
    };

    collect_keys(m, &BTreeSet::new(), state, &mut cache)
}

fn main() {
    let m = parse("input.txt", false);
    println!("#1 {}", collect_all_keys(&m, false)); // 2946

    let m = parse("input.txt", true);
    println!("#2 {}", collect_all_keys(&m, true)); // 1222
}

#[test]
fn test_samples() {
    let samples = [
        ("input.txt_test0", 8),
        ("input.txt_test1", 86),
        ("input.txt_test2", 132),
        ("input.txt_test3", 136),
        ("input.txt_test4", 81),
    ];

    for sample in samples {
        let m = parse(sample.0, false);
        assert_eq!(
            sample.1,
            collect_all_keys(&m, false)
        );
    }
}

#[test]
fn test_samples_2() {
    let samples = [
        ("input2.txt_test0", 8),
        ("input2.txt_test1", 24),
        ("input2.txt_test2", 32),
        ("input2.txt_test3", 72),
    ];
    for sample in samples {
        let m = parse(sample.0, true);
        assert_eq!(
            sample.1,
            collect_all_keys(&m, true)
        );
        println!("---");
    }
}
