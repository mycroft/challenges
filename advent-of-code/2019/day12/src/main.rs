use std::fmt;

#[macro_use] extern crate scan_fmt;

#[derive(Clone, Debug)]
struct Pos {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Clone, Debug)]
struct Vel {
    x: isize,
    y: isize,
    z: isize,
}

#[derive(Clone)]
struct Star {
    pos: Pos,
    vel: Vel,
}

impl fmt::Debug for Star {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "pos=<x={:3}, y={:3}, z={:3}>, vel=<x={:3}, y={:3}, z={:3}>",
            self.pos.x, self.pos.y, self.pos.z,
            self.vel.x, self.vel.y, self.vel.z,
        )
    }
}

fn parse(fp: &str) -> Vec<Star> {
    let mut res = vec![];

    let contents = std::fs::read_to_string(fp).expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();

    for line in lines {
        let (x, y, z) = scan_fmt!(
            line,
            "<x={}, y={}, z={}>",
            isize, isize, isize
        ).expect("position");

        res.push(Star{
            pos: Pos{x, y, z},
            vel: Vel{x: 0, y: 0, z:0},
        });
    }

    res
}

fn simulation(stars: &[Star], steps: usize) -> (isize, (isize, isize, isize)) {
    let orig = stars.to_owned();
    let mut stars = stars.to_owned();
    let mut step: isize = 0;

    let mut res0 : isize = 0;
    let mut res1 : (isize, isize, isize) = (0, 0, 0);

    loop {
        // println!("After {} steps:", step);
        // for star in &stars {
        //     println!("{:?}", star);
        // }

        // Compute Velocity for each stars
        for star_0 in 0..stars.len() {
            for star_1 in star_0+1..stars.len() {
                if stars[star_0].pos.x > stars[star_1].pos.x {
                    stars[star_0].vel.x -= 1;
                    stars[star_1].vel.x += 1;
                } else if stars[star_0].pos.x < stars[star_1].pos.x {
                    stars[star_0].vel.x += 1;
                    stars[star_1].vel.x -= 1;
                }

                if stars[star_0].pos.y > stars[star_1].pos.y {
                    stars[star_0].vel.y -= 1;
                    stars[star_1].vel.y += 1;
                } else if stars[star_0].pos.y < stars[star_1].pos.y {
                    stars[star_0].vel.y += 1;
                    stars[star_1].vel.y -= 1;
                }

                if stars[star_0].pos.z > stars[star_1].pos.z {
                    stars[star_0].vel.z -= 1;
                    stars[star_1].vel.z += 1;
                } else if stars[star_0].pos.z < stars[star_1].pos.z {
                    stars[star_0].vel.z += 1;
                    stars[star_1].vel.z -= 1;
                }
            }
        }

        // Apply velocity to stars
        for star in stars.iter_mut() {
            star.pos.x += star.vel.x;
            star.pos.y += star.vel.y;
            star.pos.z += star.vel.z;
        }

        step += 1;

        if step == steps as isize {
            res0 = stars.iter().map(|s| {
                    (s.pos.x.abs() + s.pos.y.abs() + s.pos.z.abs()) * (s.vel.x.abs() + s.vel.y.abs() + s.vel.z.abs())
                }).sum();
        }

        // Verify if we're back in initial state, for each coordinate
        let match_x = stars
            .iter()
            .enumerate()
            .map(|(idx, s)|  {
                (s.pos.x == orig[idx].pos.x, s.vel.x == 0)
            })
            .all(|v| v == (true, true));

        if match_x && res1.0 == 0 {
            res1.0 = step;
        }

        let match_y = stars
            .iter()
            .enumerate()
            .map(|(idx, s)|  {
                (s.pos.y == orig[idx].pos.y, s.vel.y == 0)
            })
            .all(|v| v == (true, true));

        if match_y && res1.1 == 0 {
            res1.1 = step;
        }

        let match_z = stars
            .iter()
            .enumerate()
            .map(|(idx, s)|  {
                (s.pos.z == orig[idx].pos.z, s.vel.z == 0)
            })
            .all(|v| v == (true, true));

        if match_z && res1.2 == 0 {
            res1.2 = step;
        }
       
        // End condition
        if step >= steps as isize && res1.0 != 0 && res1.1 != 0 && res1.2 != 0 {
            break;
        }
    }

    (res0, res1)
}

use num::integer::lcm;

fn get_lcm(res: (isize, isize, isize)) -> isize {
    lcm(res.0, lcm(res.1, res.2))
}

fn main() {
    let stars = parse("input.txt");
    let res = simulation(&stars, 1000);
    println!("#1 {}", res.0); // 5350
    println!("#2 {}", get_lcm(res.1)); // xx
    
}

#[test]
fn test_0() {
    let stars = parse("input.txt_test0");
    assert_eq!(179, simulation(&stars, 10).0);

    let stars = parse("input.txt_test1");
    assert_eq!(1940, simulation(&stars, 100).0);
}

#[test]
fn test_1() {
    let stars = parse("input.txt_test0");
    assert_eq!(2772, get_lcm(simulation(&stars, 10).1));

    let stars = parse("input.txt_test1");
    assert_eq!(4686774924, get_lcm(simulation(&stars, 100).1));
}