use std::fs::read_to_string;
use std::collections::{VecDeque, HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Proposal {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Elf {
    number: usize, // debugging purpose
    pos: Pos,
    order: VecDeque<Proposal>,
}

fn parse(fp: &str) -> Vec<Elf> {
    let mut result = Vec::new();

    let contents = read_to_string(fp).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut y = 0;
    let mut elf_idx = 0;

    let order : VecDeque<Proposal> = [
        Proposal::North,
        Proposal::South,
        Proposal::West,
        Proposal::East
    ].into();

    for line in lines {
        for c in line.chars().enumerate() {
            if c.1 == '#' {
                result.push(Elf{
                    number: elf_idx,
                    pos: Pos {
                        x: c.0 as isize,
                        y: y,
                    },
                    order: order.clone(),
                });
                elf_idx += 1;
            }
        }
        y += 1;
    }

    result
}

fn check_alone(p: &Pos, elves_map: &mut HashSet<Pos>) -> bool {
    let dirs = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1), (0, 1),
        (1, -1), (1, 0), (1, 1),
    ];

    for dir in dirs {
        let npos = Pos{x: p.x + dir.0, y: p.y + dir.1};
        if elves_map.contains(&npos) {
            return false;
        }
    }

    true
}
fn check_possible(p: &Pos, proposal: Proposal, elves_map: &mut HashSet<Pos>) -> bool {
    let dirs = match proposal {
        Proposal::North => [(-1, -1), (0, -1), (1 ,-1)],
        Proposal::South => [(-1, 1), (0, 1), (1, 1)],
        Proposal::West => [(-1, -1), (-1, 0), (-1 ,1)],
        Proposal::East => [(1, -1), (1, 0), (1, 1)]
    };

    for dir in dirs {
        let npos = Pos{x: p.x + dir.0, y: p.y + dir.1};
        if elves_map.contains(&npos) {
            return false;
        }
    }
    true
}

fn find_proposal_for_elf(elf: &Elf, elves_map: &mut HashSet<Pos>) -> Option<(Proposal, Pos)> {
    if check_alone(&elf.pos, elves_map) {
        return None
    }

    for prop_idx in 0..4 {
        let proposal = elf.order[prop_idx];

        if check_possible(&elf.pos, proposal, elves_map) {
            // compute position
            let npos = match proposal {
                Proposal::North => Pos{x: elf.pos.x, y: elf.pos.y - 1},
                Proposal::South => Pos{x: elf.pos.x, y: elf.pos.y + 1},
                Proposal::West => Pos {x: elf.pos.x - 1, y: elf.pos.y},
                Proposal::East => Pos {x: elf.pos.x + 1, y: elf.pos.y},
            };

            // println!("find_proposal for {elf:?} -> {proposal:?}");

            return Some((proposal, npos));
        }
    }

    // println!("find_proposal for {elf:?} -> X");

    None
}

fn round(elves: &mut Vec<Elf>, elves_map: &mut HashSet<Pos>, round: usize) -> usize {
    let mut propositions: HashMap<usize, (Proposal, Pos)> = HashMap::new();
    for elf_idx in 0..elves.len() {
        if let Some(proposal) = find_proposal_for_elf(&elves[elf_idx], elves_map) {
            propositions.insert(
                elf_idx,
                proposal,
            );
        }
    }

    // println!("round:{round} propositions: {}", propositions.len());

    if propositions.len() == 0 {
        return round;
    }

    // for each elf, check if it has a proposition, and if destination is unique, and if so, move it and re-order proposals
    for e in elves.iter_mut().enumerate() {
        // does it have a move proposition
        if !propositions.contains_key(&e.0) {
            let considered_direction = e.1.order.pop_front().unwrap();
            e.1.order.push_back(considered_direction);
    
            continue;
        }

        let (_, dest) = propositions.get(&e.0).unwrap();

        // is destination unique?
        let count = propositions.iter().filter(|&p| dest == &p.1.1).count();

        if count == 1 {
            // move elf
            elves_map.remove(&e.1.pos);
            e.1.pos = *dest;
            elves_map.insert(*dest);
        }

        // reorder propositions: prop must be at the end of the e.1.order queue
        // let order_idx = e.1.order.iter().position(|x| *x == *prop).unwrap();
        // e.1.order.remove(order_idx);
        // e.1.order.push_back(*prop);

        let considered_direction = e.1.order.pop_front().unwrap();
        e.1.order.push_back(considered_direction);
    }

    0
}

fn step0(elves: &mut Vec<Elf>, elves_map: &mut HashSet<Pos>, r: usize) -> isize {
    for r in 0..r {
        let r = round(elves, elves_map, r);

        if r != 0 {
            return 1 + r as isize;
        }
        // display(elves);
    }

    // find min_x, min_y, max_X, max_y
    let mut min_x = elves[0].pos.x;
    let mut max_x = min_x;

    let mut min_y = elves[0].pos.y;
    let mut max_y = min_y;

    let elves_len = elves.len() as isize;

    for e in elves {
        if e.pos.x < min_x { min_x = e.pos.x; }
        if e.pos.x > max_x { max_x = e.pos.x; }
        if e.pos.y < min_y { min_y = e.pos.y; }
        if e.pos.y > max_y { max_y = e.pos.y; }
    }

    (max_x - min_x + 1) * (max_y - min_y + 1) - elves_len
}

fn display(elves: &Vec<Elf>) {
    // find min_x, min_y, max_X, max_y
    let mut min_x = elves[0].pos.x;
    let mut max_x = min_x;

    let mut min_y = elves[0].pos.y;
    let mut max_y = min_y;

    let margin = 1;

    for e in elves {
        if e.pos.x < min_x { min_x = e.pos.x; }
        if e.pos.x > max_x { max_x = e.pos.x; }
        if e.pos.y < min_y { min_y = e.pos.y; }
        if e.pos.y > max_y { max_y = e.pos.y; }
    }

    for y in min_y-margin..=max_y+margin {
        for x in min_x-margin..=max_x+margin {
            if elves.iter().filter(|&elf| elf.pos == Pos{x, y}).count() == 1 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    println!();
}

enum Mode {
    Infinite,
    Round(usize)
}

fn play(fp: &str, mode: Mode) -> isize {
    let mut elves = parse(fp);
    let mut elves_map: HashSet<Pos> = HashSet::new();

    for e in &elves {
        elves_map.insert(e.pos);
    }

    let round_num = match mode {
        Mode::Infinite => 999999,
        Mode::Round(x) => x,
    };

    step0(&mut elves, &mut elves_map, round_num)
}

fn main() {
    println!("#1 {}", play("input.txt", Mode::Round(10)));
    println!("#2 {}", play("input.txt", Mode::Infinite));
}

#[test]
fn test_sample() {
    assert_eq!(
        110,
        play("input.txt_test1", Mode::Round(10))
    );

    assert_eq!(
        20,
        play("input.txt_test1", Mode::Infinite)
    );
}