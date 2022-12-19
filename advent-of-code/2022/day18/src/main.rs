use std::fs::read_to_string;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cube {
    x: isize,
    y: isize,
    z: isize,
}

impl Cube {
    // 2 cubes are adjacent if they share 2 coords and have a delta of one on
    // the 3rd.
    fn is_adjacent(&self, o: &Cube) -> bool {
        let delta = if self.x == o.x && self.y == o.y {
            (self.z - o.z).abs() 
        } else if self.x == o.x && self.z == o.z {
            (self.y - o.y).abs() 
        } else if self.y == o.y && self.z == o.z {
            (self.x - o.x).abs() 
        } else {
            42
        };

        delta == 1
    }

    fn get_adjacent_cubes(&self) -> Vec<Cube> {
        let mut result = Vec::new();
        let dirs = [
            (0isize, 0isize, -1isize),
            (0isize, 0isize, 1isize),
            (0isize, -1isize, 0isize),
            (0isize, 1isize, 0isize),
            (-1isize, 0isize, 0isize),
            (1isize, 0isize, 0isize),
        ];

        for dir in dirs {
            result.push(Cube{x: self.x + dir.0, y: self.y + dir.1, z: self.z + dir.2});
        }

        result
    }
}

fn parse(fp: &str) -> HashSet<Cube> {
    let c = read_to_string(fp).unwrap();
    let lines: Vec<&str> = c.lines().collect();
    let mut result = HashSet::new();

    for line in lines {
        let coords: Vec<isize> = line.split(",").map(|x| x.parse::<isize>().unwrap()).collect();
        result.insert(Cube{
            x: coords[0],
            y: coords[1],
            z: coords[2],
        });
    }

    result
}

// This is not optimized as we are doing twice the same check.
// This could be improved using Vec instead of HashSet.
fn find_exposed_part1(cubes: &HashSet<Cube>) -> isize {
    let mut result = 0;

    for cube in cubes {
        for other in cubes {
            if cube.is_adjacent(&other) {
                result += 1;
            }
        }
    }

    (cubes.len() as isize) * 6 - result
}

// expand/flood file until we can no longer or reach the limit.
// here the limit is a static number of cubes, but there are nicer ways to do.
fn expand(cube: &Cube, cubes: &HashSet<Cube>) -> HashSet<Cube> {
    let mut working_on_cubes = Vec::new();
    let mut current_idx = 0;
    let mut result = HashSet::new();

    working_on_cubes.push(cube.clone());
    result.insert(cube.clone());

    // This is a big limit, but it looks like 1000 is too low.
    // This should be somehow computed.
    // A nice way to do that would be to use the minimum bounding box:
    // https://en.wikipedia.org/wiki/Minimum_bounding_box
    // We could have an external box we can not touch. If we touch it, then the game is over.
    let limit = 10000;

    loop {
        if current_idx >= working_on_cubes.len() {
            // We hit the limit.
            break;
        }

        if result.len() > limit {
            // We consider the hole too big. Let's cancel it and go ahead.
            return HashSet::new();
        }

        let cube = working_on_cubes[current_idx];
        let adj_cubes = cube.get_adjacent_cubes();

        for adj_cube in &adj_cubes {
            if !cubes.contains(adj_cube) && !result.contains(adj_cube) {
                result.insert(adj_cube.clone());
                working_on_cubes.push(adj_cube.clone());
            } 
        }

        current_idx += 1;
    }

    result    
}

// We will fill cubes as long as there is no any hole.
// To do that, we find adjacent empty holes, and we fill them.
fn find_exposed_part2(cubes: &HashSet<Cube>) -> isize {
    // lets find adjacent cubes that shares a lot of surfaces (6)
    // with others existing cubes

    let mut cubes_with_holes = HashSet::new();

    for cube in cubes {
        let adj_cubes = cube.get_adjacent_cubes();
        for adj_cube in &adj_cubes {
            if cubes.contains(adj_cube) {
                continue;
            }

            let mut count_adjacent = 0;
            for other in cubes {
                if adj_cube.is_adjacent(other) {
                    count_adjacent += 1;
                }
            }

            if count_adjacent >= 3 {
                // Expand this cube as fas as we can go.
                let expanded_cubes = expand(adj_cube, cubes);

                if expanded_cubes.len() != 0 {
                    for expanded_cube in expanded_cubes {
                        cubes_with_holes.insert(expanded_cube);
                    }
                }
            }
        }
    }

    let mut cubes = cubes.clone();
    for cube in cubes_with_holes {
        cubes.insert(cube);
    }

    find_exposed_part1(&cubes)
}

fn main() {
    let cubes = parse("input.txt");

    println!("#1 {}", find_exposed_part1(&cubes)); // 4300
    println!("#2 {}", find_exposed_part2(&cubes)) // ?
}

#[test]
fn test_is_adjacent() {
    assert!(
        Cube{x: 1, y: 1, z: 1}.is_adjacent(&Cube{x: 2, y: 1, z: 1})
    );
 
    assert!(
        !Cube{x: 1, y: 1, z: 1}.is_adjacent(&Cube{x: 3, y: 1, z: 1})
    );

    assert!(
        !Cube{x: 1, y: 1, z: 1}.is_adjacent(&Cube{x: 1, y: 1, z: 1})
    );
}

#[test]
fn test_sample() {
    let cubes = parse("input.txt_test");

    assert_eq!(
        64,
        find_exposed_part1(&cubes)
    );

    assert_eq!(
        58,
        find_exposed_part2(&cubes)
    );
}