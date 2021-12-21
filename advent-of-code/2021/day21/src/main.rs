use std::fmt;
use std::collections::HashMap;

fn play(p0pos: usize, p1pos: usize) -> usize {
    let mut number_of_rolls = 0;

    let mut positions = vec![p0pos-1, p1pos-1];
    let mut scores = vec![0; 2];

    let mut current_dice = 0;
    let mut current_player = 0;

    loop {
        let mut rolls_total = 0;
        for _ in 0..3 {
            current_dice += 1;
            rolls_total += current_dice;
        }
        number_of_rolls += 3;

        positions[current_player] += rolls_total;
        positions[current_player] %= 10;

        scores[current_player] += positions[current_player] + 1;

        if scores[current_player] >= 1000 {
            break;
        }

        current_player = (current_player + 1) % 2;
    }

    number_of_rolls * scores[(current_player+1)%2]
}

// Max known universes:
// 0-14 rolls x 0-21 scores x 0-21 scores x 2 players = 14520 universes
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Universe {
    positions: (usize, usize),
    scores: (usize, usize),
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "pos:{}, {} scores: {}, {}",
            self.positions.0, self.positions.1,
            self.scores.0, self.scores.1,
        )
    }
}

// Returns wins for players
fn compute(univs: &mut HashMap<Universe, (u128, u128)>, current_universe: &Universe, expected_score: usize) -> (u128, u128) {
    // In this universe, we can create 7 new universes, because the current player will rolls
    // dice 27 times, but leaving with 7 differents results:
    let rolls = [
        (3usize, 1u128), // score, number of possible universes created with this one
        (4, 3),
        (5, 6),
        (6, 7),
        (7, 6),
        (8, 3),
        (9, 1),
    ];

    // If current universe is already finished, don't go forward.
    // This is our stop condition.
    if current_universe.scores.0 >= expected_score {
        return (1, 0);
    } else if current_universe.scores.1 >= expected_score {
        return (0, 1);
    } else if univs.contains_key(current_universe) {
        return *univs.get(current_universe).unwrap();
    }

    // We assume we're in an universe in which player 0 is playing.
    let mut results = (0, 0);
    for roll in rolls {
        let mut new_universe = *current_universe;

        new_universe.positions.0 = (new_universe.positions.0 + roll.0 - 1) % 10 + 1;
        new_universe.scores.0 += new_universe.positions.0;

        let switched_universe = Universe{
            positions: (new_universe.positions.1, new_universe.positions.0),
            scores: (new_universe.scores.1, new_universe.scores.0),
        };

        let res = compute(
            univs,
            &switched_universe,
            expected_score
        );
        
        results.0 += res.1 * roll.1;
        results.1 += res.0 * roll.1;
    }

    *univs.entry(*current_universe).or_insert((0, 0)) = results;

    results
}

fn play2(p0pos: usize, p1pos: usize, expected_score: usize) -> u128 {
    let mut univs = HashMap::new();
    let current_universe = Universe {
        positions: (p0pos, p1pos),
        scores: (0, 0),
    };

    let res = compute(&mut univs, &current_universe, expected_score);
    if res.0 > res.1 {
        res.0
    } else {
        res.1
    }
}

fn main() {
/*
Player 1 starting position: 2
Player 2 starting position: 1
*/
    println!("#1 {}", play(2, 1));
    println!("#2 {}", play2(2, 1, 21));

    // for fun, all possibles inputs:
    for p1 in 1..=10 {
        for p2 in 1..=10 {
            println!("#2 for {},{} {}", p1, p2,play2(p1, p2, 21));
        }
    }
}

#[test]
fn testplay() {
    assert_eq!(739785, play(4, 8));
    assert_eq!(444356092776315, play2(4, 8, 21));
}