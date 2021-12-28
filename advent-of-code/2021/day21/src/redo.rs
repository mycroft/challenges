use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Universe {
    p1_pos: usize,
    p2_pos: usize,
    p1_score: usize,
    p2_score: usize,
    current_p1: bool,
}

fn play(cache: &mut HashMap<Universe, (u128, u128)>, universe: &Universe, expected: usize) -> (u128, u128) {
    if universe.p1_score >= expected {
        return (1, 0);
    } else if universe.p2_score >= expected {
        return (0, 1);
    }

    if cache.contains_key(universe) {
        return *cache.get(universe).unwrap();
    }

    let rolls = [
        (3usize, 1u128),
        (4, 3),
        (5, 6),
        (6, 7),
        (7, 6),
        (8, 3),
        (9, 1),
    ];

    let mut res = (0u128, 0u128);

    for roll in rolls {
        let new_pos = if universe.current_p1 {
            (
                (universe.p1_pos - 1 + roll.0) % 10 + 1,
                universe.p2_pos,
            )            
        } else {
            (
                universe.p1_pos,
                (universe.p2_pos - 1 + roll.0) % 10 + 1
            )
        };

        let new_scores = if universe.current_p1 {
            (
                universe.p1_score + new_pos.0,
                universe.p2_score,
            )
        } else {
            (
                universe.p1_score,
                universe.p2_score + new_pos.1,
            )
        };
        
        let new_universe = Universe {
            p1_pos: new_pos.0,
            p2_pos: new_pos.1,
            p1_score: new_scores.0,
            p2_score: new_scores.1,
            current_p1: !universe.current_p1,
        };

        // There are roll.1 universe like this one ^

        let res_t = play(cache, &new_universe, expected);

        res.0 += res_t.0 * roll.1;
        res.1 += res_t.1 * roll.1;
    }

    cache.insert(*universe, res);

    res
}

fn main() {
    let mut cache = HashMap::new();
    let universe = Universe{p1_pos: 2, p2_pos: 1, p1_score: 0, p2_score: 0, current_p1: true};

    let res = play(&mut cache, &universe, 21);

    println!("{:?}", res);
}
