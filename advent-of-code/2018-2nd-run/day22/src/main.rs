/* AOC 2018 - 22 */
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RegionType {
    Rocky, // ClimbingGear | Torch
    Wet, // ClimbingGear | Neither
    Narrow, // Torch | Neither
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Equipment {
    Torch,
    ClimbingGear,
    Neither,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Pos(usize, usize);

#[derive(Debug, Hash, PartialEq, Eq)]
struct Context {
    eq: Equipment,
    pos: Pos,
}

fn index_type(erosion_level: u64) -> RegionType {
    match erosion_level % 3{
        0 => RegionType::Rocky,
        1 => RegionType::Wet,
        2 => RegionType::Narrow,
        _ => unreachable!()
    }
}

fn allowed(region: RegionType, eq: Equipment) -> bool {
    match region {
        RegionType::Rocky => eq == Equipment::ClimbingGear || eq == Equipment::Torch,
        RegionType::Wet => eq == Equipment::ClimbingGear || eq == Equipment::Neither,
        RegionType::Narrow => eq == Equipment::Torch || eq == Equipment::Neither
    }
}

fn common_tool(region1: RegionType, region2: RegionType) -> Equipment {
    match region1 {
        RegionType::Rocky => match region2 {
            RegionType::Wet => Equipment::ClimbingGear,
            RegionType::Narrow => Equipment::Torch,
            _ => unreachable!()
        },
        RegionType::Wet => match region2 {
            RegionType::Rocky => Equipment::ClimbingGear,
            RegionType::Narrow => Equipment::Neither,
            _ => unreachable!()
        },
        RegionType::Narrow => match region2 {
            RegionType::Rocky => Equipment::Torch,
            RegionType::Wet => Equipment::Neither,
            _ => unreachable!()
        },
    }
}

fn erosion_level(cache: &mut HashMap<Pos, u64>, depth: usize, coord: Pos, target: Pos) -> u64 {
    if cache.contains_key(&coord) {
        return *cache.get(&coord).unwrap();
    }

    let mut level = match coord {
        Pos(0, 0) => 0,
        Pos(x, 0) => x as u64 * 16807,
        Pos(0, y) => y as u64 * 48271,
        x => {
            if x == target {
                0 as u64
            } else {
                let coord_a = Pos(coord.0-1, coord.1);
                let a = if cache.contains_key(&coord_a) {
                    *cache.get(&coord_a).unwrap()
                } else {
                    erosion_level(cache, depth, coord_a, target)
                };

                let coord_b = Pos(coord.0, coord.1-1);
                let b = if cache.contains_key(&coord_b) {
                    *cache.get(&coord_b).unwrap()
                } else {
                    erosion_level(cache, depth, coord_b, target)
                };

                a * b
            }
        }
    };

    level = (level + depth as u64) % 20183;

    cache.insert(coord, level);

    level
}

fn main() {
    let depth = 6084; // test: 510, ex: 6084
    let target = Pos(14, 709); // test: 10,10, ex: 14, 709

    // erosion levels cache.
    let mut cache : HashMap<Pos, u64> = HashMap::new();
    let mut total = 0;

    for y in 0..=target.1 {
        let mut line = String::from("");
        for x in 0..=target.0 {
            let c = erosion_level(&mut cache, depth, Pos(x, y), target) % 3;

            line.push(match c {
                0 => '.',
                1 => '=',
                2 => '|',
                _ => unreachable!()
            });
            total += c
        }
    }

    println!("#1: {:?}", total);

    let mut scan_pos : Vec<Context> = vec![Context{ eq: Equipment::Torch, pos: Pos(0, 0) }];
    let mut hm_times : HashMap<Context, usize> = HashMap::new();
    let mut found_duration : Option<usize> = Some(961);

    let deltas = [(0, 1), (1, 0), (-1, 0), (0, -1)];

    hm_times.insert(Context { pos: Pos(0, 0), eq: Equipment::Torch }, 0);

    loop {
        if scan_pos.len() == 0 {
            break;
        }

        let current_context = scan_pos.pop().unwrap();
        let current_pos = current_context.pos;
        let current_eq = current_context.eq;

        let current_duration = *hm_times.get(&current_context).unwrap();

        if found_duration != None && current_duration > found_duration.unwrap() {
            // Too slow: skip.
            continue;
        }

        // for a given position, check how much time it would take to go to each adjacent.
        // if target duration is changed (because not known or lesser than before), then push the item to
        // run scanning again.

        for delta in deltas {
            if current_pos.0 as i32 + delta.0 < 0 
            || current_pos.1 as i32 + delta.1 < 0 {
                continue;
            }

            let target_pos = Pos(
                (current_pos.0 as i32 + delta.0) as usize,
                (current_pos.1 as i32 + delta.1) as usize
            );

            let current_region_type = index_type(
                erosion_level(&mut cache, depth, current_pos, target)
            );

            let target_region_type = index_type(
                erosion_level(&mut cache, depth, target_pos, target)
            );

            let new_eq = if allowed(target_region_type, current_eq) {
                current_eq
            } else {
                common_tool(current_region_type, target_region_type)
            };

            // Grab current duration
            let new_duration = current_duration + 1 + if new_eq != current_eq {
                7 // change
            } else {
                0 // no change
            };

            let target_context = Context { pos: target_pos, eq: new_eq };

            let mut duration_changed = false;

            match hm_times.get(&target_context) {
                None => {
                    hm_times.insert(target_context, new_duration);
                    // Push in stack
                    scan_pos.insert(0, Context {eq: new_eq, pos: target_pos});
                    duration_changed = true;
                },
                Some(&n) => {
                    if n > new_duration {
                        hm_times.insert(target_context, new_duration);
                        // Push in stack          
                        scan_pos.insert(0, Context {eq: new_eq, pos: target_pos});

                        duration_changed = true;
                    } else {
                        // Do nothing.
                    }
                }
            };

            if duration_changed && target_pos == target {
                let new_value = new_duration + if new_eq != Equipment::Torch { 7 } else { 0 };
                if let Some(old_value) = found_duration {
                    if old_value > new_value {
                        found_duration = Some(new_value);
                    }
                } else {
                    found_duration = Some(new_value);
                }
            }
/*
            if duration_changed {
                println!("From {:?} ({}) (type: {:?}), target is {:?} (type: {:?}) eq tool: {:?} -> new tool: {:?} / Duration: {} (current last found: {:?})",
                    current_pos,
                    current_duration,
                    current_region_type,
                    target_pos,
                    target_region_type,
                    current_eq,
                    new_eq,
                    new_duration,
                    found_duration,
                );
            }
*/
        }
    }

    println!("#2: {}", found_duration.unwrap());
}

