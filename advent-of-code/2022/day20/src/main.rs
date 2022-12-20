use std::fs::read_to_string;
use std::collections::VecDeque;

#[derive(Debug, Copy, Clone)]
struct Number {
    idx: usize,
    number: i128,
}

fn parse(fp: &str) -> VecDeque<Number> {
    let contents = read_to_string(fp).unwrap();

    contents.lines().enumerate().map(
        |(idx, x)|
        Number{
            idx: idx,
            number: x.parse::<i128>().unwrap()
        }
    ).collect()
}

fn display(ring: &VecDeque<Number>) {
    let numbers = ring.iter().map(|x| x.number).collect::<Vec<i128>>();
    println!("{:?}", numbers);
}

fn mix(ring: &mut VecDeque<Number>, decryption_key: i128, mix_number: usize) -> i128 {
    let ring_len = ring.len();

    // apply decryption_key;
    ring.iter_mut().map(|x| x.number *= decryption_key).count();

    // println!("Initial arrangement:");
    // display(&ring);

    for mix_idx in 0..mix_number {
        for idx in 0..ring_len {
            // rotate until element is in front.
            let index0 = ring.iter().position(|x| x.idx == idx).unwrap() as usize;
            ring.rotate_left(index0);
    
            let el = ring.pop_front().unwrap();
            let mut rotations = el.number.abs() as usize;

            while rotations >= ring_len {
                let full_rotations = rotations / ring_len;
                let rotations_reminder = rotations % ring_len;
    
                rotations = full_rotations + rotations_reminder;
            }

            if el.number == 0 {
                ring.push_front(el);
                continue;
            } else if el.number < 0 {
                ring.rotate_right(rotations);
            } else if el.number > 0 {
                ring.rotate_left(rotations);
            }
    
            ring.push_front(el);
        }

        // let index0 = ring.iter().position(|x| x.number == 0).unwrap() as usize;
        // ring.rotate_left(index0);

        // println!("After {} round of mixing:", mix_idx + 1);
        // display(&ring);
    }

    // find index of element 0
    let index0 = ring.iter().position(|x| x.number == 0).unwrap() as usize;
    let mut s = 0;

    ring.rotate_left(index0);

    // println!("{} {} {}",
    //     ring[1000 % ring_len].number,
    //     ring[2000 % ring_len].number,
    //     ring[3000 % ring_len].number,
    // );

    s += ring[1000 % ring_len].number;
    s += ring[2000 % ring_len].number;
    s += ring[3000 % ring_len].number;

    s

}

fn main() {
    let mut ring = parse("input.txt");
    println!("#1 {}", mix(&mut ring, 1, 1)); // 19559

    let mut ring = parse("input.txt");
    println!("#2 {}", mix(&mut ring, 811589153, 10)); // 1623178306
}

#[test]
fn test_sample() {
    let mut ring = parse("input.txt_test");
    assert_eq!(
        3,
        mix(&mut ring, 1, 1),
    );

    let mut ring = parse("input.txt_test");
    assert_eq!(
        1623178306,
        mix(&mut ring, 811589153, 10),
    );
}

#[test]
fn test_my_input() {
    let mut ring = parse("input.txt");
    assert_eq!(
        19559,
        mix(&mut ring, 1, 1),
    );
}