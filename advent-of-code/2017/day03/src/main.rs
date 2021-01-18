use std::ops::AddAssign;

fn solve1(seek: u32) -> u32 {
    let mut current_val = 1;
    let mut current_step = 2;
    let mut remaining_step = 4;

    if seek == 1 {
        return 0;
    }

    loop {
        current_val += current_step;
        remaining_step -= 1;

        if seek >= current_val - current_step/2 && current_val + current_step/2 >= seek {
            // current required step is current_step
            /*
            println!("current_step:{} seek:{} current_val:{}",
                current_step,
                seek,
                current_val
            );
            */

            return if seek > current_val {
                current_step - (seek - current_val)
            } else {
                current_step - (current_val - seek)
            };
        }

        if remaining_step == 0 {
            remaining_step = 4;
            current_step += 2;
        }
    }
}

#[derive(Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl AddAssign for Pos {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

// Also see: https://oeis.org/A141481
fn solve2(seek: u32) -> u32 {
    let mut state = [[0u32; 20]; 20];

    let mut p = Pos { x: 10, y: 10 };
    let mut direction = Pos { x: 1, y: 0 };

    let mut min = Pos { x: 10, y: 10 };
    let mut max = Pos { x: 10, y: 10 };

    state[p.x as usize][p.y as usize] = 1;

    loop {
        let mut sum : u32 = 0;

        p += direction;

        if p.x > max.x {
            direction = Pos { x: 0, y: -1 };
            max.x = p.x;
        }
        else if p.y < min.y {
            direction = Pos { x: -1, y: 0 };
            min.y = p.y;
        }
        else if p.x < min.x {
            direction = Pos { x: 0, y: 1 };
            min.x = p.x;
        }
        else if p.y > max.y {
            direction = Pos { x: 1, y: 0 };
            max.y = p.y;
        }

        for i in p.x-1..p.x+2 {
            for j in p.y-1..p.y+2 {
                sum += state[i as usize][j as usize];
            }
        }

        state[p.x as usize][p.y as usize] = sum;

        if sum > seek {
            return sum;
        }
    }
}

fn main() {
    let seek = 325489;
    // let seek = 22;

    println!("Part #1: {}", solve1(seek));
    println!("Part #2: {}", solve2(seek));
}

#[test]
fn example() {
    assert_eq!(0, solve1(1));
    assert_eq!(2, solve1(11));
    assert_eq!(3, solve1(12));
    assert_eq!(4, solve1(13));
    assert_eq!(2, solve1(23));
    assert_eq!(4, solve1(21));
    assert_eq!(3, solve1(22));
    assert_eq!(31, solve1(1024));
}