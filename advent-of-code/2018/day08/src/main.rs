use std::fs;

fn metadatas(numbers: &Vec<usize>, idx: usize) -> (usize, usize, usize) {
    let num_entries = numbers[idx];
    let num_metadatas = numbers[idx + 1];

    let mut metadata_sums = 0;
    let mut metadata_sums2 = 0;
    let mut idx = idx + 2;

    let mut v = vec![];

    for _i in 0..num_entries {
        let r = metadatas(numbers, idx);

        v.push(r.2);

        metadata_sums += r.0;
        idx = r.1;
    }

    for i in 0..num_metadatas {
        metadata_sums += numbers[idx + i];
    }

    if num_entries != 0 {
        for i in 0..num_metadatas {
            if numbers[i + idx] > 0 && numbers[i + idx] <= v.len() {
                metadata_sums2 += v[numbers[i + idx] - 1];
            }
        }
    } else {
        // if num_entries == 0, then metadata_sums doesn't change.
        metadata_sums2 = metadata_sums;
    }

    (metadata_sums, idx + num_metadatas, metadata_sums2)
}


fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let numbers = contents.split_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    let r = metadatas(&numbers, 0);
    println!("Part #1: {}", r.0);
    println!("Part #2: {}", r.2);
}
