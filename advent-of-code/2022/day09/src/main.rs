use std::fs::read_to_string;
use std::collections::HashSet;

fn is_touching(a: (i32, i32), b: (i32, i32)) -> bool {
    return (a.0 - b.0).abs() < 2 && (a.1 - b.1).abs() < 2
}

fn run_step1(fp: &str) -> usize {
    let contents = read_to_string(fp).unwrap();
    let moves: Vec<Vec<&str>> = contents.lines().map(|x| x.split(" ").collect()).collect();

    let mut head= (0i32, 0i32);
    let mut tail = head;

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(head);

    for mov in moves {
        let direction = mov[0];
        let steps = mov[1].parse::<i32>().unwrap();

        for _ in 0..steps {
            let previous_head_pos = head;

            head = match direction {
                "D" => (head.0, head.1 + 1),
                "R" => (head.0 + 1, head.1),
                "U" => (head.0, head.1 - 1),
                "L" => (head.0 - 1, head.1),
                _ => panic!("impossible case")
            };

            if !is_touching(tail, head) {
                // Might be incorrect but still working:
                tail = previous_head_pos;
                if !visited.contains(&tail) {
                    visited.insert(tail);
                }
            }
        }
    }

    visited.len()
}

fn run_step2(fp: &str) -> usize {
    let contents = read_to_string(fp).unwrap();
    let moves: Vec<Vec<&str>> = contents.lines().map(|x| x.split(" ").collect()).collect();

    let mut rope: Vec<(i32, i32)> = [(0i32, 0i32); 10].to_vec();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(rope[rope.len() - 1]);


    for mov in moves {
        let direction = mov[0];
        let steps = mov[1].parse::<i32>().unwrap();

        for _ in 0..steps {
            rope[0] = match direction {
                "D" => (rope[0].0, rope[0].1 + 1),
                "R" => (rope[0].0 + 1, rope[0].1),
                "U" => (rope[0].0, rope[0].1 - 1),
                "L" => (rope[0].0 - 1, rope[0].1),
                _ => panic!("impossible case")
            };

            for i in 0..rope.len() - 1{
                let head_x = rope[i].0;
                let head_y = rope[i].1;

                let tail_x = rope[i + 1].0;
                let tail_y = rope[i + 1].1;

                let dist_x = head_x - tail_x;
                let dist_y = head_y - tail_y;

                if dist_x.abs() >= 2 || dist_y.abs() >= 2 {
                    let dir_x = if dist_x < 0 { -1 } else if dist_x == 0 { 0 } else { 1 };
                    let dir_y = if dist_y < 0 { -1 } else if dist_y == 0 { 0 } else { 1 };
                    rope[i + 1] = (
                        tail_x + dir_x,
                        tail_y + dir_y
                    );
                }
            }

            if !visited.contains(&rope[rope.len() - 1]) {
                visited.insert(rope[rope.len() - 1]);
            }
        }
    }

    visited.len()

}

fn main() {
    println!("#1 {}", run_step1("input.txt")); // 5710
    println!("#2 {}", run_step2("input.txt")); // 2259
}

#[test]
fn test_step1() {
    assert_eq!(13, run_step1("input.txt_test"));
}

#[test]
fn test_step2() {
    assert_eq!(1, run_step2("input.txt_test"));
    assert_eq!(36, run_step2("input.txt_test2"));
}