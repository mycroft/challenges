use mkz_aoc::file;

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

            // println!("current input len: {:?}", current_input);

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

fn main() {
    // test
    // let v = (0..5).collect::<Vec<usize>>();
    // let input = vec![3, 4, 1, 5];

    let v = (0..256).collect::<Vec<usize>>();
    let input_str = file::read_to_string("input.txt").unwrap();
    let input = input_str
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let result_v = run(&v, &input, 1);

    println!("Part #1: {}", result_v[0] * result_v[1]);

    let mut input : Vec<usize> = input_str.as_bytes().iter().map(|x| *x as usize).collect::<Vec<usize>>();

    for n in vec![17, 31, 73, 47, 23] {
        input.push(n);
    }

    let mut v = (0..256).collect::<Vec<usize>>();

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

    let hash = out.iter().map(|x| format!("{:02x}", x)).collect::<String>();
    println!("Part #2: {}", hash);
}

#[test]
fn name() {
    assert_eq!(vec![0, 1], rotate(&vec![1, 0], 1));
}