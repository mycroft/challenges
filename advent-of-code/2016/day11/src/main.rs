use std::collections::BTreeSet;

use pathfinding::prelude::astar;

#[derive(PartialEq, Eq, Debug, PartialOrd, Ord, Clone, Hash)]
enum Device {
    Generator(String),
    Chip(String)
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct State {
    elevator_floor: usize,
    floors: Vec<BTreeSet<Device>>
}

fn combine(_set: &BTreeSet<Device>, num: usize) -> Vec<BTreeSet<Device>> {
    let mut sets : Vec<BTreeSet<Device>> = vec![];

    if num == 1 {
        for el in _set {
            let mut tree_set = BTreeSet::new();
            tree_set.insert(el.clone());
            sets.push(tree_set);
        }
    }


    if num == 2 {
        for el in _set {
            for el2 in _set {
                if el == el2 {
                    continue;
                }

                let mut tree_set = BTreeSet::new();
                tree_set.insert(el.clone());
                tree_set.insert(el2.clone());
                sets.push(tree_set);
            }
        }
    }

    sets
}

fn is_safe(_set: &BTreeSet<Device>) -> bool {
    let mut gens = BTreeSet::new();
    let mut chips = BTreeSet::new();
    for c in _set.into_iter().cloned() {
        match c {
            Device::Generator(name) => gens.insert(name),
            Device::Chip(name) => chips.insert(name)
        };
    };
    gens.is_empty() || chips.is_subset(&gens)

}

impl State {
    fn new(floors: Vec<BTreeSet<Device>>) -> Self {
        State {
            elevator_floor: 0,
            floors: floors
        }
    }

    fn is_success(&self) -> bool {
        for idx in 0..self.floors.len()-1 {
            if self.floors[idx].len() > 0 {
                return false
            }
        }

        true
    }

    fn successors(&self) -> Vec<(Self, u32)> {
        let mut successors = vec![];

        let mut min_floor = 0;

        loop {
            if self.floors[min_floor].len() != 0 {
                break;
            }

            min_floor += 1;
        }

        if self.elevator_floor != self.floors.len() - 1 {
            // can go upstair
            let mut went_up = false;

            let combi = combine(&self.floors[self.elevator_floor], 2);
            for subcombi in combi {
                let mut successor = self.clone();
                successor.floors[self.elevator_floor] = successor.floors[self.elevator_floor].difference(&subcombi).cloned().collect();
                successor.floors[self.elevator_floor + 1] = successor.floors[self.elevator_floor + 1].union(&subcombi).cloned().collect();

                successor.elevator_floor += 1;

                if is_safe(&successor.floors[self.elevator_floor]) && is_safe(&successor.floors[successor.elevator_floor]) {
                    successors.push(
                        (successor.clone(), 1)
                    );     
                    went_up = true;               
                }
            }

            if !went_up {
                let combi = combine(&self.floors[self.elevator_floor], 1);
                for subcombi in combi {
                    let mut successor = self.clone();
                    successor.floors[self.elevator_floor] = successor.floors[self.elevator_floor].difference(&subcombi).cloned().collect();
                    successor.floors[self.elevator_floor + 1] = successor.floors[self.elevator_floor + 1].union(&subcombi).cloned().collect();

                    successor.elevator_floor += 1;

                    if is_safe(&successor.floors[self.elevator_floor]) && is_safe(&successor.floors[successor.elevator_floor]) {
                        successors.push(
                            (successor.clone(), 1)
                        );     
                    }
                }
            }
        }

        if self.elevator_floor > min_floor {
            let mut went_down = false;

            let combi = combine(&self.floors[self.elevator_floor], 1);
            for subcombi in combi {
                let mut successor = self.clone();
                successor.floors[self.elevator_floor] = successor.floors[self.elevator_floor].difference(&subcombi).cloned().collect();
                successor.floors[self.elevator_floor - 1] = successor.floors[self.elevator_floor - 1].union(&subcombi).cloned().collect();

                successor.elevator_floor -= 1;
                if is_safe(&successor.floors[self.elevator_floor]) && is_safe(&successor.floors[successor.elevator_floor]) {
                    successors.push(
                        (successor.clone(), 1)
                    );
                    went_down = true;
                }
            }

            if !went_down {
                let combi = combine(&self.floors[self.elevator_floor], 2);
                for subcombi in combi {
                    let mut successor = self.clone();
                    successor.floors[self.elevator_floor] = successor.floors[self.elevator_floor].difference(&subcombi).cloned().collect();
                    successor.floors[self.elevator_floor - 1] = successor.floors[self.elevator_floor - 1].union(&subcombi).cloned().collect();

                    successor.elevator_floor -= 1;
                    if is_safe(&successor.floors[self.elevator_floor]) && is_safe(&successor.floors[successor.elevator_floor]) {
                        successors.push(
                            (successor.clone(), 1)
                        );
                    }
                }
            }
        }

        successors
    }
}

fn main() {
    let strontium = String::from("strontium");
    let plutonium = String::from("plutonium");
    let thulium = String::from("thulium");
    let ruthenium = String::from("ruthenium");
    let curium = String::from("curium");

    let c_strontium = Device::Chip(strontium.clone());
    let g_strontium = Device::Generator(strontium.clone());

    let c_plutonium = Device::Chip(plutonium.clone());
    let g_plutonium = Device::Generator(plutonium.clone());

    let c_thulium = Device::Chip(thulium.clone());
    let g_thulium = Device::Generator(thulium.clone());

    let c_ruthenium = Device::Chip(ruthenium.clone());
    let g_ruthenium = Device::Generator(ruthenium.clone());

    let c_curium = Device::Chip(curium.clone());
    let g_curium = Device::Generator(curium.clone());

    let mut floors : Vec<BTreeSet<Device>> = vec![];
    floors.push(BTreeSet::new());
    floors.push(BTreeSet::new());
    floors.push(BTreeSet::new());
    floors.push(BTreeSet::new());

    floors[0].insert(c_strontium);
    floors[0].insert(g_strontium);
    floors[0].insert(c_plutonium);
    floors[0].insert(g_plutonium);

    floors[1].insert(g_thulium);
    floors[1].insert(g_ruthenium);
    floors[1].insert(c_ruthenium);
    floors[1].insert(c_curium);
    floors[1].insert(g_curium);

    floors[2].insert(c_thulium);

    let mut _init_state = State::new(floors);

    let result = astar(
        &_init_state,
        |p| p.successors(),
        |_p| 0,
        |p| p.is_success()
    );

    println!("Part #1: {:?}", result.unwrap().1);

/*
    let elerium = String::from("elerium");
    let dilithium = String::from("dilithium");

    let c_elerium = Device::Chip(elerium.clone());
    let g_elerium = Device::Generator(elerium.clone());

    let c_dilithium = Device::Chip(dilithium.clone());
    let g_dilithium = Device::Generator(dilithium.clone());

    _init_state.floors[0].insert(c_elerium);
    _init_state.floors[0].insert(g_elerium);

    _init_state.floors[0].insert(c_dilithium);
    _init_state.floors[0].insert(g_dilithium);

    let result = astar(
        &_init_state,
        |p| p.successors(),
        |_p| 0,
        |p| p.is_success()
    );

    println!("Part #2: {:?}", result.unwrap().1);
*/
}

#[test]
fn test() {
    let hydrogen = String::from("hydrogen");
    let lithium = String::from("lithium");

    let chip_hydrogen = Device::Chip(hydrogen.clone());
    let chip_lithium = Device::Chip(lithium.clone());

    assert_ne!(chip_lithium, chip_hydrogen);
}

#[test]
fn example() {
    let hydrogen = String::from("hydrogen");
    let lithium = String::from("lithium");

    let chip_hydrogen = Device::Chip(hydrogen.clone());
    let chip_lithium = Device::Chip(lithium.clone());

    let generator_hydrogen = Device::Generator(hydrogen.clone());
    let generator_lithium = Device::Generator(lithium.clone());

    let mut floors : Vec<BTreeSet<Device>> = vec![];
    floors.push(BTreeSet::new());
    floors.push(BTreeSet::new());
    floors.push(BTreeSet::new());
    floors.push(BTreeSet::new());

    floors[0].insert(chip_hydrogen);
    floors[0].insert(chip_lithium);

    floors[1].insert(generator_hydrogen);
    floors[2].insert(generator_lithium);

    let _init_state = State::new(floors);

    let result = astar(
        &_init_state,
        |p| p.successors(),
        |_p| 0,
        |p| p.is_success()
    );

    assert_eq!(11, result.unwrap().1);
}