use std::fs::read_to_string;
use std::collections::HashSet;

#[macro_use] extern crate scan_fmt;

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    nearest_beacon_x: i64,
    nearest_beacon_y: i64,
    manhattan_distance: i64
}

impl Sensor {
    fn can_be_beacon(&self, nx: i64, ny: i64) -> bool {
        let manhattan_distance = (self.x - nx).abs() + (self.y - ny).abs();

        manhattan_distance > self.manhattan_distance
    }

    // fn manhattan_distance(&self, nx: i64, ny: i64) -> i64 {
    //     (self.x - nx).abs() + (self.y - ny).abs()
    // }

    fn manhattan_diff(&self, nx: i64, ny: i64) -> i64 {
        if !self.can_be_beacon(nx, ny) {
            return self.manhattan_distance - ((self.x - nx).abs() + (self.y - ny).abs());
        }

        return 0;
    }
}

fn parse(fp: &str) -> Vec<Sensor> {
    let contents = read_to_string(fp).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut res = Vec::new();

    for line in lines {
        let (x, y, nx, ny) = scan_fmt!(
            line,
            "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
            i64, i64, i64, i64).unwrap();

        res.push(
            Sensor {
                x,
                y,
                nearest_beacon_x: nx,
                nearest_beacon_y: ny,
                manhattan_distance: (x - nx).abs() + (y - ny).abs(),
            }
        );
    }

    res
}

fn find_positions_without_beacon(sensors: &Vec<Sensor>, forbidden_y: i64) -> usize {
    let mut forbidden_sensors_locations: HashSet<(i64, i64)> = HashSet::new();

    for sensor in sensors {
        let mut cursor = (sensor.x, forbidden_y);

        // going farther left we can
        loop {
            if sensor.can_be_beacon(cursor.0, cursor.1) {
               break; 
            }

            cursor.0 -= 1;
        }

        // now, going right from this point
        loop {
            cursor.0 += 1;
            if sensor.can_be_beacon(cursor.0, cursor.1) {
                break;
            }

            let mut cursor_has_beacon = false;
            // add the cursor only if there is not beacon in it
            for b in sensors {
                if cursor == (b.nearest_beacon_x, b.nearest_beacon_y) {
                    cursor_has_beacon = true;
                }
            }

            if !cursor_has_beacon {
                forbidden_sensors_locations.insert(cursor);
            }
        }
    }

    forbidden_sensors_locations.len()
}

fn find_distress(sensors: &Vec<Sensor>, max: i64) -> i64 {
    let mut can_have;
    let mut current = (0, 0);

    while current.1 <= max {
        while current.0 <= max {
            can_have = true;
            for sensor in sensors {
                let diff = sensor.manhattan_diff(current.0, current.1);
                /*
                println!("Adding distance: {} to {:?} on sensor {:?} ~ {}",
                    diff,
                    current,
                    sensor,
                    sensor.manhattan_distance(current.0, current.1)
                );
                 */
                if diff != 0 {
                    current.0 += diff;
                    can_have = false;
                } else {
                    if !sensor.can_be_beacon(current.0, current.1) {
                        can_have = false;
                    }
                }
            }

            if !can_have {
                current.0 += 1;
                continue;
            }

            if current.0 <= max {
                // println!("{:?} can:{}", current, can_have);
                return current.0 * 4000000 + current.1;
            }

            return 0;
        }
        current.0 = 0;
        current.1 += 1;
    }

    0
}

fn main() {
    let sensors = parse("input.txt");

    println!("#1 {}", find_positions_without_beacon(&sensors, 2000000));
    println!("#2 {}", find_distress(&sensors, 4000000));
}

#[test]
fn test_sample() {
    let sensors = parse("input.txt_test");
    assert_eq!(
        26,
        find_positions_without_beacon(&sensors, 10)
    );

    assert_eq!(
        56000011,
        find_distress(&sensors, 20)
    );
}
