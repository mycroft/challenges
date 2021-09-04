pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut v : Vec<Vec<u32>> = (0..size).into_iter().map(|_| vec![0; size as usize]).collect();

    let mut coords = (0i32, 0i32);
    let mut direction = (1i32, 0i32);

    let mut current_num = 1i32;

    while current_num <= (size * size) as i32 {
        v[coords.1 as usize][coords.0 as usize] = current_num as u32;

        // check if we're in a border or if next number on direction is not 0. 
        if (coords.0 + direction.0 >= size as i32 && direction.0 != 0) 
            || (coords.1 + direction.1 >= size as i32 && direction.1 != 0)
            || (coords.0 + direction.0 < 0 && direction.0 != 0)
            || (coords.1 + direction.1 < 0 && direction.1 != 0)
            || (v[(coords.1 + direction.1) as usize][(coords.0 + direction.0) as usize] != 0) {

            // change direction as required.
            direction = match direction {
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                (0, -1) => (1, 0),
                _ => panic!("invalid direction"),
            };
        }

        coords = (coords.0 + direction.0, coords.1 + direction.1);
        current_num += 1;
    }

    v
}
