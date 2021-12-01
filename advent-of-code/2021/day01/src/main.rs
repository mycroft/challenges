use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    
    let mut increase = 0;
    let mut last_value : Option<i32> = None;
    let mut value : Option<i32> = None;

    let mut increase_window = 0;
    let mut last_value_window : Option<i32> = None;
    let mut value_window : Option<i32> = None;

    let mut window : Vec<i32> = vec![];

    for line in lines {
        value = Some(line.parse::<i32>().expect("number"));

        if let Some(last_value) = last_value {
            if value.unwrap() > last_value {
                increase += 1;
            }
        }

        last_value = value;

        window.push(value.unwrap());
        if window.len() != 3 {
            continue;
        }
        
        value_window = Some(window.iter().sum::<i32>());

        println!("{:?} {:?}", value_window, window);

        if let Some(last_value_window) = last_value_window {
            if value_window.unwrap() > last_value_window {
                increase_window += 1;
            }
        }

        last_value_window = value_window;

        // remove first element.
        window.remove(0);
    }

    println!("#1: {}", increase);
    println!("#2: {}", increase_window);
}
