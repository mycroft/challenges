use std::fs;
use std::collections::BTreeSet;

use regex::Regex;

#[derive(Debug)]
enum Action {
    SCAN,
    FALL,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Ex {
    clay: BTreeSet<Pos>,
    still: BTreeSet<Pos>,
    flowing: BTreeSet<Pos>,
    queue: Vec<(Action, Pos)>,
    x0: usize,
    x1: usize,
    y0: usize,
    y1: usize,
}

impl Ex {
    fn count_all(&self) -> usize {
        self.still.union(&self.flowing).filter(|p| p.y >= self.y0 && p.y <= self.y1).count()
    }

    fn count_still(&self) -> usize {
        self.still.iter().filter(|p| p.x >= self.y0 && p.y <= self.y1).count()
    }

    fn stop(&self, p: Pos) -> bool {
        self.clay.contains(&p)
    }

    fn pile(&self, p: Pos) -> bool {
        self.clay.contains(&p) || self.still.contains(&p)
    }

    fn fall(&mut self, mut p: Pos) {
        while p.y <= self.y1 && !self.pile(Pos{x: p.x, y: p.y + 1}) {
            self.flowing.insert(p);
            p.y += 1;
        }

        if p.y <= self.y1 {
            self.flowing.insert(p);
            self.queue.push((Action::SCAN, p));
        }
    }

    fn scan(&mut self, p: Pos) {
        let mut x0 = p.x;
        while self.pile(Pos{x: x0, y: p.y + 1}) && !self.stop(Pos{x: x0 - 1, y: p.y}) {
            x0 -= 1;
        }
/*
        println!("> {:?} {:?} {:?}", Pos{x: x0 -1, y: p.y}, 
            self.pile(Pos{x: x0, y: p.y + 1}),
            self.stop(Pos{x: x0 - 1, y: p.y}));
*/
        let mut x1 = p.x;
        while self.pile(Pos{x: x1, y: p.y + 1}) && !self.stop(Pos{x: x1 + 1, y: p.y}) {
            x1 += 1;
        }

        let stop0 = self.stop(Pos{x: x0 - 1, y: p.y});
        let stop1 = self.stop(Pos{x: x1 + 1, y: p.y});

        if stop0 && stop1 {
            for i in x0..x1 + 1 {
                self.still.insert(Pos{x: i, y: p.y});
            }
            self.queue.push((Action::SCAN, Pos{x: p.x, y: p.y - 1}));
        } else {
            for i in x0..x1 + 1 {
                self.flowing.insert(Pos{x: i, y: p.y});
            }
            if !stop0 {
                self.queue.push((Action::FALL, Pos{x: x0, y: p.y}));
            }
            if !stop1 {
                self.queue.push((Action::FALL, Pos{x: x1, y: p.y}));
            }
        }

    }

    fn run(&mut self, p: Pos) {
        self.queue.push((Action::FALL, p));

        while !self.queue.is_empty() {
            let el = self.queue.pop().unwrap();

            // println!("{:?}", el);

            match el.0 {
                Action::FALL => self.fall(el.1),
                Action::SCAN => self.scan(el.1)
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();

    let r = Regex::new(r"^(.)=(\d+), (.)=(\d+)..(\d+)$").unwrap();

    let mut ex = Ex {
        clay: BTreeSet::new(),
        still: BTreeSet::new(),
        flowing: BTreeSet::new(),
        queue: Vec::new(),
        x0: 0,
        x1: 0,
        y0: 0,
        y1: 0,
    };

    for line in lines {
        let c = r.captures(line).unwrap();

        let l0 = c.get(1).unwrap().as_str();
        let d0 = c.get(2).unwrap().as_str().parse::<usize>().unwrap();

        let _l1 = c.get(3).unwrap();
        let d1 = c.get(4).unwrap().as_str().parse::<usize>().unwrap();
        let d2 = c.get(5).unwrap().as_str().parse::<usize>().unwrap();

        for i in d1..=d2 {
            if l0 == "x" {
                ex.clay.insert(Pos { x: d0, y: i });
            } else {
                ex.clay.insert(Pos { x: i, y: d0 });
            }
        }
    }

    ex.x0 = ex.clay.iter().fold(1000, |v, p| std::cmp::min(v, p.x));
    ex.x1 = ex.clay.iter().fold(0, |v, p| std::cmp::max(v, p.x));
    ex.y0 = ex.clay.iter().fold(1000, |v, p| std::cmp::min(v, p.y));
    ex.y1 = ex.clay.iter().fold(0, |v, p| std::cmp::max(v, p.y));

    ex.run(Pos{x: 500, y: 0});

    println!("Part #1: {}", ex.count_all());
    println!("Part #2: {}", ex.count_still());
}
