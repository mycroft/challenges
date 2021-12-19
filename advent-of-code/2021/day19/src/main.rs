use std::collections::HashMap;

#[macro_use] extern crate scan_fmt;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Position {
    x: i16,
    y: i16,
    z: i16,
}

impl Position {
    fn manhattan(self, other: &Position) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()) as usize
    }
}

fn main() {
    let mut scanners = parse_file("input.txt");
    let mut all_points_scanner_0: Vec<Position> = vec![];
    let mut scanner_positions = vec![];

    for p in &scanners[0] {
        all_points_scanner_0.push(*p);
    }

    scanners.remove(0);

    loop {
        if scanners.is_empty() {
            break;
        }

        for scanner_id in 0..scanners.len() {
            let res = is_overlapping(
                &all_points_scanner_0,
                &scanners[scanner_id],
                12
            );

            if !res.0 {
                continue;
            }

            scanner_positions.push(res.1.unwrap());

            // Find an overlap!
            // Add the unknown points and move on.
            for new_p in res.2 {
                if !all_points_scanner_0.contains(&new_p) {
                    all_points_scanner_0.push(new_p);
                }
            }

            scanners.remove(scanner_id);
            break;
        }
    }

    println!("#1 {}", all_points_scanner_0.len());

    let mut manhattan_max = 0;

    for i in 0..scanner_positions.len() {
        for j in 0..scanner_positions.len() {
            if scanner_positions[i].manhattan(&scanner_positions[j]) > manhattan_max {
                manhattan_max = scanner_positions[i].manhattan(&scanner_positions[j]);
            }
        }
    }

    println!("#2 {}", manhattan_max);
}

fn parse_file(fp: &str) -> Vec<Vec<Position>> {
    let contents = std::fs::read_to_string(fp).expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut scanners = vec![];

    let mut current_scanner = vec![];
    for line in lines {
        let line = line.trim_end();

        if line.starts_with("--") {
            if current_scanner.is_empty() {
                continue;
            }

            scanners.push(current_scanner);
            current_scanner = vec![];
            continue;
        }

        if line.is_empty() {
            continue;
        }

        let (x, y, z) = scan_fmt!(
            line,
            "{},{},{}",
            i16, i16, i16
        ).unwrap();
        current_scanner.push(Position{x, y, z});
    }

    scanners.push(current_scanner);

    scanners
}

// The following functions should be OK:
fn pos_rotate_xy(p: &Position) -> Vec<Position> {
    vec![
        Position{x: p.x, y: p.y, z: p.z },
        Position{x: -p.x, y: -p.y, z: p.z },
        Position{x: -p.y, y: p.x, z: p.z },
        Position{x: p.y, y: -p.x, z: p.z }
    ]
}

fn pos_rotate_xz(p: &Position) -> Vec<Position> {
    vec![
        Position{x: p.x, y: p.y, z: p.z },
        Position{x: -p.x, y: p.y, z: -p.z },
        Position{x: p.z, y: p.y, z: -p.x },
        Position{x: -p.z, y: p.y, z: p.x },
    ]
}

fn pos_rotate_yz(p: &Position) -> Vec<Position> {
    vec![
        Position{x: p.x, y: p.y, z: p.z },
        Position{x: p.x, y: -p.y, z: -p.z },
        Position{x: p.x, y: p.z, z: -p.y },
        Position{x: p.x, y: -p.z, z: p.y }
    ]
}

// From a single position, return all possible orientations (24 vec)
// They must be all presents are we're gonna to use this vec to build
// the whole sets of possible orientations of a Position vec list.
fn get_all_orientations(l: &Position) -> Vec<Position> {
    let mut res = vec![];

    for a in pos_rotate_xy(l) {
        for b in pos_rotate_xz(&a) {
            for c in pos_rotate_yz(&b) {
                res.push(c);
            }
        }
    }

    res
}

// From a beacon position list, return all possible orientations (24 vec)
fn get_all_orientations_vec(l: &[Position]) -> Vec<Vec<Position>> {
    let mut temp_result = vec![vec![]; 64];

    for p in l {
        let a = get_all_orientations(p);
        for z in a.iter().enumerate() {
            temp_result[z.0].push(*z.1);
        }
    }

    let mut res = vec![];

    // Remove duplicates
    for el in temp_result {
        if !res.contains(&el) {
            res.push(el);
        }
    }

    res
}

// Find overlapping between two list. The intersection must be at least N common positions
// Inputs:
// - The known points in scanner 0 view
// - The new beacons viewed by scanner X
// - Number of required points that must match (12)
// Output:
// - If we found something or not
// - The scanner X position
// - The beacons position, as viewed by scanner 0.
fn is_overlapping(orig: &[Position], candidate: &[Position], required: usize) -> (bool, Option<Position>, Vec<Position>) {
    // We will list all vectors into both original & candidate vectors.
    // We will check if at least 12 vectors are similar

    let mut orig_vec = vec![];

    // orig doesn't change: We keept the same orientation
    for i in 0..orig.len() {
        for j in 1+i..orig.len() {
            orig_vec.push(
                (
                    orig[j].x - orig[i].x,
                    orig[j].y - orig[i].y,
                    orig[j].z - orig[i].z,
                    orig[i],
                    orig[j],
                )
            );
        }
    }

    for oriented_candidate in get_all_orientations_vec(candidate) {
        let mut candidate_vec = vec![];

        for i in 0..oriented_candidate.len() {
            for j in 0..oriented_candidate.len() {
                if i == j {
                    continue;
                }
                candidate_vec.push(
                    (
                        oriented_candidate[j].x - oriented_candidate[i].x,
                        oriented_candidate[j].y - oriented_candidate[i].y,
                        oriented_candidate[j].z - oriented_candidate[i].z,
                        oriented_candidate[i],
                        oriented_candidate[j],
                    )
                );
            }
        }

        // check if we have common elements in both candidate_vec & orig_vec
        let mut possible_vectors: HashMap<(i16, i16, i16), usize> = HashMap::new();

        for candidate in &candidate_vec {
            // we check if we have candidate vector only in orig_vec.
            for orig_pos in &orig_vec {
                if candidate.0 != orig_pos.0 || candidate.1 != orig_pos.1 || candidate.2 != orig_pos.2 {
                    continue;
                }

                *possible_vectors.entry(
                    (orig_pos.3.x - candidate.3.x, orig_pos.3.y - candidate.3.y, orig_pos.3.z - candidate.3.z)
                ).or_insert(0) += 1;
            }
        }

        for (v, k) in possible_vectors {
            if k < required {
                continue;
            }

            // Found a match!
            return (
                true,
                Some(Position{x: v.0, y: v.1, z: v.2}),
                oriented_candidate
                .iter()
                .map(|p| Position{x: p.x + v.0, y: p.y + v.1, z: p.z + v.2})
                .collect::<Vec<Position>>()
            );
        }
    }

    (false, None, vec![])
}

#[test]
fn test_get_all_orientations() {
    let orig = Position{x: 1, y: 2, z: 3};
    assert_eq!(64, get_all_orientations(&orig).len());

    let orig = vec![orig];
    assert_eq!(24, get_all_orientations_vec(&orig).len());
}

#[test]
fn test_overlapping() {
    let orig: Vec<Position> = vec![
        Position{x: 0, y: 2, z: 0}, Position{x: 4, y: 1, z: 0}, Position{x: 3, y: 3, z: 0}
    ];
    let candidate: Vec<Position> = vec![
        Position{x: -1, y: -1, z: 0}, Position{x: -5, y: 0, z: 0}, Position{x: -2, y: 1, z: 0}
    ];
    
    let res = is_overlapping(&orig, &candidate, 3);
    assert_eq!(true, res.0);
    assert_eq!(Some(Position{x: 5, y: 2, z: 0}), res.1);
    assert_eq!(vec![
        Position{x: 4, y: 1, z: 0}, Position{x: 0, y: 2, z: 0}, Position{x: 3, y: 3, z: 0}
    ], res.2);
}
