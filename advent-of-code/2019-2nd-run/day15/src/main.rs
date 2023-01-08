use std::collections::{HashMap, HashSet};
use intcode::{parse, Machine};

use pathfinding::prelude::bfs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Wall,
    Empty,
    OxygenSystem,
}

// Tile describe a tile in the world: Its kind, and the machine state to get there.
#[derive(Debug, Clone)]
struct Tile {
    kind: Kind,
    machine: Machine
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

fn successors(world: &mut HashMap<Position, Tile>, position: &Position) -> Vec<Position> {
    let mut result = Vec::new();

    let mut directions = HashMap::new();
    directions.insert(1isize, (0, -1));
    directions.insert(2, (0, 1));
    directions.insert(3, (-1, 0));
    directions.insert(4, (1, 0));

    // get initial machine
    let machine = world.get(position).unwrap().machine.clone();

    // lets run the different positions we can go
    for (input, direction) in directions {
        let new_position = Position{
            x: position.x + direction.0,
            y: position.y + direction.1,
        };

        // if the next position is a wall, do not compute anything.
        if world.contains_key(&new_position) {
            let next_tile = world.get(&new_position).unwrap();
            if next_tile.kind == Kind::Wall {
                continue;
            }

            result.push(new_position);
            continue;
        }

        // clone the machine
        let mut cloned_machine = machine.clone();

        // run the machine with the given input
        cloned_machine.add_input(input);

        // run machine
        cloned_machine.run();

        // get output
        let output = cloned_machine.get_output();
        cloned_machine.clean_output();

        // println!("{new_position:?}: {:?} // {:?}", output, cloned_machine.get_output());

        let tile_kind = match output[0] {
            1 => Kind::Empty,
            0 => Kind::Wall,
            2 => Kind::OxygenSystem,
            _ => unreachable!()
        };

        world.insert(new_position, Tile{ kind: tile_kind, machine: cloned_machine});
        if tile_kind == Kind::Wall {
            continue;
        }

        result.push(new_position);
    }

    result
}

fn display(world: &HashMap<Position, Tile>) {
    let min_x = world.iter().map(|(&pos, _)| pos.x).min().unwrap();
    let max_x = world.iter().map(|(&pos, _)| pos.x).max().unwrap();
    let min_y = world.iter().map(|(&pos, _)| pos.y).min().unwrap();
    let max_y = world.iter().map(|(&pos, _)| pos.y).max().unwrap();

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if let Some(o) = world.get(&Position{x, y}) {
                let c = match o.kind {
                    Kind::Empty => ".",
                    Kind::Wall => "#",
                    Kind::OxygenSystem => "O",
                };
                print!("{c}");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn fill(world: &HashMap<Position, Tile>) -> isize {
    let mut filled = HashSet::new();
    let oxygen_system_position = *world.iter().find(|&(_, t)| t.kind == Kind::OxygenSystem).unwrap().0;

    let mut stack = Vec::new();

    filled.insert(oxygen_system_position);
    stack.push(oxygen_system_position);

    let mut steps = 0;

    let directions = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
    ];

    loop {
        let mut next_stack = Vec::new();

        // for each element in stack, we search for possible elements not yet filled.
        loop {
            let position = stack.pop().unwrap();

            // this position is considered as filled.
            filled.insert(position);

            for direction in directions {
                let new_position = Position{x: position.x + direction.0, y: position.y + direction.1};
                if filled.contains(&new_position) {
                    continue;
                }

                let tile = world.get(&Position{x: position.x + direction.0, y: position.y + direction.1}).unwrap();
                if tile.kind == Kind::Wall {
                    continue;
                }

                // add the position to the next stack.
                next_stack.push(new_position);
            }

            if stack.is_empty() {
                break;
            }
        }

        stack = next_stack;
        
        // if there is no longer any element in the stack, we stop processing and return the number of steps
        if stack.is_empty() {
            break;
        }

        steps += 1;
    }

    steps
}

fn main() {
    let machine = Machine::new(&parse("input.txt"));
    let mut world = HashMap::new();

    world.insert(Position{x: 0, y: 0}, Tile{kind: Kind::Empty, machine });

    successors(&mut world, &Position{x: 0, y: 0});

    // Initial bfs run to build the map.
    bfs(
        &Position { x: 0, y: 0 },
        |pos| successors(&mut world, pos),
        |_| false,
    );

    // display(&world);

    // New bfs run with a target
    let target = *world.iter().find(|&(_, t)| t.kind == Kind::OxygenSystem).unwrap().0;
    let path = bfs(
        &Position { x: 0, y: 0 },
        |pos| successors(&mut world, pos),
        |pos| pos == &target,
    ).unwrap();

    println!("#1 {}", path.len() - 1);

    // fill the map starting from the 0 until all empty space is full.
    println!("#2 {}", fill(&world));
}
