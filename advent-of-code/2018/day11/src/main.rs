fn p(x: usize, y: usize, input: i32) -> i32 {
    let rack_id : i32 = x as i32 + 10;
    let res = (rack_id * y as i32 + input) * rack_id;
    let digit = (res / 100) % 10;

    digit - 5
}

fn main() {
    let input = 2187;

    let mut grid = vec![vec![0; 300]; 300];

    for x in 0..300 {
        for y in 0..300 {
            grid[x][y] = p(x + 1, y + 1, input);
        }
    }

    let mut max_value = -10;
    let mut max_point = (0, 0);

    for x in 0..298 {
        for y in 0..298 {
            let mut t = 0;

            for i in 0..3 {
                for j in 0..3 {
                    t += grid[x + i][y + j];
                }
            }

            if t > max_value {
                max_value = t;
                max_point = (x+1, y+1);
            }

        }
    }

    println!("Part #1: {},{}", max_point.0, max_point.1);

    let mut max_value = -10;
    let mut max_point = (0, 0, 0);
    let mut dim = 2;

    for dim in 1..20 {
        for x in 0..300-(dim-1) {
            for y in 0..300-(dim-1) {
                let mut t = 0;

                for i in 0..dim {
                    for j in 0..dim {
                        t += grid[x +i][y + j];
                    }
                }

                if t > max_value {
                    max_value = t;
                    max_point = (x+1, y+1,dim);
                }
            }
        }
    }

    println!("{:?} {:?}", max_value, max_point);
}

#[test]
fn name() {
    assert_eq!(4, p(3, 5, 8));
    assert_eq!(-5, p(122, 79, 57));
    assert_eq!(0, p(217, 196, 39));
    assert_eq!(4, p(101, 153, 71));
}
