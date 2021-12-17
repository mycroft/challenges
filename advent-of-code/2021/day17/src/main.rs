#[macro_use] extern crate scan_fmt;

struct Area {
    x_from: i32,
    x_to: i32,
    y_from: i32,
    y_to: i32
}

fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file");

    let (x_from, x_to, y_from, y_to) = scan_fmt!(
        &contents,
        "target area: x={}..{}, y={}..{}",
        i32, i32, i32, i32
    ).unwrap();

    let area = Area{
        x_from,
        x_to,
        y_from,
        y_to,
    };

    let mut max_y = 0;
    let mut count = 0;

    for x in 0..x_to+1 {
        for y in y_to..1000 {
            let res = play(&area, x, y);
            if res.0 {
                if res.1 > max_y {
                    max_y = res.1;
                }
                count += 1;
            }
        }
    }

    println!("#1 {}", max_y);
    println!("#2 {}", count);
}

fn play(area: &Area, vel_x: i32, vel_y: i32) -> (bool, i32) {
    let mut vel_x = vel_x;
    let mut vel_y = vel_y;
    let mut pos_x = 0;
    let mut pos_y = 0;

    let mut max_y = 0;

    let mut result = false;

    loop {
        // We apply velocity to the probe to move it
        pos_x += vel_x;
        pos_y += vel_y;

        // We change velocity
        if vel_x > 0 {
            vel_x -= 1;
        }
        vel_y -= 1;

        if pos_y > max_y {
            max_y = pos_y;
        }

        // if probe in the area, we stop with true
        if pos_x >= area.x_from && pos_x <= area.x_to && pos_y >= area.y_from && pos_y <= area.y_to {
            result = true;
            break
        }

        // if probe under the target area, we stop.
        if pos_y < area.y_from || pos_x > area.x_to {
            break
        }
    }

    (result, max_y)
}