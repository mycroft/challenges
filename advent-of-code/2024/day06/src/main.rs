use std::fs;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    fn move_pos(&self, direction: Direction) -> Pos {
        let (dx, dy) = direction.move_index();
        Pos {
            x: self.x + dx,
            y: self.y + dy,
        }
    }

    fn is_valid(&self, max_position: Pos) -> bool {
        self.x >= 0 && self.y >= 0 && self.x <= max_position.x && self.y <= max_position.y
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn move_index(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Cursor {
    pos: Pos,
    direction: Direction,
}

fn read_input(fp: &str) -> (Pos, Direction, HashSet<Pos>, Pos) {
    let mut pos = Pos { x: 0, y: 0 };
    let mut set = HashSet::new();
    let mut max_x = 0;
    let mut max_y = 0;
    let mut current_direction = Direction::Up;

    let contents = fs::read_to_string(fp).expect("Error reading the file");

    for (y, line) in contents.lines().enumerate() {
        max_y = y;
        max_x = line.len();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                set.insert(Pos { x: x as isize, y: y as isize });
            } else if c != '.' {
                current_direction = match c {
                    '^' => Direction::Up,
                    'v' => Direction::Down,
                    '>' => Direction::Right,
                    '<' => Direction::Left,
                    _ => unreachable!(),
                };
                pos = Pos { x: x as isize, y: y as isize };
            }
        }
    }

    (pos, current_direction, set, Pos{ x: max_x as isize, y: max_y as isize})
}

fn move_pos(current: Pos, current_direction: Direction, blockers: &HashSet<Pos>, max_position: Pos) -> (Option<Pos>, Option<Direction>) {
    let next_pos = current.move_pos(current_direction);

    if next_pos.is_valid(max_position) && !blockers.contains(&next_pos) {
        return (Some(next_pos), Some(current_direction));
    }

    if blockers.contains(&next_pos) {
        return (Some(current), Some(current_direction.next()));
    }

    (None, None)
}

fn run(position: Pos, direction: Direction, blockers: &HashSet<Pos>, max_position: Pos) -> Option<HashSet<Pos>> {
    let mut current_pos = position;
    let mut current_direction = direction;
    let mut visited = HashSet::new();
    let mut cursors: HashSet<Cursor> = HashSet::new();

    visited.insert(current_pos);

    loop {
        let (next_pos, next_direction) = move_pos(current_pos, current_direction, blockers, max_position);

        match next_pos {
            Some(pos) => {
                let cursor = Cursor { pos, direction: next_direction.unwrap() };

                if cursors.contains(&cursor) {
                    // println!("Loop detected at {:?}", cursor);
                    return None;
                }

                //println!("Moving to {:?}", cursor);

                cursors.insert(cursor);

                visited.insert(pos);
                current_pos = cursor.pos;
                current_direction = cursor.direction;
            },
            None => break,
        }
    }

    Some(visited)
}

fn solve_part1(position: Pos, direction: Direction, blockers: &HashSet<Pos>, max_position: Pos) -> usize {
    run(position, direction, blockers, max_position).unwrap().len()
}


fn solve_part2(position: Pos, direction: Direction, blockers: &HashSet<Pos>, max_position: Pos) -> usize {
    let mut result = 0;

    // first, get all visited, like solve_part1
    let visited = run(position, direction, blockers, max_position).unwrap();

    // then, for each visited, add a block, try to find if we can exit or not.
    for visited_el in visited {
        // we skip initial position
        if visited_el == position {
            continue;
        }

        let mut new_blockers = blockers.clone();
        new_blockers.insert(visited_el);

        // println!("Trying with element {:?} {:?}", position, visited_el);

        if run(position, direction, &new_blockers, max_position).is_none() {
            result += 1;
        }
    }

    result
}

fn main() {
    let (position, current_direction, blockers, max_position) = read_input("input.txt");

    let result_step1 = solve_part1(position, current_direction, &blockers, max_position);
    let result_step2 = solve_part2(position, current_direction, &blockers, max_position);
    println!("#1: {}", result_step1); // 4665
    println!("#2: {}", result_step2);

}
