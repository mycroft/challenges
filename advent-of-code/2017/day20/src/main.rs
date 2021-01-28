use std::ops::Add;
use std::fs;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Particule {
    p: Pos,
    v: Pos,
    a: Pos,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/*
Increase the X velocity by the X acceleration.
Increase the Y velocity by the Y acceleration.
Increase the Z velocity by the Z acceleration.
Increase the X position by the X velocity.
Increase the Y position by the Y velocity.
Increase the Z position by the Z velocity.
*/
fn step(particules: &mut Vec<Particule>) {
    let particules_len = particules.len();

    for id in 0..particules_len {
        let particule = &particules[id];

        let new_v = particule.v + particule.a;
        let new_p = particule.p + new_v;

        particules[id] = Particule{p: new_p, v: new_v, a: particule.a};

//        println!("{}: {:?} distance:{}",
//            id,
//            particules[id], 
//            (particules[id].p.x.abs() + particules[id].p.y.abs() + particules[id].p.z.abs()) / 3
//        );
    }

//    println!("");
}

fn clean_collide(particules: &mut Vec<Particule>) {
    let mut h = HashMap::new();
    let particules_len = particules.len();

    for id in 0..particules_len {
        h.entry(particules[id].p.clone()).or_insert(vec![]).push(id);
    }

    let mut to_remove = vec![];

    for (_k, mut v) in h {
        if v.len() >= 2 {
            loop {
                if v.len() == 0 {
                    break;
                }

                let id = v.pop().unwrap();

                to_remove.push(id);
            }
        }
    }

    to_remove.sort();

    loop {
        if to_remove.len() == 0 {
            break;
        }

        let id = to_remove.pop().unwrap();

        particules.remove(id);
    }
}

fn check(particules: &Vec<Particule>) -> usize {
    let mut nearest = None;
    let mut nearest_id = None;

    for (id, particule) in particules.iter().enumerate() {
        let c = (particule.p.x.abs() + particule.p.y.abs() + particule.p.z.abs()) / 3;

        if None == nearest {
            nearest = Some(c);
            nearest_id = Some(id);
        }

        if nearest > Some(c) {
            nearest = Some(c);
            nearest_id = Some(id);
        }
    }

    nearest_id.unwrap()
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();

    let re = Regex::new(r"^p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>$").unwrap();

    let mut particules : Vec<Particule> = vec![];

    for line in lines {
        let caps = re.captures(line).unwrap();

        particules.push(
            Particule {
                p: Pos {
                    x: caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    y: caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                    z: caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                },
                v: Pos {
                    x: caps.get(4).unwrap().as_str().parse::<i32>().unwrap(),
                    y: caps.get(5).unwrap().as_str().parse::<i32>().unwrap(),
                    z: caps.get(6).unwrap().as_str().parse::<i32>().unwrap(),                    
                },
                a: Pos {
                    x: caps.get(7).unwrap().as_str().parse::<i32>().unwrap(),
                    y: caps.get(8).unwrap().as_str().parse::<i32>().unwrap(),
                    z: caps.get(9).unwrap().as_str().parse::<i32>().unwrap(),                    
                },
            }
        );
    }

    let save = particules.clone();

    let mut nearest_id;
    let mut z = 0;

    loop {
        step(&mut particules);
        nearest_id = check(&particules);

        z += 1;

        if z > 1000 { 
            break;
        }
    }

    println!("Part #1: {}", nearest_id);

    let mut particules = save.clone();
    let mut z = 0;

    loop {
        step(&mut particules);
        clean_collide(&mut particules);

        z += 1;
        if z > 500 {
            break;
        }
    }

    println!("Part #2: {}", particules.len());
}
