use std::cmp::{min,max};

#[macro_use] extern crate scan_fmt;

#[derive(Copy, Clone, Debug, Hash)]
struct Cube {
    x1: i128,
    x2: i128,
    y1: i128,
    y2: i128,
    z1: i128,
    z2: i128,
}

impl Cube {
    fn dont_overlap(self, other: &Cube) -> bool {
        other.x1 > self.x2 || self.x1 > other.x2
        || other.y1 > self.y2 || self.y1 > other.y2
        || other.z1 > self.z2 || self.z1 > other.z2
    }

    fn size(self) -> usize {
        ((self.x2 + 1 - self.x1) * (self.y2 + 1 - self.y1) * (self.z2 + 1 - self.z1)) as usize
    }

    // Takes 2 cubes on input, returns the result of all smaller cubes.
    // If remove, then the cubes created from the "other" are not included in the result.
    fn overlap_and_split(self, other: &Cube) -> Vec<Cube> {
        let mut smaller_cubes = vec![];

        // Cubes are not overlapping: No need to split them. Returns them as it.
        if self.dont_overlap(other) {
            smaller_cubes.push(self);
            return smaller_cubes;
        }

        if other.x1 > self.x1 {
            // We leave only the self.x1...other.x1 part.
            smaller_cubes.push(Cube{
                x1: self.x1, x2: other.x1 - 1,
                y1: self.y1, y2: self.y2,
                z1: self.z1, z2: self.z2,
            });
        }
        if other.x2 < self.x2 {
            // other's [-5, 5] overlaps self [0, 10]
            // Smaller cube we want to keep is [6, 10]
            smaller_cubes.push(Cube{
                x1: other.x2 + 1, x2: self.x2,
                y1: self.y1, y2: self.y2,
                z1: self.z1, z2: self.z2,
            });
        }

        if other.y1 > self.y1 {
            // For Y, we're using the same thing than for above's X.
            // For X, we keep only the smaller part if we cut above.
            smaller_cubes.push(Cube{
                x1: max(self.x1, other.x1), x2: min(self.x2, other.x2),
                y1: self.y1, y2: other.y1 - 1,
                z1: self.z1, z2: self.z2,
            });
        }

        if other.y2 < self.y2 {
            smaller_cubes.push(Cube{
                x1: max(self.x1, other.x1), x2: min(self.x2, other.x2),
                y1: other.y2 + 1, y2: self.y2,
                z1: self.z1, z2: self.z2,
            });
        }

        if other.z1 > self.z1 {
            smaller_cubes.push(Cube{
                x1: max(self.x1, other.x1), x2: min(self.x2, other.x2),
                y1: max(self.y1, other.y1), y2: min(self.y2, other.y2),
                z1: self.z1, z2: other.z1 - 1,
            });
        }

        if other.z2 < self.z2 {
            smaller_cubes.push(Cube{
                x1: max(self.x1, other.x1), x2: min(self.x2, other.x2),
                y1: max(self.y1, other.y1), y2: min(self.y2, other.y2),
                z1: other.z2 + 1, z2: self.z2,
            });
        }

        smaller_cubes
    }
}

fn parse(fp: &str) -> Vec<(bool, Cube)> {
    let contents = std::fs::read_to_string(fp).expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut res = vec![];

    for line in lines {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }

        let (state, x1, x2, y1, y2, z1, z2) = scan_fmt!(
            line,
            "{} x={}..{},y={}..{}, z={}..{}",
            String, i128, i128, i128, i128, i128, i128
        ).unwrap();

        res.push(
            (
                state == "on",
                Cube {
                    x1, x2, y1, y2, z1, z2,
                }
            )
        );
    }

    res
}

fn parse_and_resolve(fp: &str, extended: bool) -> usize {
    let mut world: Vec<Cube> = vec![];
    let orders = parse(fp);

    for order in &orders {
        if world.is_empty() && order.0 {
            world.push(order.1);
            continue;
        }

        let new_cube = order.1;

        if !extended {
            if new_cube.x1 > 50 || new_cube.x2 < -50 
            || new_cube.y1 > 50 || new_cube.y2 < -50 
            || new_cube.z1 > 50 || new_cube.z2 < -50 {
                continue;
            }
        }

        let mut new_world = vec![];

        for w in world {
            let mut new_cubes = w.overlap_and_split(&new_cube);
            new_world.append(&mut new_cubes);
        }

        // At the end, we add the remaining that should remain, if we don't remove it:
        if order.0 {
            new_world.push(new_cube);
        }
        
        world = new_world;
    }

    let mut total = 0;
    for area in &world {
        total += area.size();
    }

    total
}

fn main() {
    println!("#1: {}", parse_and_resolve("input.txt", false));
    println!("#2: {}", parse_and_resolve("input.txt", true));
}

#[test]
fn test_overlap() {
    let orig = Cube{x1: 10, x2: 12, y1: 10, y2: 12, z1: 10, z2: 12};
    assert_eq!(false, orig.dont_overlap(&Cube{x1: 11, x2: 13, y1: 11, y2: 13, z1: 11, z2: 13}));
    assert_eq!(true, orig.dont_overlap(&Cube{x1: 20, x2: 22, y1: 20, y2: 22, z1: 20, z2: 22}));
}

#[test]
fn trivial_test() {
    assert_eq!(39, parse_and_resolve("input.txt_test0", false));
    assert_eq!(590784, parse_and_resolve("input.txt_test1", false));
    assert_eq!(2758514936282235, parse_and_resolve("input.txt_test2", true));
}
