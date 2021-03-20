/*
 * Advent of code 2018, day 15
 */
use std::cmp::Ordering;
use std::fs;

extern crate anyhow;
extern crate pathfinding;

use anyhow::Result;
use pathfinding::prelude::bfs;

#[derive(Clone,Copy,Debug,Eq,Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y < other.y || (self.y == other.y && self.x < other.x) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone,Copy,Debug,Eq,PartialEq,Hash)]
enum PersoType {
    Elf,
    Grunt,
}

#[derive(Clone,Copy,Debug,PartialEq,Eq,Hash)]
struct Perso {
    pos: Pos,
    perso_type: PersoType,
    hits: usize,
    alive: bool,
}

impl Ord for Perso {
    fn cmp(&self, other: &Self) -> Ordering {
        self.pos.cmp(&other.pos)
    }
}

impl PartialOrd for Perso {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Perso {
    fn successors(&self, grid: &Vec<Vec<bool>>, persos: &Vec<Perso>) -> Vec<Perso> {
        let mut successors = vec![];

        for it in [(0,-1),(-1,0),(1,0),(0,1)].iter() {
            let new_pos = Pos{
                x: self.pos.x + it.0,
                y: self.pos.y + it.1,
            };

            // if can't go there, pass.
            if grid[new_pos.y as usize][new_pos.x as usize] == false {
                continue;
            }

            // if any other character in next, pass.
            if persos.iter().any(|x| x.alive && x.pos == new_pos) {
                continue;
            }

            // valid move!
            successors.push(Perso{
                pos: new_pos,
                perso_type: self.perso_type,
                hits: self.hits,
                alive: true,
            })
        }

        successors
    }

    fn goal(&self, persos: &Vec<Perso>) -> bool {
        self.is_in_range(persos)
    }

    fn is_in_range(&self, persos: &Vec<Perso>) -> bool {
        for it in [(0,-1),(-1,0),(1,0),(0,1)].iter() {
            let new_pos = Pos{
                x: self.pos.x + it.0,
                y: self.pos.y + it.1,
            };

            if persos.iter().any(|other| other.alive && other.pos == new_pos && self.perso_type != other.perso_type) {
                return true;
            }
        }

        false
    }

    fn get_target(&self, persos: &Vec<Perso>) -> Option<Perso> {
        let mut min_points = 200;
        let mut target = None;

        for it in [(0,-1),(-1,0),(1,0),(0,1)].iter() {
            let new_pos = Pos{
                x: self.pos.x + it.0,
                y: self.pos.y + it.1,
            };

            for other in persos.iter() {
                if other.alive && other.pos == new_pos && self.perso_type != other.perso_type {
                    if target == None {
                        target = Some(*other);
                        min_points = other.hits;
                    } else if other.hits < min_points {
                        target = Some(*other);
                        min_points = other.hits;
                    }
                }
            }
        }

        target
    }
}

fn step(grid: &Vec<Vec<bool>>, persos: &mut Vec<Perso>, drawdebug: bool, elf_power: usize) -> (Vec<Perso>, bool) {
    // first, sort perso by order of reading
    persos.sort();

    // for each perso, check if we want to move, or move to destination, & attack
    for i in 0..persos.len() {
        // draw(&grid, &persos);
        // println!("Now doing {:?}", persos[i]);

        if !persos[i].alive {
            continue;
        }

        // before doing anything, check if we can have a target.
        let mut has_grunt = false;
        let mut has_elf = false;

        for perso in persos.iter() {
            if perso.alive && perso.perso_type == PersoType::Grunt {
                has_grunt = true;
            }

            if perso.alive && perso.perso_type == PersoType::Elf {
                has_elf = true;
            }
        }

        if !has_grunt || !has_elf {
            return (persos.to_vec(), true);
        }

        // check if in range
        let mut in_range = persos[i].is_in_range(&persos);

        if !in_range {
            // move
            let results = bfs(
                &persos[i],
                |p| p.successors(&grid, &persos),
                |p| p.goal(&persos)
            );

            if results != None {
                persos[i].pos = results.unwrap()[1].pos;
                in_range = persos[i].is_in_range(&persos);
            }
        }

        if !in_range {
            continue
        }

        // if in range, attack
        // println!("{:?} is in range!", persos[i]);

        let target = persos[i].get_target(&persos).unwrap();

        let mut attack_point = 3;

        if persos[i].perso_type == PersoType::Elf {
            attack_point = elf_power;
        }

        // println!("Target is {:?}", target);
        for perso in persos.iter_mut() {
            if perso.pos == target.pos {
                if perso.hits <= attack_point {
                    // This one dies.
                    perso.alive = false;
                    perso.hits = 0;

                    if perso.perso_type == PersoType::Elf && elf_power != 3 {
                        // game stopped: not what we wanted.

                        return (persos.to_vec(), true)
                    }

                } else {
                    perso.hits -= attack_point;
                }
            }
        }
    }

    if drawdebug {
        draw(&grid, &persos);
    }

    (persos.to_vec(), false)
}

fn draw(grid: &Vec<Vec<bool>>, persos: &Vec<Perso>) {
    for (lineno, line) in grid.iter().enumerate() {
        let mut s = String::from("");
        for c in line {
            if *c {
                if persos.iter().any(|x| x.alive && x.pos == Pos{x: s.len() as isize, y: lineno as isize }) {
                    s.push('X');
                } else {
                    s.push('.');
                }
                
            } else {
                s.push('#');
            }
        }
        println!("{}", s);
    }
}

fn main() -> Result<()> {
    let contents = fs::read_to_string("input.txt")?;
    let mut grid : Vec<Vec<bool>> = vec![];
    let mut persos : Vec<Perso> = vec![];

    for line in contents.lines() {
        let mut grid_line : Vec<bool> = vec![];
        for c in line.chars() {
            if c == '#' {
                grid_line.push(false);
            } else {
                grid_line.push(true);
            }

            match c {
                'E' | 'G' => {
                    let t = if c == 'E' {
                        PersoType::Elf
                    } else {
                        PersoType::Grunt
                    };

                    let pos = Pos {
                        x: grid_line.len() as isize - 1,
                        y: grid.len() as isize,
                    };

                    persos.push(Perso{
                        pos: pos,
                        perso_type: t,
                        hits: 200,
                        alive: true,
                    });
                }
                _ => {},
            }
        }

        grid.push(grid_line);
    }

    let orig_persos = persos.clone();

    let mut round = 0;

    loop {
        let ret = step(&grid, &mut persos, false, 3);
        persos = ret.0;

        let finished = ret.1;
        if finished {
            // println!("Game finished!");
            break;
        }

        round += 1;
        // println!("We are continuing after {:?} full round(s).", round);
    }

    let mut remaining_hits = 0;

    for perso in &persos {
        // println!("{:?}", perso);
        remaining_hits += perso.hits;
    }

    println!("Finished after {} rounds, remaining hits: {}", round, remaining_hits);
    println!("Part #1: {}", round * remaining_hits);

    let mut current_attack = 4;

    'endgame: loop {
        // println!("Trying with attack power: {}", current_attack);

        persos = orig_persos.clone();
        round = 0;

        loop {
            let ret = step(&grid, &mut persos, false, current_attack);
            let finished = ret.1;
            if finished {
                // println!("Game finished!");
                break;
            }

            round += 1;
        }

        // check if all our elves are alive.
        let mut hope = true;
        for perso in &persos {
            if perso.perso_type == PersoType::Elf && !perso.alive {
                hope = false;
            }
        }

        if hope {
            break 'endgame;
        }

        current_attack += 1;
    }

    let mut remaining_hits = 0;

    for perso in &persos {
        // println!("{:?}", perso);
        remaining_hits += perso.hits;
    }

    println!("Finished after {} rounds, remaining hits: {}", round, remaining_hits);
    println!("Part #2: {}", round * remaining_hits);


    Ok(())
}
