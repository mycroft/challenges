#[allow(dead_code)]
fn dump(grid: &Vec<Vec<bool>>) {
    for line in grid {
        println!("{}", line.iter().map(|x| if *x { '#' } else { '.' }).collect::<String>());
    }
}

fn rotate(v: &Vec<usize>, n: usize) -> Vec<usize> {
    let mut new_v = vec![];

    for z in v[n..].iter() {
        new_v.push(*z);
    }

    for z in v[..n].iter() {
        new_v.push(*z);
    }

    new_v
}

fn run(v: &Vec<usize>, input: &Vec<usize>, rounds: usize) -> Vec<usize> {
    let mut v = v.to_vec();
    let mut skip_size = 0;
    let mut dec = 0;

    for _ in 0..rounds {
        let mut input = input.to_vec();

        loop {
            if input.len() == 0 {
                break;
            }

            let current_input = input[0];
            input = input[1..].to_vec();

            let to_rotate = &mut v[0..current_input];
            to_rotate.reverse();

            let new_dec = (skip_size+current_input) % v.len();
            v = rotate(&v, new_dec);

            dec += new_dec;
            skip_size += 1;
        }
    }

    rotate(&v, v.len() - dec % v.len())
}

fn hash(input: &str) -> Vec<usize> {
    let mut input : Vec<usize> = input.as_bytes().iter().map(|x| *x as usize).collect::<Vec<usize>>();
    let mut v = (0..256).collect::<Vec<usize>>();

    for n in vec![17, 31, 73, 47, 23] {
        input.push(n);
    }

    v = run(&v, &input, 64);

    let mut idx : usize = 0;
    let mut out = vec![];

    loop {
        if idx >= v.len() {
            break;
        }

        out.push(v[idx..idx+16].iter().fold(0, |x, y| x ^ y));
        idx+=16;
    }

    out
}

fn to_bits(n: usize) -> Vec<bool> {
    let mut n = n;
    let mut out = Vec::new();

    for _ in 0..8 {
        out.insert(0, n % 2 == 1);
        n = n.rotate_right(1);
    }

    out
}

fn remove(grid: &mut Vec<Vec<bool>>, point: (usize, usize)) {
    let mut subset = Vec::<(usize,usize)>::new();
    subset.push(point);

    let deltas = [(0,1),(0,-1),(1,0),(-1,0)];

    loop {
        if subset.len() == 0 {
            break;
        }

        // find current point neighbors
        let current_point = subset.pop().unwrap();

        for delta in &deltas {
            let x : i16 = delta.0 + current_point.0 as i16;
            let y : i16 = delta.1 + current_point.1 as i16;

            if x < 0 || y < 0 || x > 127 || y > 127 {
                continue;
            }

            if grid[x as usize][y as usize] {
                subset.push((x as usize, y as usize));
            }
        }

        // remove current point from grid
        grid[current_point.0][current_point.1] = false;
    }

}

fn main() {
    let input = "hxtvlmkl";

    let mut grid = vec![];

    for idx in 0..128 {
        let mut line = vec![];
        let h = hash(format!("{}-{}", input, idx).as_str());

        for el in &h {
            let bits = to_bits(*el);
            for b in bits {
                line.push(b);
            }
        }

        grid.push(line);
    }

    let _res : usize = grid
        .iter()
        .map(|x| x
            .iter()
            .filter(|x| **x)
            .count()
        )
        .sum();

    println!("Part #1: {}", _res);

    let mut groups : usize = 0;

    for i in 0..128 {
        for j in 0..128 {
            // find next '#t'

            if !grid[i][j] {
                continue;
            }

            remove(&mut grid, (i, j));

            groups += 1;
        }
    }

    println!("Part #2: {}", groups);
}
