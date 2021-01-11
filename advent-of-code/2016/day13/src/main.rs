use pathfinding::prelude::astar;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

fn is_space(p: &Pos) -> bool {
    let x = p.0;
    let y = p.1;
    let val = x*x + 3*x + 2*x*y + y + y*y + 1362;

    val.count_ones() % 2 == 0
}

impl Pos {
    fn successors(&self) -> Vec<(Pos, u32)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            .map(|(x, y)| (Pos(self.0 + x, self.1 + y), 1))
            .filter(|(x, _)| is_space(&x))
            .collect::<Vec<(Pos, u32)>>()
    }

    fn distance(&self, _p: &Pos) -> u32 {
        42
    }
}

fn main() {
    let goal = Pos(31, 39);

    let _res = astar(
        &Pos(1, 1),
        |p| p.successors(),
        |p| p.distance(&goal),
        |p| *p == goal
    );

    println!("Part #1: {:?}", _res.unwrap().1);

    let mut count = 0;

    for i in 0..51 {
        for j in 0..51 {
            if !is_space(&Pos(i, j)) {
                continue;
            }

            let goal = Pos(i, j);
            match astar(&Pos(1, 1), |p| p.successors(), |p| p.distance(&goal), |p| *p == goal) {
                Some(val) => {
                    if val.1 <= 50 {
                        count += 1;
                    }
                },
                None => {
                }
            }
        }
    }

    println!("Part #2: {:?}", count);
}

