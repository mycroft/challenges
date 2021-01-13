use std::collections::VecDeque;

#[derive(Debug,Clone)]
struct Elf {
    id: usize,
    presents: usize,
}

fn get_ring(input: usize) -> VecDeque<Elf> {
    (1..=input)
        .map(|x| Elf{id: x, presents: 1})
        .collect::<VecDeque<Elf>>()
}

fn round_step(values: Vec<usize>) -> Vec<usize> {
    let mut r : Vec<usize> = Vec::with_capacity(values.len() - 1);
    let first_high = (values.len() + 2) / 3;
    let removed_index = values.len() / 2;
    let first_inc = 2 - values.len() % 2;

    for &value in values[first_high..removed_index].iter() {
        r.push(value);
    }

    for i in (removed_index+first_inc..values.len()).step_by(3) {
        r.push(values[i]);
    }

    for &value in values[..first_high].iter() {
        r.push(value);
    }

    r
}

fn main() {
    let input = 3017957;

    let mut ring = get_ring(input);

    loop {
        if ring.len() == 1 {
            break;
        }

        let next = ring.remove(1).unwrap();
        let elf = ring.get_mut(0).unwrap();

        elf.presents += next.presents;
        ring.rotate_left(1);
    }

    println!("Part #1: {}", ring[0].id);


    let mut elves = (1..=input).collect::<Vec<usize>>();

    while elves.len() > 1 {
        elves = round_step(elves);
    }

    println!("Part #2: {}", elves[0]);
}
