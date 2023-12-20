use std::fs;
use std::collections::VecDeque;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ModuleType {
    None,
    Conjunction,
    FlipFlop,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SignalType {
    Low,
    High,
    None,
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    destination: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
enum FlipFlopState {
    On,
    Off,
}

fn read_rules(filename: &str) -> HashMap<String, Module> {
    let mut res = HashMap::new();

    let contents = fs::read_to_string(filename).expect("a file to expect");
    let lines = contents.lines();

    res.insert("button".to_string(), Module {
        module_type: ModuleType::None,
        destination: ["broadcaster".to_string()].to_vec()
    });

    for line in lines {
        let parts = line.split(" -> ").collect::<Vec<&str>>();

        let (module_type, module_name, ) = match parts[0].chars().next().unwrap() {
            '&' => (ModuleType::Conjunction, parts[0][1..].to_string()),
            '%' => (ModuleType::FlipFlop, parts[0][1..].to_string()),
            _ => (ModuleType::None, parts[0].to_string()),
        };

        res.insert(module_name, Module{
            module_type,
            destination: parts[1].split(", ").map(String::from).collect::<Vec<String>>(),
        });
    }

    res
}

fn solve(rules: &HashMap<String, Module>, mut remaining_init_signals: isize, targets: &[&str]) -> (u128, u128) {
    let mut conjunction_states : HashMap<String, HashMap<String, SignalType>> = HashMap::new();
    let mut flipflop_states: HashMap<String, FlipFlopState> = HashMap::new();

    let mut signals: VecDeque<(String, SignalType, String)> = VecDeque::new();

    let mut count_low: u128 = 0;
    let mut count_high: u128 = 0;

    let mut clicks = 0;

    let mut printables: Vec<&str> = targets.into();
    let mut part2: u128 = 1;

    // Initialize states
    for (module_name, module) in rules {
        match module.module_type {
            ModuleType::Conjunction => {
                let mut inputs = HashMap::new();
                for (module_from, module) in rules {
                    if module.destination.contains(module_name) {
                        inputs.insert(module_from.to_string(), SignalType::Low);
                    }
                }
                conjunction_states.insert(module_name.to_string(), inputs);
            },
            ModuleType::FlipFlop => {
                flipflop_states.insert(module_name.to_string(), FlipFlopState::Off);
            },
            _ => {},
        }
    }

    loop {
        if signals.is_empty() {
            if remaining_init_signals > 0 {
                clicks += 1;
                // println!();
                // println!("{:?}", flipflop_states);
                // println!("{:?}", conjunction_states);
                signals.push_back(("broadcaster".to_string(), SignalType::Low, "button".to_string()));
                remaining_init_signals -= 1;
            } else {
                break;
            }
        }

        let (module_name, signal_type, module_from) = signals.pop_front().unwrap();

        // println!("{}: {} -> {:?} -> {}", clicks, module_from, signal_type, module_name);

        if printables.contains(&module_name.as_str()) && signal_type == SignalType::Low {
            println!("{} {}", clicks, module_name);

            printables.remove(
                printables.iter().position(|&x| x == module_name.as_str()).unwrap()
            );

            part2 *= clicks
        }

        if clicks <= 1000 {
            match signal_type {
                SignalType::Low => { count_low += 1},
                SignalType::High => { count_high += 1},
                _ => {},
            };
        }

        if !rules.contains_key(&module_name) {
            continue;
        }

        let module = rules.get(&module_name).unwrap();

        let new_signal: SignalType = match module.module_type {
            ModuleType::Conjunction => {
                // first, update the state
                let entries = conjunction_states.get_mut(&module_name).unwrap();
                entries.insert(module_from, signal_type);

                let mut next_signal = SignalType::Low;
                for signal_type in entries.values() {
                    if signal_type == &SignalType::Low {
                        next_signal = SignalType::High;
                    }
                }

                next_signal
            },
            ModuleType::FlipFlop => {
                match signal_type {
                    SignalType::High => SignalType::None,
                    SignalType::Low => {
                        let flipflop_state = *flipflop_states.get(&module_name).unwrap();
                        match flipflop_state {
                            FlipFlopState::On => {
                                flipflop_states.insert(module_name.to_string(), FlipFlopState::Off);
                                SignalType::Low
                            },
                            FlipFlopState::Off => {
                                flipflop_states.insert(module_name.to_string(), FlipFlopState::On);
                                SignalType::High
                            }
                        }                        
                    },
                    _ => unreachable!(),
                }
            },
            ModuleType::None => {
                signal_type
            },
        };

        if new_signal != SignalType::None {
            for destination in &module.destination {
                signals.push_back((destination.to_string(), new_signal, module_name.to_string()));
            }
        }
    }

    // println!("low={} high={}", count_low, count_high);

    (count_high * count_low, part2)
}

fn main() {
    let rules = read_rules("input.txt");

    // for (k, v) in &rules {
    //     println!("{} -> {:?}", k, v);
    // }

    let res = solve(&rules, 10000, &["xl", "ln", "xp", "gp"]);

    println!("#1 {}", res.0); // 666795063
    println!("#2 {}", res.1); // 253302889093151

    // for 2nd part, read input and find out that low -> rx is requiring high -> to xl, ln, xp and gp
    // this happens on xl = 4051, ln = 4021, xp = 4057, gp = 3833
}
