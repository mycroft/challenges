use std::collections::VecDeque;

/// ring first element is current_marble.
/// Return score to add.
fn step(ring: &mut VecDeque<usize>, marble: usize) -> usize {
    // first element is current_marble.
    // to place "marble" at its place, we pop front elements we push back & place our marble.
    let mut score = 0;

    if marble % 23 == 0 {
        for _i in 0..6 {
            let el = ring.pop_back();
            ring.push_front(el.unwrap());
        }

        score += marble;

        let el = ring.pop_back();
        score += el.unwrap();
    } else {
        for _i in 0..2 {
            let el = ring.pop_front();
            ring.push_back(el.unwrap());
        }

        ring.push_front(marble);
    }

    score
}

fn play(players: usize, highest: usize) -> usize {
    let mut ring = VecDeque::<usize>::new();
    let mut current_marble = 1;
    let mut current_player = 2;

    let mut scores = vec![0; players];

    ring.push_front(0);

    loop {
        let score = step(&mut ring, current_marble);

        scores[current_player - 1] += score;

        current_player += 1;
        current_marble += 1;

        if current_marble > highest {
            break;
        }

        if current_player > players {
            current_player = 1;
        }
    }

    *scores.iter().max().unwrap()
}

fn main() {
    let score = play(431, 70950);
    println!("Part #1: {}", score);

    let score = play(431, 70950*100);
    println!("Part #1: {}", score);

}

#[test]
fn sampeles() {
    assert_eq!(8317, play(10, 1618));
    assert_eq!(146373, play(13, 7999));
    assert_eq!(2764, play(17, 1104));
    assert_eq!(54718, play(21, 6111));
    assert_eq!(37305, play(30, 5807));
}
