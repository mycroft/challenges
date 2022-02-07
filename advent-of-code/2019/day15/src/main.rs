//
// Advent of Code 2019, day 15.
//
mod intcode;
use intcode::{IntCode, str_to_prog};

use std::collections::{HashMap, HashSet};
use pathfinding::prelude::astar;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct Node {
    pos:       Pos,
    vm:        IntCode,
    is_oxygen: bool,
}

impl Pos {
    fn successors(self, map: &mut HashMap<Pos, (IntCode, usize)>) -> Vec<(Pos, u32)> {
        let possible_moves = [
            (1, (0, -1)),
            (2, (0, 1)),
            (3, (-1, 0)),
            (4, (1, 0)),
        ];

        let mut result = vec![];

        // get original VM.
        let orig_state = map.get(&self).unwrap().clone();

        // available: commands: north (1), south (2), west (3), and east (4).
        for command in possible_moves {
            let new_pos = Pos {
                x: self.x + command.1.0,
                y: self.y + command.1.1,
            };

            // Check if we know this map position.
            if map.contains_key(&new_pos) {
                let state = map.get(&new_pos).unwrap();

                if state.1 == 99 {
                    // We consider the state is unpure: we need to recompute it.
                } else {
                    if state.1 != 0 {
                        result.push((new_pos, 1));
                    }
                    continue;
                }
            }

            let mut vm = orig_state.0.clone();

            vm.push(command.0);
            vm.execute();

            if let Some(v) = vm.output.last() {
                if v != &0 {
                    // We store this into the map.
                    map.insert(new_pos, (vm.clone(), *v as usize));
                    result.push((new_pos, 1));
                }
            }
        }

        result
    }
}


fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("invalid file");
    let contents = contents.trim();
    let program = str_to_prog(contents);

    let init_vm = IntCode::new(&program, &[]);

    let mut map: HashMap<Pos, (IntCode, usize)> = HashMap::new();

    map.insert(
        Pos{x: 0, y: 0},
        (init_vm, 99),
    );

    // fill the map.
    astar(
        &Pos{x: 0, y: 0},
        |p| p.successors(&mut map),
        |_| 0,
        |_| false,
    );

    // find the destination
    let dest = *map.iter().find(|(_, v)| v.1 == 2).unwrap().0;
    println!("Destination is {:?}", dest);

    // recompute the thing
    let result: Option<(Vec<Pos>, u32)> = astar(
        &Pos{x: 0, y: 0},
        |p| p.successors(&mut map),
        |_| 0,
        |&p| p == dest,
    );
    println!("#1: {}", result.unwrap().1); // 238

    // step 2: filling things.
    let mut turns = 0;
    let mut stack : HashSet<Pos> = HashSet::new();
    let mut filled: HashSet<Pos> = HashSet::new();

    let possible_moves = [
        (0, -1), (0, 1),
        (-1, 0), (1, 0),
    ];

    stack.insert(dest);
    filled.insert(dest);

    loop {
        let mut new_stack: HashSet<Pos> = HashSet::new();

        for pos in stack {
            for delta in possible_moves {
                let new_pos = Pos{x: pos.x + delta.0, y: pos.y + delta.1 };
                if filled.contains(&new_pos) || !map.contains_key(&new_pos) {
                    continue;
                }

                let state = map.get(&new_pos).unwrap();
                if state.1 == 0 {
                    continue;
                }

                filled.insert(new_pos);
                new_stack.insert(new_pos);
            }
        }

        
        if new_stack.is_empty() {
            break;
        }
        turns += 1;

        stack = new_stack;
    }

    println!("#2: {}", turns);
}
