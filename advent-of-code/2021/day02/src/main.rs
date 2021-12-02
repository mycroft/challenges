fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut horizontal_position = 0;
    let mut depth1 = 0;
    let mut depth2 = 0;
    let mut aim = 0;

    for line in lines {
        let elements = line.split(" ").collect::<Vec<&str>>();

        let number = elements[1].parse::<i32>().expect("valid number");

        match elements[0] {
            "forward" => {
                horizontal_position += number;
                depth2 += aim * number;
            },
            "up" => {
                depth1 -= number;
                aim -= number;
            },
            "down" => {
                depth1 += number;
                aim += number;
            }
            _ => unimplemented!(),
        }
    }

    println!("#1: {}", horizontal_position * depth1);
    println!("#2: {}", horizontal_position * depth2);
}
