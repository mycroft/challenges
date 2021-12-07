fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file to read");
    let contents = contents.trim_end();
    let numbers = contents
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let max_number = *numbers.iter().max().unwrap();

    let mut least_fuel = (None, None);

    let mut triangles: Vec<i32> = vec![];
    triangles.push(0);
    triangles.push(1);

    for n in 2..2000 {
        triangles.push(triangles[n-1]+n as i32);
    }

    for n in 0..max_number {
        let mut required_fuel = (0, 0);

        for num in &numbers {
            let this_required = (n - num).abs();

            required_fuel.0 += this_required;
            required_fuel.1 += triangles[this_required as usize]
        }

        if least_fuel.0 == None || required_fuel.0 < least_fuel.0.unwrap() {
            least_fuel.0 = Some(required_fuel.0);
        }
        if least_fuel.1 == None || required_fuel.1 < least_fuel.1.unwrap() {
            least_fuel.1 = Some(required_fuel.1);
        }
    }

    println!("#1: {}", least_fuel.0.unwrap());
    println!("#2: {}", least_fuel.1.unwrap());
}
