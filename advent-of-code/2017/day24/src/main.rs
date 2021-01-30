use std::fs;

fn path_to_strength(tuples: &Vec<(usize, usize)>) -> usize {
    tuples.iter().map(|(x, y)| *x + *y).sum()
}

fn pathlen(tuples: &Vec<(usize, usize)>, current_number: usize) -> (usize, Vec<Vec<(usize, usize)>>) {
    let tuples_len = tuples.len();

    let mut max_path = 0;
    let mut longuest_pathes = vec![vec![]];
    let mut tuples = tuples.clone();

    for index in 0..tuples_len {
        if tuples[index].0 != current_number && tuples[index].1 != current_number {
            continue;
        }

        let tuple = tuples[index];

        tuples.remove(index);

        let (mut ret, mut longuest) = if tuple.0 == current_number {
            pathlen(&tuples, tuple.1)
        } else {
            pathlen(&tuples, tuple.0)
        };

        tuples.insert(index, tuple);

        ret += tuple.0 + tuple.1;

        if ret > max_path {
            max_path = ret;
        }

        for new_longuest in longuest.iter_mut() {
            new_longuest.push(tuple);
        }

        if longuest_pathes.len() == 0 {
            for new_longuest in longuest {
                longuest_pathes.push(new_longuest);
            }

            continue;
        }

        if longuest_pathes[0].len() < longuest[0].len() {
            // replace existing longuest_pathes by longuest.
            // println!("Replacing {:?} by {:?}",longuest_pathes, longuest);
            longuest_pathes = longuest;
        } else if longuest_pathes[0].len() == longuest[0].len() {
            for new_longuest in longuest {
                longuest_pathes.push(new_longuest);
            }
        }

    }

    (max_path, longuest_pathes)
}



fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();

    let mut tuples : Vec<(usize, usize)> = vec![];

    for line in contents.lines() {
        println!("{:?}", line);

        let values = line.split("/").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        tuples.push((values[0], values[1]));
    }

    let (max, longuest) = pathlen(&tuples, 0);
    println!("Part #1: {}", max);

    let max = longuest.iter().map(|x| path_to_strength(&x)).max().unwrap();
    println!("Part #2: {}", max);
}
