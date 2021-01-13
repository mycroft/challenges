use std::fs;
use std::cmp::Ordering;

fn simplify(vectors: &mut Vec<(u32, u32)>) -> (Vec<(u32, u32)>, bool) {
    let mut my_ref = vectors.remove(0);
    let mut idx = 1;
    let mut changed = false;

    loop {
        if idx == vectors.len() {
            break;
        }

        let current = vectors[idx];
        let mut used = false;

        if my_ref.0 >= current.0 && my_ref.0 <= current.1 {
            my_ref = (current.0, my_ref.1);
            used = true;
        }

        if my_ref.1 >= current.0 && my_ref.1 <= current.1 {
            my_ref = (my_ref.0, current.1);
            used = true;
        }

        if current.1 != u32::MAX && my_ref.0 == current.1 + 1 {
            my_ref = (current.0, my_ref.1);
            used = true;
        }

        if my_ref.1 != u32::MAX && my_ref.1 + 1 == current.0 {
            my_ref = (my_ref.0, current.1);
            used = true;
        }

        if used {
            vectors.remove(idx);
            changed = true;
            continue;
        }

        idx += 1;
    }

    vectors.push(my_ref);

    (vectors.to_vec(), changed)
}

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines();

    let mut vectors = vec![];

    for line in lines {
        let numbers : Vec<&str> = line.split('-').collect();

        vectors.push((
            numbers[0].parse::<u32>().unwrap(),
            numbers[1].parse::<u32>().unwrap()
        ))
    }

    for _i in 0..vectors.len() {
        let res = simplify(&mut vectors);
        vectors = res.0;
    }

    let mut start = 0;
    let mut count = 0;

    loop {
        if vectors.len() == 0 {
            break;
        }

        let min_idx = vectors
            .iter()
            .enumerate()
            .min_by(|(_, (x, _)), (_, (y, _))| x.partial_cmp(y).unwrap_or(Ordering::Equal))
            .unwrap();

        let index = min_idx.0;

        if start != 0 {
            count += min_idx.1.0 - start;
            // println!("{} -> {} (count:{})", start, min_idx.1.0, count);
        } else {
            println!("Part #1: {}", min_idx.1.1 + 1);
        }
        
        if min_idx.1.1 != u32::MAX {
            start = min_idx.1.1 + 1;    
        }

        vectors.remove(index);
    }

    println!("Part #2: {}", count);
}
