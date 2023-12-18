use std::{fs, collections::HashMap};

#[derive(Debug, Clone)]
struct Item {
    label: String,
    focal: usize,
}

fn hash(s: &str) -> usize {
    let mut res = 0;

    for c in s.chars() {
        res = ((res + (c as usize)) * 17) % 256;
    }

    res
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("file to open");
    let items = contents.trim().split(',').collect::<Vec<&str>>();

    let p1: usize = items.iter().map(|item| hash(item)).sum();

    println!("#1 {}", p1);

    let mut hm: HashMap<usize, Vec<Item>> = HashMap::new();

    for item in items {
        if item.ends_with('-') {
            let item = item.split('-').next().unwrap();
            let h = hash(item);

            let v = hm.get_mut(&h);
            if v.is_none() {
                continue;
            }

            let pos = v.as_ref().unwrap().iter().position(|x| x.label == *item);
            if pos.is_none() {
                continue;
            }

            v.unwrap().remove(pos.unwrap());
        } else {
            let mut parts = item.split('=');
            let item = parts.next().unwrap();
            let focal = parts.next().unwrap().parse::<usize>().unwrap();

            let h = hash(item);

            hm.entry(h).or_default();

            let v = hm.get_mut(&h).unwrap();
            let pos = v.iter().position(|x| x.label == *item);
            if pos.is_none() {
                v.push(Item{
                    label: item.to_string(),
                    focal,
                });
            } else {
                v[pos.unwrap()].focal = focal;
            }
        }
    }

    let mut p2 = 0;

    for box_id in 0..256 {
        let v = hm.get(&box_id);

        if v.is_none() {
            continue;
        }

        let v = v.unwrap();

        for (n, b) in v.iter().enumerate() {
            // println!("{}", (1+box_id) * (1+n) * b.focal);
            p2 += (1+box_id) * (1+n) * b.focal;
        }

    }

    println!("#2 {}", p2);

}