use pathfinding::prelude::bfs;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Position(String);

impl Position {
    fn successors(&self) -> Vec<Position> {
        let mut out : Vec<Position> = vec![];

        let digest_str = format!("{:x}", md5::compute(&self.0));

        for (idx, v) in digest_str.as_bytes().iter().take(4).enumerate() {
            if *v < 'b' as u8 || *v > 'f' as u8 {
                continue;
            }

            let mut n = self.0.clone();
            n.push(match idx {
                0 => 'U',
                1 => 'D',
                2 => 'L',
                3 => 'R',
                _ => unreachable!()
            });

            let pos = Position(n);

            if let Ok((_i, _j)) = pos.indices() {
                out.push(pos);
            }
        }

        out
    }

    fn indices(&self) -> Result<(usize, usize), ()> {
        let mut i : i32 = 0;
        let mut j : i32 = 0;

        self.0.chars().map(|c| match c {
            'L' => i -= 1,
            'R' => i += 1,
            'U' => j -= 1,
            'D' => j += 1,
            _ => {},
        }).count();

        if i < 0 || i > 3 || j < 0 || j > 3 {
            return Err(());
        }

        Ok((i as usize, j as usize))
    }

    fn is_goal(&self) -> bool {
        Ok((3, 3)) == self.indices()
    }
}

fn compute(passcode: &str) -> String {
    let _result = bfs(
        &Position(String::from(passcode)),
        |p| p.successors(),
        |p| p.is_goal()
    );

    let pathes = _result.unwrap();
    let lastpath = &pathes[pathes.len()-1].0;

    String::from(lastpath.get(passcode.len()..).unwrap())
}

fn main() {
    let passcode = "gdjjyniy";

    println!("Part #1: {}", compute(passcode));

    let mut pathes = vec![];
    let mut longestpath = Position("".to_string());

    pathes.push(Position(passcode.to_string()));

    loop {
        if pathes.len() == 0 {
            break;
        }

        let current = pathes.pop().unwrap();
        for successor in current.successors() {
            if successor.is_goal() {
                if successor.0.len() > longestpath.0.len() {
                    longestpath = successor;
                }
            } else {
                pathes.push(successor);
            }
        }
    }

    println!("Part #2: {}", String::from(longestpath.0.get(passcode.len()..).unwrap()).len());
}

#[test]
fn indices() {
    assert_eq!(Ok((0, 0)), Position(String::from("hijkl")).indices());
    assert_eq!(Ok((0, 0)), Position(String::from("hijklRL")).indices());
    assert_eq!(Ok((0, 1)), Position(String::from("hijklRLD")).indices());

    assert_eq!(String::from("DDRRRD"), compute("ihgpwlah"));
    assert_eq!(String::from("DDUDRLRRUDRD"), compute("kglvqrro"));
    assert_eq!(String::from("DRURDRUDDLLDLUURRDULRLDUUDDDRR"), compute("ulqzkmiv"));
}
