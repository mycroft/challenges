use std::cmp::{max, min};
use std::fmt;

use pathfinding::prelude::astar;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct State {
    hallway: [Option<char>; 11],
    rooms: [Vec<Option<char>>; 4], // exit at 2, 4, 6, 8
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "hallway:{} rooms:{:?}",
            self.hallway.iter().map(|&x| if let Some(c) = x { c } else { '.' }).collect::<String>(),
            self.rooms.iter().map(
                |r| r.iter().map(|&c| {
                    if let Some(c) = c { c } else { '.' }
                }).collect::<String>()
            ).collect::<Vec<String>>()
        )
    }
}

impl State {
    // Rules to get successors of a state:
    // - They will never step on hallway's 2, 4, 6 or 8.
    // - They will never move into a room that is not their or that has an entity not in correct place
    // - Once out, they will not move unless they can get into their room.
    fn successors(&self) -> Vec<(State, u32)> {
        let mut res = vec![];

        // First, get the hallway letters thay could eventually go into their space.
        for (idx, l) in self.hallway.iter().enumerate() {
            if l.is_none() {
                continue;
            }

            let moves = self.hallway_amphibot_can_move(idx);
            if moves.is_none() {
                continue;
            }

            let moves = moves.unwrap();

            let new_state = self.get_new_state(
                &(
                    self.hallway[idx].unwrap(),
                    (idx, 0),
                    moves.0,
                    moves.1,
                )
            );

            res.push((new_state, moves.1 as u32));
        }

        // The, for each room we check if we can move or not
        for idx in vec![2usize, 4, 6, 8] {
            let moves = self.room_amphibot_can_move(idx);

            for current_move in &moves {
                let new_state = self.get_new_state(current_move);

                res.push((new_state, current_move.3 as u32));
            }
        }

        res
    }

    // Compute new state using the move given.
    fn get_new_state(&self, current_move: &(char, (usize, usize), (usize, usize), usize)) -> State {
        let mut new_state = self.clone();

        let letter = current_move.0;
        let from = current_move.1;
        let to = current_move.2;

        // remove char from "from"
        if from.1 == 0 {
            new_state.hallway[from.0] = None;
        } else {
            new_state.rooms[from.0 / 2 - 1][from.1 - 1] = None;
        }

        // add char to "to"
        if to.1 == 0 {
            new_state.hallway[to.0] = Some(letter);
        } else {
            new_state.rooms[to.0 / 2 - 1][to.1 - 1] = Some(letter);
        }

        new_state
    }

    // Check if an amphipod from any of the room can go somewhere
    // from is the room number (2, 4, 6 or 8)
    // returns all the possibles moves from this room to
    // Return format: (letter, (from_x, from_y), (to_x, to_y), moves)
    fn room_amphibot_can_move(&self, from: usize) -> Vec<(char, (usize, usize), (usize, usize), usize)> {
        let mut res = vec![];

        let expected_letter = match from {
            2 => 'A',
            4 => 'B',
            6 => 'C',
            8 => 'D',
            _ => unreachable!()
        };

        let mut must_move = false;

        // First, we check if any amphipod needs to move
        for y in 0..self.rooms[0].len() {
            let y = self.rooms[0].len() - y - 1;

            if let Some(letter) = self.get_letter_from_position((from, y+1)) {
                if letter != expected_letter {
                    must_move = true;
                }
            }
        }

        if !must_move {
            return res;
        }

        let mut letter : char = '0';
        let mut from = (from, 0);

        // If we need to move, find first letter to move.
        for y in 0..self.rooms[0].len() {
            if let None = self.get_letter_from_position((from.0, y+1)) {
                continue;
            }

            letter = self.get_letter_from_position((from.0, y+1)).unwrap();
            from = (from.0, y + 1);

            break;
        }

        // We moves letter "letter" from "from" to anywhere possible
        for idx in 0..self.hallway.len() {
            let to = (idx, 0);
            let current_move = self.can_move_from_to(from, to);
            if current_move.is_none() {
                continue;
            }

            if idx == 2 || idx == 4 || idx == 6 || idx == 8 {
                continue;
            }

            res.push(
                (letter, from, to, current_move.unwrap())
            );
        }

        // Then try to go into target.
        let target = self.target_room_for_letter(letter);
        if target.is_none() {
            return res;
        }

        let move_to_target = self.can_move_from_to(from, target.unwrap());
        if let Some(v) =  move_to_target {
            res.push(
                (letter, from, target.unwrap(), v)
            );
        }
        
        res
    }

    // Check if an amphipod from the hallway can go somewhere
    // Returns number of moves
    fn hallway_amphibot_can_move(&self, from: usize) -> Option<((usize, usize), usize)> {
        let c = self.hallway[from];
        if c.is_none() {
            return None;
        }

        let target = self.target_room_for_letter(c.unwrap());
        if target.is_none() {
            return None;
        }

        let from = (from, 0);
        let target = target.unwrap();

        let can_move_result = self.can_move_from_to(from, target);
        if can_move_result.is_none() {
            return None;
        }

        Some(
            (target, can_move_result.unwrap())
        )
    }

    // Check if an amphipod can go from a location to another
    // Returns the move number.
    // from.0 is x, from 0 to 11
    // from.1 == 0 if hallway, 1 if hallway side room, 2 if inner room
    fn can_move_from_to(&self, from: (usize, usize), to: (usize, usize)) -> Option<usize> {
        // If amphipod can move, it will be only in two ways:
        // right or left, then up or down

        // checking horizontally
        for x in min(from.0, to.0)..=max(from.0, to.0) {
            let y = 0;

            // either or origin or destination
            if from == (x, y) /* || to == (x, y) */ {
                continue;
            }

            if self.hallway[x].is_some() {
                // there is something on the way
                return None;
            }
        }

        // We moved <--->
        let mut moves = ((to.0 as i8 - from.0 as i8)).abs() as usize;

        // Next, moving vertically.

        // First, check if letter can go there.
        if to.1 != 0 {
            let letter = self.get_letter_from_position(from);
            if letter.is_none() {
                unreachable!();
            }

            let target = self.target_room_for_letter(letter.unwrap());
            if target.is_none() {
                unreachable!();
            }

            if to != target.unwrap() {
                return None;
            }
        }

        // We might leave a room to go to another room
        // don't forget from.1 is always +1 bigger than array position.
        if from.1 != 0 {
            let mut y = from.1 - 1;
            while y > 0 {
                if self.rooms[from.0 / 2 - 1][y - 1].is_some() {
                    return None;
                } else {
                    moves += 1;
                }

                y -= 1;
            }

            moves += 1;
        }

        // We enter a room.
        if to.1 != 0 {
            let mut y = 0;

            while y != to.1 {
                if self.rooms[to.0 / 2 - 1][y].is_some() {
                    return None;
                } else {
                    moves += 1;
                }

                y += 1;
            }
        }

        // Retrieve target move price
        let letter_price = match self.get_letter_from_position(from).unwrap() {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
            _ => unreachable!(),
        };

        Some(moves * letter_price)
    }

    // Returns room indice & depth
    // hallway's side room is depth 1, inner's: 2, so that can be reused in can_move_from_to.
    fn target_room_for_letter(&self, c: char) -> Option<(usize, usize)>{
        let room_number = dest_room_for_letter(c);

        // We check first if we have some other letter before the first free room.
        for y in 0..self.rooms[0].len() {
            let y = self.rooms[0].len() - y - 1;

            let res = self.rooms[room_number / 2 - 1][y];
            if res.is_none() {
                return Some((room_number, y+1));
            }

            if let Some(found) = res {
                if found != c {
                    return None;
                }
            }
        }

        None
    }

    // Retrieve letter for position
    fn get_letter_from_position(&self, from: (usize, usize)) -> Option<char> {
        if from.1 == 0 {
            return self.hallway[from.0];
        }

        self.rooms[from.0 / 2 - 1][from.1  - 1]
    }
}

fn dest_room_for_letter(c: char) -> usize {
    match c {
        'A' => 2,
        'B' => 4,
        'C' => 6,
        'D' => 8,
        _ => unreachable!()
    }
}

fn play(init_state: State, expected_state: State) -> usize {
    let res = astar(
        &init_state,
        |p| p.successors(),
        |_p| 0,
        |p| *p == expected_state,
    ).expect("value");

    res.1 as usize
}

fn get_expected_state(init_state: &State) -> State {
    State {
        hallway: [None; 11],
        rooms: [
            vec![Some('A'); init_state.rooms[0].len()],
            vec![Some('B'); init_state.rooms[0].len()],
            vec![Some('C'); init_state.rooms[0].len()],
            vec![Some('D'); init_state.rooms[0].len()],
        ]
    }
}

fn main() {
    let init_state = State {
        hallway: [None; 11],
        rooms: [
            vec![Some('D'), Some('C')],
            vec![Some('A'), Some('A')],
            vec![Some('C'), Some('B')],
            vec![Some('D'), Some('B')],
        ]
    };

    let expected_state = get_expected_state(&init_state);

    println!("#1: {:?}", play(init_state, expected_state)); // 14346

    let init_state = State {
        hallway: [None; 11],
        rooms: [
            vec![Some('D'), Some('D'), Some('D'), Some('C')],
            vec![Some('A'), Some('C'), Some('B'), Some('A')],
            vec![Some('C'), Some('B'), Some('A'), Some('B')],
            vec![Some('D'), Some('A'), Some('C'), Some('B')],
        ]
    };

    let expected_state = get_expected_state(&init_state);

    println!("#2: {:?}", play(init_state, expected_state)); // 48984
}

#[test]
fn test_full() {
    let init_state = State {
        hallway: [None; 11],
        rooms: [
            vec![Some('B'), Some('A')],
            vec![Some('C'), Some('D')],
            vec![Some('B'), Some('C')],
            vec![Some('D'), Some('A')],
        ]
    };

    let expected_state = get_expected_state(&init_state);
    assert_eq!(12521, play(init_state, expected_state));
}

#[test]
fn test_full2() {
    let init_state = State {
        hallway: [None; 11],
        rooms: [
            vec![Some('B'), Some('D'), Some('D'), Some('A')],
            vec![Some('C'), Some('C'), Some('B'), Some('D')],
            vec![Some('B'), Some('B'), Some('A'), Some('C')],
            vec![Some('D'), Some('A'), Some('C'), Some('A')],
        ]
    };

    let expected_state = get_expected_state(&init_state);
    assert_eq!(44169, play(init_state, expected_state));
}

#[test]
fn test_successors_0() {
    let init_state = State {
        hallway: [None; 11],
        rooms: [
            vec![Some('B'), Some('A')],
            vec![Some('C'), Some('D')],
            vec![Some('B'), Some('C')],
            vec![Some('D'), Some('A')],
        ]
    };

    let successors = init_state.successors();
    assert_eq!(28, successors.len());
}

// Controlled that C was meeting its right place.
#[test]
fn test_successors_1() {
    let init_state = State {
        hallway: [None, None, None, Some('B'), None, None, None, None, None, None, None],
        rooms: [
            vec![Some('B'), Some('A')],
            vec![Some('C'), Some('D')],
            vec![None, Some('C')],
            vec![Some('D'), Some('A')],
        ]
    };

    let successors = init_state.successors();
    assert_eq!(11, successors.len());
}

// C just moved
#[test]
fn test_successors_2() {
    let init_state = State {
        hallway: [None, None, None, Some('B'), None, None, None, None, None, None, None],
        rooms: [
            vec![Some('B'), Some('A')],
            vec![None, Some('D')],
            vec![Some('C'), Some('C')],
            vec![Some('D'), Some('A')],
        ]
    };

    let successors = init_state.successors();
    assert_eq!(10, successors.len());
}

// D just moved - B must go outside
#[test]
fn test_successors_3() {
    let init_state = State {
        hallway: [None, None, None, Some('B'), None, Some('D'), None, None, None, None, None],
        rooms: [
            vec![Some('B'), Some('A')],
            vec![None, None],
            vec![Some('C'), Some('C')],
            vec![Some('D'), Some('A')],
        ]
    };

    let successors = init_state.successors();
    assert_eq!(6, successors.len());
}

#[test]
fn test_successors_4() {
    let init_state = State {
        hallway: [None, None, None, None, None, Some('D'), None, None, None, None, None],
        rooms: [
            vec![Some('B'), Some('A')],
            vec![None, Some('B')],
            vec![Some('C'), Some('C')],
            vec![Some('D'), Some('A')],
        ]
    };

    let successors = init_state.successors();
    assert_eq!(7, successors.len());

    let expected_state = State {
        hallway: [None, None, None, None, None, Some('D'), None, None, None, None, None],
        rooms: [
            vec![None, Some('A')],
            vec![Some('B'), Some('B')],
            vec![Some('C'), Some('C')],
            vec![Some('D'), Some('A')],
        ]
    };

    for (successor, price) in successors {
        if successor == expected_state {
            assert_eq!(price, 40);
        }
    }
}
