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

/*
    # count_all returns how many in-bounds cells have still or flowing water
    def count_all(self):
        return sum(1 for x, y in self.still | self.flowing
            if y >= self.y0 and y <= self.y1)

    # count_still returns how many in-bounds cells have still water
    def count_still(self):
        return sum(1 for x, y in self.still
            if y >= self.y0 and y <= self.y1)

    # stop returns true if (x, y) has clay (horizontal scan stops here)
    def stop(self, x, y):
        return (x, y) in self.clay

    # pile returns true if (x, y) is clay or still water (water can pile up here)
    def pile(self, x, y):
        return (x, y) in self.clay or (x, y) in self.still

    # fall scans downward until it hits clay or still water
    def fall(self, x, y):
        while y <= self.y1 and not self.pile(x, y + 1):
            self.flowing.add((x, y))
            y += 1
        if y <= self.y1:
            self.flowing.add((x, y))
            self.queue.append((self.scan, x, y))

*/

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

/*
    # scan looks left and right until it hits clay or falls off an edge
    def scan(self, x, y):
        x0 = x
        while self.pile(x0, y + 1) and not self.stop(x0 - 1, y):
            x0 -= 1
        x1 = x
        while self.pile(x1, y + 1) and not self.stop(x1 + 1, y):
            x1 += 1
        stop0 = self.stop(x0 - 1, y)
        stop1 = self.stop(x1 + 1, y)
        if stop0 and stop1:
            for i in range(x0, x1 + 1):
                self.still.add((i, y))
            self.queue.append((self.scan, x, y - 1))
        else:
            for i in range(x0, x1 + 1):
                self.flowing.add((i, y))
            if not stop0:
                self.queue.append((self.fall, x0, y))
            if not stop1:
                self.queue.append((self.fall, x1, y))
*/

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

    // println!("{:?}", ex);

    ex.run(Pos{x: 500, y: 0});

    println!("Part #1: {}", ex.count_all());
    println!("Part #2: {}", ex.count_still());
}
