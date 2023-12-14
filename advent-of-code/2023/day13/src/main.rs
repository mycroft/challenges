use std::fs;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Location(isize, isize);

fn get_size(grid: &HashSet<Location>) -> (isize, isize) {
    let mut max_x = 0;
    let mut max_y = 0;

    for item in grid {
        if item.0 > max_x {
            max_x = item.0;
        }
        if item.1 > max_y {
            max_y = item.1;
        }
    }

    (max_x+1, max_y+1)
}

fn find_mirror_x(grid: &HashSet<Location>, offset: usize) -> isize {
    let (size_x, _size_y) = get_size(grid);

    for x in 0..size_x-1 {
        let mut other_side = HashSet::new();
        let mut mirrored = HashSet::new();

        for item in grid {
            if item.0 <= x {
                let other = Location(x+(1+x-item.0), item.1);
                if other.0 >= size_x {
                    continue;
                }
                other_side.insert(other);
            }
        }

        // x = 0, 1, 2   x == 2
        // o = 3, 4, 5   o >= 3

        for item in grid {
            if item.0 > x && item.0 < (x+1)*2 {
                mirrored.insert(*item);
            }
        }

        // println!("x:{} grid:{} other:{} mirrored:{}", x, grid.len(), other_side.len(), mirrored.len());
        // println!("{:?}", mirrored);

        let intersect: HashSet<&Location> = other_side.intersection(&mirrored).collect();

        if offset > 0 {
            if other_side.len() == intersect.len() && mirrored.len() == intersect.len() + offset {
                return x + 1;
            }

            if other_side.len() == intersect.len() + offset && mirrored.len() == intersect.len() {
                return x + 1;
            }
        } else if other_side.len() == intersect.len() && mirrored.len() == intersect.len() {
            return x + 1;
        }
    }

    -1
}

fn find_mirror_y(grid: &HashSet<Location>, offset: usize) -> isize {
    let (_size_x, size_y) = get_size(grid);

    for y in 0..size_y-1 {
        let mut other_side = HashSet::new();
        let mut mirrored = HashSet::new();

        for item in grid {
            if item.1 <= y {
                let other = Location(item.0, y+(1+y-item.1));
                if other.1 >= size_y {
                    continue;
                }
                other_side.insert(other);
            }
        }

        for item in grid {
            if item.1 > y && item.1 < (y+1)*2 {
                mirrored.insert(*item);
            }
        }

        // println!("x:{} grid:{} other:{} mirrored:{}", y, grid.len(), other_side.len(), mirrored.len());
        // println!("{:?}", mirrored);

        let intersect: HashSet<&Location> = other_side.intersection(&mirrored).collect();

        if offset > 0 {
            if other_side.len() == intersect.len() && mirrored.len() == intersect.len() + offset {
                return y + 1;
            }

            if other_side.len() == intersect.len() + offset && mirrored.len() == intersect.len() {
                return y + 1;
            }
        } else if other_side.len() == intersect.len() && mirrored.len() == intersect.len() {
            return y + 1;
        }

        // println!("{} {} {}", y, other_side.len(), intersect.len());
    }

    -1
}


fn main() {
    let contents = fs::read_to_string("input.txt").expect("a file to open");
    let lines = contents.lines();

    let mut current_y = 0;
    let mut current_grid = HashSet::new();
    let mut all_grids = Vec::new();

    for line in lines {
        if line.is_empty() {
            current_y = 0;
            all_grids.push(current_grid);
            current_grid = HashSet::new();
            continue;
        }

        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                current_grid.insert(Location(x as isize, current_y));
            }
        }

        current_y += 1;
    }

    all_grids.push(current_grid);

    let mut p1 = 0;
    let mut p2 = 0;

    for grid in all_grids {
        let r0 = find_mirror_x(&grid, 0);
        if r0 != -1 {
            p1 += r0;
        }

        let r1 = find_mirror_y(&grid, 0);
        if r1 != -1 {
            p1 += 100*r1;
        }

        let r2 = find_mirror_x(&grid, 1);
        if r2 != -1 {
            println!("{}", r2);
            p2 += r2;
        }

        let r3 = find_mirror_y(&grid, 1);
        if r3 != -1 {
            println!("{}", r3);
            p2 += 100*r3;
        }
    }

    println!("#1 {}", p1);
    println!("#2 {}", p2);

}

fn draw(grid: &HashSet<Location>) {
    let (size_x, size_y) = get_size(grid);

    for y in 0..size_y {
        for x in 0..size_x {
            if grid.contains(&Location(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}