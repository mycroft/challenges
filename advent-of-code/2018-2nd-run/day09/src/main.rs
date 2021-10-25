/* AOC 2018 day09 */
use std::collections::VecDeque;

fn play(players_num: usize, turns_num: usize) -> usize {
    let mut current_player = 0; // players are going from 0 to players_num - 1
    let mut scores : Vec<usize> = vec![0; players_num];
    let mut marbles : VecDeque<usize> = VecDeque::new();

    marbles.push_back(0);
    let mut current_turn = 1;

    loop {
        // end of game
        if current_turn > turns_num {
            break;
        }

        if current_turn % 23 != 0 {
            for _i in 0..2 {
                let el = marbles.pop_front().unwrap();
                marbles.push_back(el);
            }

            marbles.push_front(current_turn);
        } else {
            // special case: 23.
            scores[current_player] += current_turn;

            for _ in 0..6 {
                let el = marbles.pop_back().unwrap();
                marbles.push_front(el);
            }

            scores[current_player] += marbles.pop_back().unwrap();
        }

        // next player.
        current_player = (current_player + 1) % players_num;
        current_turn += 1;
    }

    *scores.iter().max().unwrap()
}

fn main() {
    println!("#1: {}", play(431, 70950));
    println!("#2: {}", play(431, 7095000));
}

#[test]
fn basic_test() {
    assert_eq!(32,      play(9, 25));
    assert_eq!(8317,    play(10, 1618));
    assert_eq!(146373,  play(13, 7999));
    assert_eq!(2764,    play(17, 1104));
    assert_eq!(54718,   play(21, 6111));
    assert_eq!(37305,   play(30, 5807));
}
