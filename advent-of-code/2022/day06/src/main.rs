use std::fs::read_to_string;
use std::collections::{VecDeque, HashSet};

fn check_valid(v: &VecDeque<char>, expected_size: usize) -> bool {
    if v.len() != expected_size {
        return false
    }

    let mut hs: HashSet<char> = HashSet::new();
    
    for c in v.iter() {
        if hs.contains(c) {
            return false;
        }
        hs.insert(*c);
    }

    return true;
}

fn get_start_pos(s: &str, expected_size: usize) -> usize {
    let chars: Vec<char> = s.chars().collect();

    let mut start_packet : VecDeque<char> = VecDeque::new();
    let mut pos = 0;

    for i in 0..chars.len() {
        if start_packet.len() == expected_size {
            start_packet.pop_front();
        }

        start_packet.push_back(chars[i]);

        if check_valid(&start_packet, expected_size) {
            pos = i + 1;
            break;
        }
    }

    pos
}

fn main() {
    let contents = read_to_string("input.txt").unwrap();

    println!("#1 {}", get_start_pos(&contents, 4));
    println!("#2 {}", get_start_pos(&contents, 14));
}

#[test]
fn test() {
    assert_eq!(7, get_start_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
    assert_eq!(5, get_start_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
    assert_eq!(6, get_start_pos("nppdvjthqldpwncqszvftbrmjlhg", 4));
    assert_eq!(10, get_start_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
    assert_eq!(11, get_start_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));

    assert_eq!(19, get_start_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
    assert_eq!(23, get_start_pos("bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
    assert_eq!(23, get_start_pos("nppdvjthqldpwncqszvftbrmjlhg", 4));
    assert_eq!(29, get_start_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
    assert_eq!(26, get_start_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));

}