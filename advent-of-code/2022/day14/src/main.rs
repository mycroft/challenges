use std::fs::read_to_string;
use std::collections::HashMap;
use std::convert::From;

#[derive(Debug)]
enum TileContent {
    Wall,
    Sand
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    x: isize,
    y: isize
}

impl From<&str> for Pos {
    fn from(s: &str) -> Self {
        let parts = s.split(",").collect::<Vec<&str>>();

        Pos {
            x: parts[0].parse::<isize>().unwrap(),
            y: parts[1].parse::<isize>().unwrap(),
        }
    }
}

impl Pos {
    fn all_points_from_to(&self, to: &Self) -> Vec<Pos> {
        let mut all = Vec::new();

        let dir = (
            to.x as isize - self.x as isize,
            to.y as isize - self.y as isize,
        );

        let mut current_point = self.clone();

        while current_point != *to {
            all.push(current_point);

            current_point.x = current_point.x + dir.0.signum();
            current_point.y = current_point.y + dir.1.signum();
        }

        all.push(current_point);

        all
    }
}

#[test]
fn test_all_points_from_to() {
    assert_eq!(
        [Pos{x: 123, y: 1}, Pos{x: 123, y: 2}].to_vec(),
        Pos{x: 123, y: 1}.all_points_from_to(&Pos{x: 123, y: 2})
    );
}

// returns (lower ground coordonnate, an hashmap with known tiles)
fn parse(fp: &str) -> (isize, HashMap<Pos, TileContent>) {
    let mut hm = HashMap::new();

    let contents = read_to_string(fp).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut ground = 0;

    for line in lines {
        let coords = line.split(" -> ").map(|x| x.into()).collect::<Vec<Pos>>();
        let mut current_coord = coords[0];
        for coord in coords {
            if coord.y > ground {
                ground = coord.y
            }
            if current_coord == coord {
                continue;
            }

            let all = current_coord.all_points_from_to(&coord);
            for p in all {
                hm.insert(
                    p,
                    TileContent::Wall,
                );
            }

            current_coord = coord;
        }
    }

    (ground, hm)
}

fn display(hm: &HashMap<Pos, TileContent>) {
    // find the high, the x size, the y size.
    let mut corner = Pos{x: 500, y: 0};
    let mut x_size = 0; // max value found - corner.x
    let mut y_size = 0; // max value found.

    for el in hm {
        if el.0.y > y_size {
            y_size = el.0.y;
        }
        if el.0.x > x_size {
            x_size = el.0.x;
        }
        if el.0.x < corner.x {
            corner.x = el.0.x;
        }
    }

    x_size = x_size - corner.x;

    // Drawing the map
    for y in corner.y..=corner.y + y_size {
        for x in corner.x..=corner.x + x_size {
            if x == 500 && y == 0 {
                print!("+");
                continue;
            }

            if let Some(el) = hm.get(&Pos{x, y}) {
                match el {
                    TileContent::Wall => { print!("#") },
                    TileContent::Sand => { print!("o") },
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn play(hm: &mut HashMap<Pos, TileContent>, ground: isize, with_floor: bool) -> Option<Pos> {
    let mut sand_pos = Pos{x: 500, y: 0};

    let dirs = [
        (0isize, 1isize),
        (-1, 1),
        (1, 1),
    ];

    if with_floor && hm.contains_key(&sand_pos) {
        return None;
    }

    loop {
        // can we move?
        let mut new_sand_pos = sand_pos;
        let mut can_move = false;

        for dir in dirs {
            new_sand_pos = Pos{
                x: sand_pos.x + dir.0,
                y: sand_pos.y + dir.1
            };

            if !hm.contains_key(&new_sand_pos) {
                can_move = true;
                break;
            }
        }

        if can_move {
            if with_floor == false && sand_pos.y >= ground {
                break None;
            } else if with_floor && sand_pos.y - 1 >= ground {
                hm.insert(sand_pos, TileContent::Sand);
                break Some(sand_pos);
            }

            sand_pos = new_sand_pos;
        } else {
            // can not move
            hm.insert(sand_pos, TileContent::Sand);
            break Some(sand_pos);
        }
    }
}

fn run_sand(fp: &str, with_floor: bool) -> usize {
    let (ground, mut hm) = parse(fp);
    let mut count = 0;
    loop {
        if let Some(_pos) = play(&mut hm, ground, with_floor) {
            count += 1
        } else {
            break;
        }
    }

    // Enable this for the big picture:
    // display(&hm);

    count
}
fn main() {
    println!("#1 {}", run_sand("input.txt", false));
    println!("#2 {}", run_sand("input.txt", true));
}

#[test]
fn test_sample() {
    assert_eq!(
        24,
        run_sand("input.txt_test", false)
    );

    assert_eq!(
        93,
        run_sand("input.txt_test", true)
    )
}