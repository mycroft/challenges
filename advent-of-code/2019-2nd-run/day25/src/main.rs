use std::{collections::{HashMap, VecDeque}};
use intcode::{Machine, parse};
use pathfinding::prelude::bfs;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn invert(&self) -> Direction {
        match self {
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::North => Direction::South,
            Direction::South => Direction::North
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Direction::North => "north",
            Direction::East => "east",
            Direction::West => "west",
            Direction::South => "south",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "north" => Direction::North,
            "east" => Direction::East,
            "south" => Direction::South,
            "west" => Direction::West,
            _ => unreachable!(),
        }
    }
}

fn parse_output(s: &str) -> (String, Vec<Direction>, Vec<String>) {
    let mut result = Vec::new();
    let mut result_items = Vec::new();
    let mut room_name = String::new();

    let mut is_in_doors = false;
    let mut is_in_items = false;

    let forbidden_items = [
        "giant electromagnet".to_string(),
        "photons".to_string(),
        "escape pod".to_string(),
        "molten lava".to_string(),
        "infinite loop".to_string(),
    ].to_vec();

    for line in s.lines() {
        // room name.
        if line.contains("==") {
            room_name = String::from(line);
            room_name = room_name.strip_prefix("== ").unwrap().to_string();
            room_name = room_name.strip_suffix(" ==").unwrap().to_string();
        }
    
        if line.contains("Doors here lead:") {
            is_in_doors = true;
        } else if is_in_doors && line.is_empty() {
            is_in_doors = false;
        } else if is_in_doors {
            let parts: Vec<&str> = line.split(' ').collect();
            result.push(parts[1].into());
        }

        if line.contains("Items here:") {
            is_in_items = true;
        } else if is_in_items && line.is_empty() {
            is_in_items = false;
        } else if is_in_items {
            let s = line.strip_prefix("- ").unwrap().to_string();

            if !forbidden_items.contains(&s) {
                result_items.push(s);
            }
        }
    }

    (room_name, result, result_items)
}

fn main() {
    let code = parse("input.txt");
    let mut machine = Machine::new(&code);
    let mut unknown_places: VecDeque<(String, Direction)> = VecDeque::new(); // Ex: Navigation, east
    let mut known_places: HashMap<String, HashMap<Direction, String>> = HashMap::new();
    let mut stored_items: Vec<String> = Vec::new();

    let mut last_room = String::new();
    let mut last_direction = Direction::North;

    loop {
        machine.run();
        let output = machine.get_output();
        machine.clean_output();

        let s: String = output.iter().map(|c| *c as u8 as char).collect();
        println!("{s}");

        let (mut room, doors, items) = parse_output(&s);

        if room.is_empty() && !last_room.is_empty() {
            room = last_room.clone();
        }

        // is this a known place?
        println!("My place is {:?}", room);
        println!("My items are: {:?}", stored_items);
        if !room.is_empty() && !known_places.contains_key(&room) {
            let mut hs: HashMap<Direction, String> = HashMap::new();
            
            // if we do not know the room, fill the unknown_places.
            for door in &doors {
                hs.insert(*door, String::new());
                unknown_places.push_back((room.to_owned(), *door));
            }

            if !last_room.is_empty() {
                hs.insert(last_direction.invert(), last_room.clone());
                known_places.get_mut(&last_room).unwrap().insert(last_direction, room.clone());
            }

            known_places.insert(room.to_owned(), hs);
        }

        // clean up unknown places
        while !unknown_places.is_empty() {
            let target = unknown_places[0].clone();

            if !known_places.get(&target.0).unwrap().get(&target.1).unwrap().is_empty() {
                unknown_places.pop_front();
            } else {
                break;
            }
        }

        if [("Security Checkpoint".to_string(), Direction::South)].to_vec() == unknown_places.clone().into_iter().collect::<Vec<(String, Direction)>>() {
            break;
        }

        // No more room to visit.
        if unknown_places.is_empty() {
            println!("Known places:");
            for known_place in &known_places {
                println!("{:?}", known_place);
            }
            println!("Unknown places: {unknown_places:?}");
            break;
        }

        last_room = room.clone();

        if !items.is_empty() {
            let mut s_take = String::from("take ");
            s_take.push_str(&items[0]);
            stored_items.push(items[0].to_owned());

            fill_instruction(&mut machine, &s_take);
            continue;
        }

        // find a place to go
        let target = unknown_places.iter().next().unwrap().clone();
        // println!("Known: {known_places:?}");
        // println!("Unknown: {unknown_places:?}");
        // println!("Target: {target:?}");

        let path = bfs(
            &room,
            |room| successors(&known_places, room),
            |room| room == &target.0,
        );

        let path = path.unwrap();
        last_direction = get_direction(&known_places, &path, &room, &target);

        fill_instruction(&mut machine, &last_direction.to_string());
    }

    // step 2: I've a lot of items, and I want to go south.
    for n in 0..stored_items.len() {
        for items4test in stored_items.iter().combinations(n) {
            machine_drop_all(&mut machine, &stored_items);

            for item in &items4test {
                machine_take(&mut machine, item);
            }

            let res = machine_go_south(&mut machine);
            // println!("{:?} = {}", items4test, res);

            if !res {
                return;
            }
        }
    }
}

/*
== Pressure-Sensitive Floor ==
Analyzing...

Doors here lead:
- north

A loud, robotic voice says "Analysis complete! You may proceed." and you enter the cockpit.
Santa notices your small droid, looks puzzled for a moment, realizes what has happened, and radios your ship directly.
"Oh, hello! You should be able to get in by typing 10504192 on the keypad at the main airlock."

["polygon", "bowl of rice", "candy cane", "hypercube", "dark matter", "manifold", "dehydrated water"] = true
 */

fn machine_drop(machine: &mut Machine, item: String) {
    let mut s = String::from("drop ");
    s.push_str(item.as_str());

    fill_instruction(machine, &s);
    machine.run();
}

fn machine_drop_all(machine: &mut Machine, items: &[String]) {
    items.iter().map(|item| machine_drop(machine, item.to_string())).count();
}

fn machine_take(machine: &mut Machine, item: &str) {
    let mut s = String::from("take ");
    s.push_str(item);

    fill_instruction(machine, &s);
    machine.run();
}

fn machine_go_south(machine: &mut Machine) -> bool {
    let s = String::from("south");
    fill_instruction(machine, &s);
    machine.clean_output();
    machine.run();
    let output= machine.get_output();
    let s: String = output.iter().map(|c| *c as u8 as char).collect();

    if !s.contains("you are ejected back to the checkpoint.") {
        println!("{s}");
        false
    } else {
        true
    }
}

fn fill_instruction(machine: &mut Machine, s: &str) {
    // println!("Sending '{}'", s);
    for c in s.chars() {
        machine.add_input(c as u8 as isize);
    }
    machine.add_input(10);
}

fn get_direction(
    known_places: &HashMap<String, HashMap<Direction, String>>,
    path: &[String],
    current: &String,
    target: &(String, Direction)
) -> Direction {
    if current == &target.0 {
        target.1
    } else {
        // need to find the direction between current & path[1]
        *known_places.get(current).unwrap().iter().find(|(_,  n)| **n == path[1]).unwrap().0
    }
}

fn successors(known_places: &HashMap<String, HashMap<Direction, String>>, room: &String) -> Vec<String> {
    known_places.get(room).unwrap().iter().map(|(_, n)| n.to_owned()).filter(|x| !x.is_empty()).collect()
}
