// to be used later:
// use std::fs::read_to_string;

use std::{ops::{Add, AddAssign}, fmt::Display};
use num::integer::lcm;

#[derive(Debug, Clone, Copy)]
struct Coords {
    x: isize,
    y: isize,
    z: isize,
}

impl Coords {
    fn new(x: isize, y: isize, z: isize) -> Self {
        Self {
            x, y, z
        }
    }

    fn default() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }
}

impl Add for Coords {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Coords {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<x={:3}, y={:3}, z={:3}>", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone, Copy)]
struct Moon {
    position: Coords,
    velocity: Coords,
}

impl Moon {
    fn new(position: Coords) -> Self {
        Self {
            position,
            velocity: Coords::default(),
        }
    }

    fn compute_gravity(&self, moons: &Vec<Moon>) -> Coords {
        let mut result = Coords::default();

        for moon in moons {
            result.x += (moon.position.x - self.position.x).signum();
            result.y += (moon.position.y - self.position.y).signum();
            result.z += (moon.position.z - self.position.z).signum();
        }

        result
    }
    
    fn energy(&self) -> isize {
        (self.position.x.abs() + self.position.y.abs() + self.position.z.abs()) * (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs())
    }
}

impl Display for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos={}, vel={}", self.position, self.velocity)
    }
}

fn run_step(moons: &mut Vec<Moon>) {
    // for each moon, compute gravity
    for moon_idx in 0..moons.len() {
        let gravity = moons[moon_idx].compute_gravity(moons);

        // apply gravity to velicity
        moons[moon_idx].velocity += gravity;
    }

    // now that velocity is known, compute now moon positions
    for moon_idx in 0..moons.len() {
        let velocity = moons[moon_idx].velocity;
        moons[moon_idx].position += velocity;
    }
}

fn run(moons: &mut Vec<Moon>, steps: usize) -> isize {
    for _ in 0..steps {
        run_step(moons);
    };

    moons.iter().map(|x| x.energy()).sum()
}

fn run_cyclic(moons: &mut Vec<Moon>) -> isize {
    let mut cycles = (0isize, 0isize, 0isize);
    let initial_state = moons.clone();
    let mut index = 0;

    // we need to find the cycle where all velocities for x, y, z == 0 and for initial state to be found for x, y, z.
    loop {
        run(moons, 1);
        index += 1;

        // is x in initial state?
        if cycles.0 == 0 && moons.len() == moons.iter().enumerate().filter(|(idx, &m)| initial_state[*idx].position.x == m.position.x && m.velocity.x == 0).count() {
            cycles.0 = index;
        }

        // is y in initial state?
        if cycles.1 == 0 && moons.len() == moons.iter().enumerate().filter(|(idx, &m)| initial_state[*idx].position.y == m.position.y && m.velocity.y == 0).count() {
            cycles.1 = index;
        }

        // is z in initial state?
        if cycles.2 == 0 && moons.len() == moons.iter().enumerate().filter(|(idx, &m)| initial_state[*idx].position.z == m.position.z && m.velocity.z == 0).count() {
            cycles.2 = index;
        }

        if cycles.0 != 0 && cycles.1 != 0 && cycles.2 != 0 {
            break;
        }
    }

    lcm(cycles.0, lcm(cycles.1, cycles.2))
}

fn get_input() -> Vec<Moon> {
    let mut moons = Vec::new();

    moons.push(Moon::new(Coords::new(4, 12, 13)));
    moons.push(Moon::new(Coords::new(-9, 14, -3)));
    moons.push(Moon::new(Coords::new(-7, -1, 2)));
    moons.push(Moon::new(Coords::new(-11, 17, -1)));

    moons
}

fn get_sample_input() -> Vec<Moon> {
    let mut moons = Vec::new();

    moons.push(Moon::new(Coords::new(-1, 0, 2)));
    moons.push(Moon::new(Coords::new(2, -10, -7)));
    moons.push(Moon::new(Coords::new(4, -8, 8)));
    moons.push(Moon::new(Coords::new(3, 5, -1)));

    moons
}

fn get_sample_input2() -> Vec<Moon> {
    let mut moons = Vec::new();

    moons.push(Moon::new(Coords::new(-8, -10, 0)));
    moons.push(Moon::new(Coords::new(5, 5, 10)));
    moons.push(Moon::new(Coords::new(2, -7, 3)));
    moons.push(Moon::new(Coords::new(9, -8, -3)));

    moons
}

fn main() {
    let mut moons = get_input(); 
    println!("#1 {}", run(&mut moons, 1000)); // 5350

    let mut moons = get_input();
    println!("#2 {}", run_cyclic(&mut moons)); // 467034091553512
}

#[test]
fn test_sample() {
    let mut moons = get_sample_input();
    assert_eq!(
        179,
        run(&mut moons, 10)
    );

    let mut moons = get_sample_input2();
    assert_eq!(
        1940,
        run(&mut moons, 100)
    );
}

#[test]
fn test_sample_step2() {
    let mut moons = get_sample_input();
    assert_eq!(
        2772,
        run_cyclic(&mut moons)
    );

    let mut moons = get_sample_input2();
    assert_eq!(
        4686774924,
        run_cyclic(&mut moons)
    );
}