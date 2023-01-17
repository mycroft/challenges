use std::fs::read_to_string;
use std::collections::{BTreeSet, VecDeque};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct Position(isize, isize);

fn parse(fp: &str) -> (BTreeSet<Position>, (isize, isize)) {
    let mut map = BTreeSet::new();
    let contents = read_to_string(fp).unwrap();
    let mut max_x: isize = 0;
    let mut max_y: isize = 0;

    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                map.insert(Position(x as isize, y as isize));
            }

            max_x = x as isize;
        }

        max_y = y as isize;
    }

    (map, (max_x + 1, max_y + 1))
}

fn play_step1(map: &BTreeSet<Position>, size: (isize, isize)) -> BTreeSet<Position> {
    let mut new_map = BTreeSet::new();

    let dirs = [(-1isize, 0isize), (1isize, 0isize), (0isize, -1isize), (0isize, 1isize)];

    for y in 0..size.1 {
        for x in 0..size.0 {
            let adj = dirs.iter().filter(|&d| map.contains(&Position(x + d.0, y + d.1))).count();
            let mut has_bug = map.contains(&Position(x as isize, y as isize));

            if has_bug && adj != 1 {
                has_bug = false;
            } else if !has_bug && (adj == 1 || adj == 2) {
                has_bug = true
            }

            if has_bug {
                new_map.insert(Position(x, y));
            }
        }
    }

    new_map
}

fn display(map: &BTreeSet<Position>, size: (isize, isize)) {
    for y in 0..size.1 {
        for x in 0..size.0 {
            if map.contains(&Position(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn diversity(map: &BTreeSet<Position>, size: (isize, isize)) -> usize {
    let mut score = 0;
    let mut pt = 1;

    for y in 0..size.1 {
        for x in 0..size.0 {
            if map.contains(&Position(x, y)) {
                score += pt;
            }

            pt *= 2;
        }
    }

    score
}

fn find_adjacent(above: &BTreeSet<Position>, same: &BTreeSet<Position>, bellow: &BTreeSet<Position>, pos: &Position) -> usize {
    let dirs = [(-1isize, 0isize), (1isize, 0isize), (0isize, -1isize), (0isize, 1isize)];
    let mut adj = 0;

    for dir in &dirs {
        let np = Position(pos.0 + dir.0, pos.1 + dir.1);

        // easy part: check bugs in the same layer.
        if same.contains(&np) {
            adj += 1;
        }

        // not easy but not that hard. We go outer the current matrix and we pick one value
        if np.0 < 0 && above.contains(&Position(1, 2)) { // 12
            adj += 1;
        }

        if np.1 < 0 && above.contains(&Position(2, 1)) { // 8
            adj += 1;
        }

        if np.0 > 4 && above.contains(&Position(3, 2)) { // 14
            adj += 1;
        }

        if np.1 > 4 && above.contains(&Position(2, 3)) { // 18
            adj += 1;
        }
    }

    // hard part: we are in the inner square and we need to check for 5 other values.
    if pos == &Position(1, 2) { // 12
        // we need the bellow left column
        adj += bellow.iter().filter(|pos| pos.0 == 0).count();
    }

    if pos == &Position(2, 1) { // 8
        // need the bellow up column
        adj += bellow.iter().filter(|pos| pos.1 == 0).count();
    }

    if pos == &Position(3, 2) { // 14
        // bellow right
        adj += bellow.iter().filter(|pos| pos.0 == 4).count();
    }

    if pos == &Position(2, 3) { // 18
        // bellow down
        adj += bellow.iter().filter(|pos| pos.1 == 4).count();
    }

    // to write
    adj
}

fn play_step2(layers: &VecDeque<BTreeSet<Position>>, size: (isize, isize)) -> VecDeque<BTreeSet<Position>> {
    // We will build a new layer, fill it, then go from existing layer 0 to n, build a new layer, fill it, return everything.
    let mut new_layers = VecDeque::new();


    for new_layer_num in 0..layers.len() + 2 {        
        let mut new_layer = BTreeSet::new();

        // find out which is the above layer to use.
        let mut current_above_layer = &BTreeSet::new();
        if new_layer_num > 1 {
            current_above_layer = &layers[new_layer_num - 2];
        }

        let mut current_bellow_layer = &BTreeSet::new();
        if new_layer_num < layers.len() {
            current_bellow_layer = &layers[new_layer_num];
        }

        let mut current_same_layer = &BTreeSet::new();
        if new_layer_num > 0 && new_layer_num < layers.len() + 1 {
            current_same_layer = &layers[new_layer_num - 1];
        }

        for y in 0..size.1 {
            for x in 0..size.0 {
                // '?' case. We do not do anything (yet).
                if x == 2 && y == 2 {
                    continue;
                }

                // for each dirs, check for adjacent... with the exception to go upper/bellow.
                let adj = find_adjacent(current_above_layer, current_same_layer, current_bellow_layer, &Position(x, y));
                let mut has_bug = current_same_layer.contains(&Position(x as isize, y as isize));

                if has_bug && adj != 1 {
                    has_bug = false;
                } else if !has_bug && (adj == 1 || adj == 2) {
                    has_bug = true
                }

                if has_bug {
                    new_layer.insert(Position(x, y));
                }
            }
        }
        
        new_layers.push_back(new_layer);
    }
    
    new_layers
}

fn main() {
    let mut layouts: BTreeSet<BTreeSet<Position>> = BTreeSet::new();
    let (mut map, size) = parse("input.txt");

    loop {
        if layouts.contains(&map) {
            break;
        }
        layouts.insert(map.clone());
        map = play_step1(&map, size);
    }

    // display(&map, size);
    println!("#1 {}", diversity(&map, size)); // 28772955

    // step 2

    let (map, size) = parse("input.txt");

    let mut layers: VecDeque<BTreeSet<Position>> = VecDeque::new();
    layers.push_back(map);

    for _ in 0..200 {
        // compute layers
        layers = play_step2(&layers, size);
    }

    let mut bugs = 0;

    for (_, layer) in layers.iter().enumerate() {
        bugs += layer.len();
    }

    println!("#2 {}", bugs); // 2023
}
