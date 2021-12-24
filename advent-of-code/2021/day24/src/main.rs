/* AOC 2021 day 24 */

/*
inp w
mul x 0
add x z
mod x 26

div z F1 // 1 or 26
add x F2

eql x w
eql x 0

mul y 0
add y 25
mul y x
add y 1

mul z y
mul y 0
add y w
add y F3
mul y x
add z y
*/

use std::collections::HashMap;

fn compute(inp: i128, z: i128, f1: i128, f2: i128, f3: i128) -> i128 {
    let mut z = z;
    let x;

    // Simplified code:
    x = ((z % 26 + f2) != inp) as i128;
    z /= f1;
    z *= (25 * x) + 1;
    z += (inp+f3)*x;

    z
}

fn dorec(cache: &mut HashMap<(usize, i128), i128>, steps: &[(i128, i128, i128)], step_index: usize, z: i128, part2: bool) -> (i128, Option<Vec<i8>>) {
    if step_index == steps.len() {
        return (z, None);
    }

    if cache.contains_key(&(step_index, z)) {
        return (*cache.get(&(step_index, z)).unwrap(), None);
    }

    for x in 0..9 {
        // for part 2, we go from 1 to 9 instead of 9 to 1.
        let x = if part2 {
            x + 1
        } else {
            9 - x
        };

        let ret_z = compute(x, z, steps[step_index].0, steps[step_index].1, steps[step_index].2);
        cache.insert((step_index, z), ret_z);

        let ret_z = dorec(cache, steps, step_index + 1, ret_z, part2);
        if ret_z.0 == 0 {
            let mut v = vec![];

            if let Some(tmp) = ret_z.1 {
                v = tmp;
            }

            v.insert(0, x as i8);

            return (
                ret_z.0,
                Some(v)
            );
        }
    }

    (z, None)
}

fn main() {
    // This is my input, which only includes changing numbers.
    // The arithmetic part was decoded and is located in the compute() function.
    let steps = vec![
        (1, 10, 15),
        (1, 12, 8),
        (1, 15, 2),
        (26, -9, 6),
        (1, 15, 13),

        (1, 10, 4),
        (1, 14, 1),
        (26, -5, 9),
        (1, 14, 5),
        (26, -7, 13),

        (26, -12, 9),
        (26, -10, 6),
        (26, -1, 2),
        (26, -11, 2),
    ];

    let val = dorec(&mut HashMap::new(), &steps, 0, 0, false);
    println!("#1 {}", val.1.unwrap().iter().fold(0i128, |acc, &x| acc * 10 + x as i128));

    let val = dorec(&mut HashMap::new(), &steps, 0, 0, true);
    println!("#2 {}", val.1.unwrap().iter().fold(0i128, |acc, &x| acc * 10 + x as i128));
}
