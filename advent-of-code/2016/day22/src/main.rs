use std::fs;
use regex::Regex;
use std::collections::HashMap;

#[derive(Debug,Clone,PartialEq)]
struct Node {
    used: usize,
    avail: usize,
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();

    let re = Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T").unwrap();

    let mut nodes = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for line in lines {
        if !re.is_match(line) {
            continue;
        }

        let captures = re.captures(line).unwrap();

        let x = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let y = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();

        let _total = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let used = captures.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let avail = captures.get(5).unwrap().as_str().parse::<usize>().unwrap();

        // println!("x:{:?} y:{:?} total:{:?} used:{:?} avail:{:?}", x, y, _total, used, avail);

        nodes.insert((x, y), Node { used: used, avail: avail });

        if x > max_x { max_x = x };
        if y > max_y { max_y = y };
    }

    let mut viables = 0;

    for n1 in &nodes {
        for n2 in &nodes {
            if n1 == n2 || n1.1.used == 0 || n1.1.used > n2.1.avail {
                continue;
            }

            viables += 1;
        }
    }

    println!("Part #1: {}", viables);
}

/*
Part 2 is done by hand:

OT is at x:17 y:22
there is a wall from x:1 y:21 to x:35 y:21

so, we have to:
move from x:17 to x:0 (17 moves)
move from y:22 to y:0 (22 moves)
move from x:0 to x:35 (35 moves)
repeat:
move back 0T from x:35 y:0 to x:33 y:0 but without moving x:34 y:0 (4 moves)
move x:33 y:0 to x:34 y:0 (1 move)


So: 17 + 2é + 35 + 5*34 = 224 moves.
*/