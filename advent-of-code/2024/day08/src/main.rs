use std::fs::read_to_string;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn antinode(&self, other: &Pos) -> HashSet<Pos> {
        let mut result = HashSet::new();

        let x_abs = (self.x - other.x).abs();
        let y_abs = (self.y - other.y).abs();

        let x0;
        let y0;

        let x1;
        let y1;

        if self.x < other.x {
            x0 = self.x - x_abs;
            x1 = other.x + x_abs;
        } else {
            x0 = self.x + x_abs;
            x1 = other.x - x_abs;
        }

        if self.y < other.y {
            y0 = self.y - y_abs;
            y1 = other.y + y_abs;
        } else {
            y0 = self.y + y_abs;
            y1 = other.y - y_abs;
        }

        result.insert(Pos { x: x0, y: y0 });
        result.insert(Pos { x: x1, y: y1 });

        result
    }

    fn is_in_map(&self, map: &Map) -> bool {
        self.x < map.width && self.y < map.height && self.x >= 0 && self.y >= 0
    }

    fn are_in_line(&self, other0: &Pos, other1: &Pos) -> bool {
        if self.x == other0.x && self.x == other1.x {
            return true;
        }

        if self.y == other0.y && self.y == other1.y {
            return true;
        }

        if (self.x - other0.x) * (self.y - other1.y) == (self.x - other1.x) * (self.y - other0.y) {
            return true;
        }

        false
    }

    // retrieves all positions in the same line as self and other
    fn get_all_pos_in_line(&self, other: &Pos, map: &Map) -> HashSet<Pos> {
        let mut result = HashSet::new();

        result.insert(*self);
        result.insert(*other);

        for x in 0..map.width {
            for y in 0..map.height {
                let pos = Pos { x, y };
                if pos == *self || pos == *other {
                    continue;
                }

                if pos.are_in_line(self, other) {
                    result.insert(pos);
                }
            }
        }

        result
    }
}

#[derive(Debug)]
struct Map {
    width: isize,
    height: isize,
    antennas: HashMap<char, HashSet<Pos>>,
}


fn read_input(fp: &str) -> Map {
    let contents = read_to_string(fp).unwrap();
    let mut antennas = HashMap::new();
    let mut width = 0;
    let mut height = 0;

    for (y, line) in contents.lines().enumerate() {
        height += 1;
        width = line.len();
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                let entry = antennas.entry(c).or_insert(HashSet::new());
                entry.insert(Pos { x: x as isize, y: y as isize });
            }
        }
    }

    Map {
        width: width as isize,
        height: height as isize,
        antennas,
    }
}

fn find_antinodes_step1(map: &Map) -> HashSet<Pos> {
    let mut antinodes = HashSet::new();

    for positions in map.antennas.values() {
        let couples = positions.iter().combinations(2).collect::<Vec<_>>();

        for couple in couples {
            // println!("{} {:?} {:?}", antenna, couple[0], couple[1]);
            let couple_antinodes = couple[0].antinode(couple[1]);
            for antinode in couple_antinodes {
                if antinode.is_in_map(map) {
                    antinodes.insert(antinode);
                }
            }
        }
    }

    antinodes
}

fn find_antinodes_step2(map: &Map) -> HashSet<Pos> {
    let mut antinodes = HashSet::new();

    for positions in map.antennas.values() {
        let couples = positions.iter().combinations(2).collect::<Vec<_>>();

        for couple in couples {
            // println!("{} {}", (couple[0].x - couple[1].x).abs(), (couple[0].y - couple[1].y).abs());
            let all_pos = couple[0].get_all_pos_in_line(couple[1], map);

            antinodes.extend(&all_pos);
        }
    }

    antinodes
}

fn print_map(map: &Map, antinodes: &HashSet<Pos>) {
    for y in 0..map.height {
        for x in 0..map.width {
            let pos = Pos { x, y };
            if antinodes.contains(&pos) {
                print!("#");
            } else {
                let mut found = false;
                for (antenna, positions) in &map.antennas {
                    if positions.contains(&pos) {
                        print!("{}", antenna);
                        found = true;
                        break;
                    }
                }
                if !found {
                    print!(".");
                }
            }
        }
        println!();
    }
}

fn main() {
    let input = read_input("input.txt");
    // println!("{:?}", input);

    let antinodes1 = find_antinodes_step1(&input);
    let antinodes2 = find_antinodes_step2(&input);

    println!("#1 {}", antinodes1.len());
    println!("#2 {}", antinodes2.len());

    // print_map(&input, &antinodes2);
}
