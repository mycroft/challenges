fn zip(positions: &Vec<usize>, init: &Vec<usize>, n: usize) -> bool {

    positions
        .iter()
        .enumerate()
        .map(|(x, y)| (n + 1 + x + init[x]) % y)
        .all(|x| x == 0)
}

fn find(positions: &Vec<usize>, init: &Vec<usize>) -> usize {
    let mut i = 0;
    let mut c = 1;

    let mut my_pos = vec![positions[0]];

    loop {
        if zip(&my_pos, &init, i) {
            c *= my_pos[my_pos.len() - 1];
            if my_pos.len() == positions.len() {
                break;
            }            
            my_pos.push(positions[my_pos.len()]);
        }

        i += c
    }

    i
}

fn main() {
    let mut positions = vec![13, 17, 19, 7, 5, 3];
    let mut init = vec![10, 15, 17, 1, 0, 1];

    println!("Part #1: {:?}", find(&positions, &init));

    positions.push(11);
    init.push(0);

    println!("Part #2: {:?}", find(&positions, &init));
}

#[test]
fn name() {
    let positions = vec![5, 2];
    let init = vec![4, 1];

    assert_eq!(false, zip(&positions, &init, 0));
    assert_eq!(false, zip(&positions, &init, 2));
    assert_eq!(true, zip(&positions, &init, 5));
}
