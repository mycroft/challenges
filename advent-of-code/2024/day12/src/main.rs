use std::fs;
use std::collections::{HashMap, HashSet};

type Pos = (isize, isize);

static CORNERS: [[Pos; 3]; 4] = [
    [
        (-1, -1),
        (-1, 0),
        (0, -1),
    ],
    [
        (1, -1),
        (1, 0),
        (0, -1),
    ],
    [
        (1, 1),
        (1, 0),
        (0, 1),
    ],
    [
        (-1, 1),
        (-1, 0),
        (0, 1),
    ],
];


fn read_input(fp: &str) -> HashMap<Pos, char> {
    let contents = fs::read_to_string(fp).expect("Error reading the file");
    let mut map = HashMap::new();

    for (i, line) in contents.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            map.insert((i as isize, j as isize), c);
        }
    }

    map
}

fn find_all_adjacent_positions(map: &HashMap<Pos, char>, initial: Pos) -> HashSet<Pos> {
    let mut to_visit = vec![initial];
    let mut field = HashSet::new();

    let field_name = map[&initial];

    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    loop {
        if to_visit.is_empty() {
            break;
        }

        let current = to_visit.pop().unwrap();

        if field.contains(&current) {
            continue;
        }

        field.insert(current);

        for dir in &dirs {
            let new_pos = (current.0 + dir.0, current.1 + dir.1);

            if map.contains_key(&new_pos) && map[&new_pos] == field_name {
                to_visit.push(new_pos);
            }
        }
    }

    field
}

fn find_all_fields(map: &HashMap<Pos, char>) -> Vec<HashSet<Pos>> {
    let mut fields = Vec::new();
    let mut visited: HashSet<Pos> = HashSet::new();

    for (pos, _) in map {
        if visited.contains(pos) {
            continue;
        }

        let field = find_all_adjacent_positions(map, *pos);
        visited.extend(field.iter());
        fields.push(field);
    }

    fields
}

fn compute_field_perimeter(field: &HashSet<Pos>) -> usize {
    let mut perimeter = 0;

    let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];

    for pos in field {
        for dir in &dirs {
            let new_pos = (pos.0 + dir.0, pos.1 + dir.1);

            if !field.contains(&new_pos) {
                perimeter += 1;
            }
        }
    }

    perimeter
}

fn get_field_area(field: &HashSet<Pos>) -> usize {
    field.len()
}

fn solve_step1(field: &HashMap<Pos, char>) -> usize {
    let all_fields = find_all_fields(field);
    let mut sum = 0;

    for field in all_fields {
        sum += compute_field_perimeter(&field) * get_field_area(&field);
    }

    sum
}

fn corner_for_pos(field: &HashSet<Pos>, pos: &Pos) -> usize {
    CORNERS
        .iter()
        .filter(|corner| {
            let opposite = field.get(&(pos.0 + corner[0].0, pos.1 + corner[0].1));
            let first = field.get(&(pos.0 + corner[1].0, pos.1 + corner[1].1));
            let second = field.get(&(pos.0 + corner[2].0, pos.1 + corner[2].1));

            (second.is_none() && first.is_none())
                || (opposite.is_none() && first.is_some() && second.is_some())              
        })
        .count()
}

fn corners(field: &HashSet<Pos>) -> usize {
    let mut corners = 0;

    for pos in field {
        corners += corner_for_pos(field, pos);
    }

    corners
}

fn solve_step2(field: &HashMap<Pos, char>) -> usize {
    let fields = find_all_fields(field);
    let mut result = 0;

    for field in fields {
        result += corners(&field) * get_field_area(&field);
    }

    result
}

fn main() {
    let input = read_input("input.txt");

    println!("#1: {}", solve_step1(&input));
    println!("#2: {}", solve_step2(&input));
}

#[test]
fn test_sample() {
    let input = read_input("input_test.txt");
    assert_eq!(solve_step1(&input), 140);
    assert_eq!(solve_step2(&input), 80);

    let input = read_input("input_test1.txt");
    assert_eq!(solve_step1(&input), 772);
    assert_eq!(solve_step2(&input), 436);

    let input = read_input("input_test2.txt");
    assert_eq!(solve_step1(&input), 1930);
    assert_eq!(solve_step2(&input), 1206);

    let input = read_input("input_test4.txt");
    assert_eq!(solve_step2(&input), 236);

    let input = read_input("input_test5.txt");
    assert_eq!(solve_step2(&input), 368);
}