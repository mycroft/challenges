/* AOC 2018: Ex 11 */

fn fuel_cell(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id : i32 = x as i32 + 10;
    let power_level : i32 = (rack_id * (y as i32) + serial) * rack_id;

    (((power_level - (power_level % 100)) % 1000) / 100) - 5
}

fn find_max(matrix: &Vec<Vec<i32>>, size: usize) -> (usize, usize, i32) {
    let mut max_coord = (0, 0);
    let mut max_value = -1;

    let deltas : Vec<(i32, i32)> = (0..size).map(|x| {
        (0..size).map(|y| {
            (x as i32, y as i32)
        }).collect::<Vec<(i32, i32)>>()
    }).flatten().collect();

    for x in 0..(300-size) {
        for y in 0..(300-size) {
            let sum : i32 = deltas
                .iter()
                .map(|delta| (x as i32 + delta.0, y as i32 + delta.1))
                .map(|p| matrix[p.0 as usize][p.1 as usize])
                .sum();

            if sum > max_value {
                max_value = sum;
                max_coord = (x, y);
            }
        }
    }

    (max_coord.0 as usize + 1, max_coord.1 as usize + 1, max_value)
}

fn build_matrix(serial: i32) -> Vec<Vec<i32>> {
    (1..=300).map(|x| {
        (1..=300).map(|y| {
            fuel_cell(x, y, serial)
        }).collect::<Vec<i32>>()
    }).collect::<Vec<Vec<i32>>>()
}

fn main() {
    let serial = 2187;

    let matrix = build_matrix(serial);
    let (x, y, val) = find_max(&matrix, 3);

    println!("#1 {},{} (value: {})", x, y, val);

    let mut max_value = -1;
    let mut max_coords = (0, 0, 0);

    for size in 1..=30 {
        let (x, y, val) = find_max(&matrix, size);
        if val > max_value {
            max_coords = (x, y, size);
            max_value = val;
        }
    }

    println!("#2 {},{},{} (value: {})", max_coords.0, max_coords.1, max_coords.2, max_value);
}

#[test]
fn test0() {
    assert_eq!(4, fuel_cell(3, 5, 8));
    assert_eq!(-5, fuel_cell(122, 79, 57));
    assert_eq!(0, fuel_cell(217, 196, 39));
    assert_eq!(4, fuel_cell(101, 153, 71));
}

#[test]
fn test1() {
    let matrix = build_matrix(18);
    assert_eq!(
        (33, 45, 29),
        find_max(&matrix, 3)
    );

    assert_eq!(
        (90, 269, 113),
        find_max(&matrix, 16)
    );

    let matrix = build_matrix(42);
    assert_eq!(
        (232,251,119),
        find_max(&matrix, 12)
    );
}
