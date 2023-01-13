use std::fs::read_to_string;
use std::collections::{BTreeMap, BTreeSet};
use pathfinding::prelude::bfs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position(isize, isize, isize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum DoorType {
    Inner,
    Outer
}


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Door {
    name: String,
    door_type: DoorType,
}

#[derive(Debug)]
struct Map {
    ground: BTreeSet<Position>,
    doors: BTreeMap<Position, Door>,
    vortexes: BTreeMap<String, BTreeSet<Position>>,
}

impl Map {
    fn door_positions(&self, name: String) -> Vec<Position> {
        self.doors
            .iter()
            .filter(|(_pos, door)| door.name == name)
            .map(|(pos, _)| *pos)
            .collect()
    }

    fn successors(&self, pos: &Position, is_3d: bool) -> Vec<Position> {
        let mut result = Vec::new();

        let directions = [
            (-1, 0), (1, 0), (0, -1), (0, 1)
        ];

        // Find if there is ground.
        for dir in directions {
            if self.ground.contains(&Position(pos.0 + dir.0, pos.1 + dir.1, 0)) {
                result.push(Position(pos.0 + dir.0, pos.1 + dir.1, pos.2));
            }
        }

        // Find if we're a vortex and need to go the other side
        if let Some(door) = self.doors.get(&Position(pos.0, pos.1, 0)) {
            if let Some(&vortex_position) = self.vortexes
                .get(&door.name)
                .unwrap()
                .iter()
                .find(|vortex_pos| *vortex_pos != &Position(pos.0, pos.1, 0))
            {
                if !is_3d {
                    result.push(vortex_position);
                } else {
                    // We need to know if we can add the vortex (ie: the other side of a gate)
                    // We can only if the current door is an inner door on depth=0 (pos.2=0)
                    if !(door.door_type == DoorType::Outer && pos.2 == 0) {
                        // We add the vortex
                        let new_depth = pos.2 + match door.door_type {
                            DoorType::Inner => 1,
                            DoorType::Outer => -1,
                        };

                        result.push(Position(vortex_position.0, vortex_position.1, new_depth));
                    }
                }
            }
        }

        result
    }
}

// I need to find doors by position to find out if successor 
// to a position is leading to another position...
// I also need to find other doors by name 
fn parse(fp: &str) -> Map {
    let contents = read_to_string(fp).expect("a file to open");
    let mut data: Vec<Vec<char>> = Vec::new();
    let mut ground = BTreeSet::new();
    let mut doors = BTreeMap::new();

    for line in contents.lines() {
        data.push(line.chars().collect())
    }

    // compute ground
    for y in 2..data.len() - 2 {
        for x in 2..data[0].len() - 2 {
            if data[y][x] == '.' {
                ground.insert(Position(x as isize, y as isize, 0));
            }
        }
    }

    let directions = [
        (0isize, 1isize), (0, -1), (1, 0), (-1, 0)
    ];

    // find doors
    for y in 2..data.len() - 2 {
        for x in 2..data[0].len() - 2 {
            for dir in directions {
                let p = (x as isize + dir.0, y as isize + dir.1, 0);

                if data[y][x] != '.' {
                    continue;
                }

                // when going up or left, revert the letters
                if data[p.1 as usize][p.0 as usize] >= 'A' && data[p.1 as usize][p.0 as usize] <= 'Z' {
                    let gate: String = if dir.0 == -1 || dir.1 == -1 {
                        [
                            data[(p.1 + dir.1) as usize][(p.0 + dir.0) as usize],
                            data[p.1 as usize][p.0 as usize],
                        ]
                    } else {
                        [
                            data[p.1 as usize][p.0 as usize],
                            data[(p.1 + dir.1) as usize][(p.0 + dir.0) as usize],
                        ]
                    }.into_iter().collect();

                    let door_type = if x == 2 || y == 2 || x == data[0].len() - 2 - 1 || y == data.len() - 2 - 1 {
                        DoorType::Outer
                    } else {
                        DoorType::Inner
                    };

                    doors.insert(
                        Position(x as isize, y as isize, 0),
                        Door{name: gate, door_type}
                    );
                }
            }
        }
    };

    // compute vortexes: BTreeMap<String, BTreeSet<Position>>
    let mut vortexes: BTreeMap<String, BTreeSet<Position>> = BTreeMap::new();

    for (pos, door) in &doors {
        vortexes.entry(door.name.to_owned()).or_default().insert(*pos);
    }

    Map {
        ground,
        doors,
        vortexes,
    }

}

/*
fn dump(m: &Map) {
    println!("{:?}", m.ground);

    for door in &m.doors {
        println!("{door:?}");
    }

    for vortex in &m.vortexes {
        println!("{vortex:?}");
    }
}
*/

fn step1(fp: &str) -> usize {
    let map = parse(fp);

    let start = map.door_positions("AA".to_string())[0];
    let end = map.door_positions("ZZ".to_string())[0];

    let path = bfs(
        &start,
        |pos| map.successors(pos, false),
        |pos| *pos == end,
    );

    path.unwrap().len() - 1
}

fn step2(fp: &str) -> usize {
    let map = parse(fp);

    let start = map.door_positions("AA".to_string())[0];
    let end = map.door_positions("ZZ".to_string())[0];

    let path = bfs(
        &start,
        |pos| map.successors(pos, true),
        |pos| *pos == end,
    );

    path.unwrap().len() - 1
}

fn main() {
    println!("#1 {}", step1("input.txt")); // 442
    println!("#2 {}", step2("input.txt")); // 5208
}

#[test]
fn test_sample_step1() {
    assert_eq!(
        23,
        step1("input.txt_test0")
    );
    assert_eq!(
        58,
        step1("input.txt_test1")
    );
}

#[test]
fn test_sample_step2() {
    assert_eq!(
        396,
        step2("input.txt_test2")
    );
}