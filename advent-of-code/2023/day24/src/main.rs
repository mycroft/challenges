use std::fs;

use z3::{Config, Context, Solver};
use z3::ast::{Int, Ast};

#[derive(Debug, Clone, Copy)]
struct Triplet {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Debug, Clone, Copy)]
struct Line  {
    from: Triplet,
    to: Triplet,
}

#[derive(Debug)]
struct Hailtstone {
    position: Triplet,
    velocity: Triplet,
}

impl From<&str> for Triplet {
    fn from(value: &str) -> Self {
        let mut parts = value.split(',');

        Triplet{
            x: parts.next().unwrap().trim().parse().unwrap(),
            y: parts.next().unwrap().trim().parse().unwrap(),
            z: parts.next().unwrap().trim().parse().unwrap(),
        }
    }
}

impl From<&str> for Hailtstone {
    fn from(value: &str) -> Self {
        let mut parts = value.split('@');

        Hailtstone{
            position: parts.next().unwrap().into(),
            velocity: parts.next().unwrap().into(),
        }
    }
}

impl Line {
    fn intersection(&self, other: &Line) -> Option<(f64, f64)> {
        let a1 = self.to.y - self.from.y;
        let b1 = self.from.x - self.to.x;
        let c1 = a1 * self.from.x + b1 * self.from.y;

        let a2 = other.to.y - other.from.y;
        let b2 = other.from.x - other.to.x;
        let c2 = a2 * other.from.x + b2 * other.from.y;


        let delta = a1 * b2 - a2 * b1;

        if delta == 0.0 {
            return None;
        }

        Some(
            ((b2 * c1 - b1 * c2) / delta, (a1 * c2 - a2 * c1) / delta)
        )
    }
}

fn get_line(value: &Hailtstone, limit_min: f64, limit_max: f64) -> Line {
    let mut count = 0.;

    while (value.velocity.x > 0. && (value.position.x + count * value.velocity.x) < limit_max) || (value.velocity.x < 0. && (value.position.x + count * value.velocity.x) > limit_min) {
        count += limit_max;
    }

    Line {
        from: value.position,
        to: Triplet {
            x: value.position.x + count * value.velocity.x,
            y: value.position.y + count * value.velocity.y,
            z: value.position.z + count * value.velocity.z,
        }
    }
}

fn read(fp: &str) -> Vec<Hailtstone> {
    let mut result = Vec::new();

    let contents = fs::read_to_string(fp).expect("a file to open");
    let lines = contents.lines();

    for line in lines {
        result.push(line.into());
    }

    result
}

fn is_in_range(from: f64, to: f64, value: f64) -> bool {
    if from < to {
        from <= value && value <= to
    } else {
        to <= value && value <= from
    }
}

fn solve2(hailstones: &[Hailtstone]) -> isize {
    let z3_conf = Config::new();
    let ctx = Context::new(&z3_conf);

    let solver = Solver::new(&ctx);

    let x = Int::new_const(&ctx, "x");
    let y = Int::new_const(&ctx, "y");
    let z = Int::new_const(&ctx, "z");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for (i, _) in hailstones.iter().enumerate().take(4) {
        let t = Int::new_const(&ctx, format!("t{i}"));
        solver.assert(&t.ge(&Int::from_i64(&ctx, 0)));

        let eq_1_left = Int::add(&ctx, &[&x, &Int::mul(&ctx, &[&vx, &t])]);
        let eq_1_right = Int::add(&ctx, &[&Int::from_i64(&ctx, hailstones[i].position.x as i64), &Int::mul(&ctx, &[&Int::from_i64(&ctx, hailstones[i].velocity.x as i64), &t])]);
        solver.assert(&eq_1_left._eq(&eq_1_right));

        let eq_2_left = Int::add(&ctx, &[&y, &Int::mul(&ctx, &[&vy, &t])]);
        let eq_2_right = Int::add(&ctx, &[&Int::from_i64(&ctx, hailstones[i].position.y as i64), &Int::mul(&ctx, &[&Int::from_i64(&ctx, hailstones[i].velocity.y as i64), &t])]);
        solver.assert(&eq_2_left._eq(&eq_2_right));

        let eq_3_left = Int::add(&ctx, &[&z, &Int::mul(&ctx, &[&vz, &t])]);
        let eq_3_right = Int::add(&ctx, &[&Int::from_i64(&ctx, hailstones[i].position.z as i64), &Int::mul(&ctx, &[&Int::from_i64(&ctx, hailstones[i].velocity.z as i64), &t])]);
        solver.assert(&eq_3_left._eq(&eq_3_right));
    }

    solver.check();

    let model = solver.get_model().unwrap();
    
    let xv = model.eval(&x, true).unwrap().as_i64();
    let yv = model.eval(&y, true).unwrap().as_i64();
    let zv = model.eval(&z, true).unwrap().as_i64();

    // println!("{:?} {:?} {:?} {:?}", solver.check(), xv, yv, zv);
    // println!("{}", xv.unwrap() + yv.unwrap() + zv.unwrap());

    // assert_eq!(solver.check(), SatResult::Sat);

    (xv.unwrap() + yv.unwrap() + zv.unwrap()) as isize
}

fn main() {
    let hailstones = read("input.txt");

    let limit_min = 200000000000000_f64;
    let limit_max = 400000000000000_f64;

    let mut res = 0;

    let mut lines = Vec::new();
    for entry in &hailstones {
        lines.push(get_line(entry, limit_min, limit_max));
    }

    for i0 in 0..lines.len()-1 {
        for i1 in i0+1..lines.len() {
            if let Some(inter) = lines[i0].intersection(&lines[i1]) {
                if !is_in_range(lines[i0].from.x, lines[i0].to.x, inter.0) {
                    continue;
                }

                if !is_in_range(lines[i1].from.x, lines[i1].to.x, inter.0) {
                    continue;
                }

                if !is_in_range(lines[i0].from.y, lines[i0].to.y, inter.1) {
                    continue;
                }

                if !is_in_range(lines[i1].from.y, lines[i1].to.y, inter.1) {
                    continue;
                }

                if inter.0 >= limit_min && inter.0 <= limit_max && inter.1 >= limit_min && inter.1 <= limit_max {
                    res += 1;
                }
            }
        }
    }

    println!("#1 {}", res);
    println!("#2 {}", solve2(&hailstones));
}
