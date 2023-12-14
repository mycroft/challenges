use std::fs;
use std::collections::{HashSet, HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Location {
    x: isize,
    y: isize,
}

fn readfile(filepath: &str) -> (HashSet<Location>, Location) {
    let contents = fs::read_to_string(filepath).expect("a file to open");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut galaxies = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in lines.iter().enumerate() {
        max_y = y;
        max_x = line.len() - 1;
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            galaxies.insert(Location{x: x as isize, y: y as isize});
        }
    }

    (galaxies, Location{x: max_x as isize, y: max_y as isize})
}

fn expand(galaxies: &HashSet<Location>, max_locations: &Location, expand_size: isize) -> (HashSet<Location>, Location) {
    let mut rows = Vec::new();
    let mut cols = Vec::new();

    let mut results = HashSet::new();

    for y in 0..=max_locations.y {
        let mut has_element = false;
        for x in 0..=max_locations.x {
            if galaxies.contains(&Location{x, y}) {
                has_element = true;
                break;
            }
        }

        if !has_element {
            rows.push(y);
        }
    }

    for x in 0..=max_locations.x {
        let mut has_element = false;
        for y in 0..=max_locations.y {
            if galaxies.contains(&Location{x, y}) {
                has_element = true;
                break;
            }
        }

        if !has_element {
            cols.push(x);
        }
    }

    for galaxy in galaxies {
        let mut new_location = galaxy.clone();

        new_location.x += (expand_size-1)*cols.iter().filter(|&&x| x < new_location.x).count() as isize;
        new_location.y += (expand_size-1)*rows.iter().filter(|&&y| y < new_location.y).count() as isize;

        results.insert(new_location);
    }

    (results, Location{x: max_locations.x+cols.len() as isize, y: max_locations.y+rows.len() as isize})
}

fn shorter_distance(from: &Location, to: &Location) -> isize {
    isize::abs(from.x - to.x) + isize::abs(from.y - to.y)
}

fn get_sum_short_distance(galaxies: &HashSet<Location>) -> isize {
    let mut result = 0;
    let mut couples : HashMap<(Location, Location), isize> = HashMap::new();

    for &from in galaxies {
        for &to in galaxies {
            if couples.contains_key(&(from, to)) || couples.contains_key(&(to, from)) {
                continue;
            }

            if from == to {
                continue;
            }

            let dist = shorter_distance(&from, &to);

            // println!("{:?} -> {:?}: {}", from, to, dist);

            couples.insert((from, to), dist);

            result += dist;
        }
    }

    result
}

fn draw(galaxies: &HashSet<Location>, max_locations: &Location) {
    for y in 0..max_locations.y+1 {
        for x in 0..max_locations.x+1 {
            if galaxies.contains(&Location{x, y}) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!()
    }
}

fn main() {
    let (galaxies, max_locations) = readfile("input.txt");
    let (galaxies_exp1, _) = expand(&galaxies, &max_locations, 2);
    println!("#1 {}", get_sum_short_distance(&galaxies_exp1)); // 9418609

    let (galaxies_exp2, _) = expand(&galaxies, &max_locations, 1000000);
    println!("#2 {}", get_sum_short_distance(&galaxies_exp2)); // 593821230983    
}

#[test]
fn test() {
    let (galaxies, max_locations) = readfile("input_test.txt");
    let (galaxies1, _) = expand(&galaxies, &max_locations, 2);
    assert_eq!(374, get_sum_short_distance(&galaxies1));

    let (galaxies10, _) = expand(&galaxies, &max_locations, 10);
    assert_eq!(1030, get_sum_short_distance(&galaxies10));

    let (galaxies100, _) = expand(&galaxies, &max_locations, 100);
    assert_eq!(8410, get_sum_short_distance(&galaxies100));

}