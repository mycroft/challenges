use std::collections::VecDeque;

fn main() {
    let mut z = VecDeque::new();
    let mut idx = 0;

    let input = 394;

    z.push_back(0);

    for _id in 1..=2017 {
        idx = (idx + input) % z.len();

        z.insert(idx+1, _id);
        idx += 1;
    }

    let pos = z.iter().enumerate().filter(|(_x, &y)| y == 2017).map(|(x, _y)| x).nth(0).unwrap();
    println!("Part #1: {}", z[pos+1]);

    let mut idx = 0;
    let mut num = 0;

    for _id in 1..=50000000 {
        idx = (idx + input) % _id;

        if idx == 0 {
            num = _id;
        }

        idx += 1;
    }

    println!("Part #2: {}", num);
}
