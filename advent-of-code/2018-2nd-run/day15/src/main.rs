/*
 * AOC 2018 - 15
 */
use std::cmp::Ordering;
use std::fs;
use pathfinding::directed::bfs::bfs;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Race {
    Elf,
    Goblin,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Pos(i32, i32);

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
struct Character {
    race: Race,
    hits: usize,
    alive: bool,
    position: Pos,
}

impl Pos {
    fn successors(&self, walls: &Vec<Pos>, players: &Vec<Pos>) -> Vec<Pos> {
        let deltas = [(0,-1),(-1,0),(1,0),(0,1)];

        let &Pos(x, y) = self;
        deltas
            .iter()
            .map(|delta|
                Pos(x + delta.0, y + delta.1)
            )
            .filter(|p| !walls.contains(p) && !players.contains(p))
            .collect()
    }

    fn goal(&self, me: Character, players: &Vec<Character>) -> bool {
        let deltas = [(0,-1),(-1,0),(1,0),(0,1)];

        let dst = players
            .iter()
            .filter(|p| p.race != me.race && p.alive)
            .map(|p| {
                deltas.iter().map(|d| Pos(p.position.0 + d.0, p.position.1 + d.1)).collect::<Vec<Pos>>()
            })
            .flatten()
            .collect::<Vec<Pos>>();

        dst.contains(self)
    }
}

impl PartialOrd for Pos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.1 < other.1 || (self.1 == other.1 && self.0 < other.0) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}

impl Ord for Character {
    fn cmp(&self, other: &Self) -> Ordering {
        self.position.cmp(&other.position)
    }
}

impl PartialOrd for Character {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn display(map: &Vec<Vec<bool>>, players: &Vec<Character>) {
    for (y, line) in map.iter().enumerate() {
        println!("{}", 
            line
                .iter()
                .enumerate()
                .map(|(x, b)| if *b { '#' } else {
                    let mut c = '.';
                    for player in players {
                        if player.alive && player.position == Pos(x as i32, y as i32) {
                            c = match player.race {
                                Race::Elf => 'E',
                                Race::Goblin => 'G',
                            };
                        }
                    }
                    c
                })
                .collect::<String>()
        );
    }
}

fn has_target(player: Character, players: &Vec<Character>) -> Option<Character> {
    let mut target : Option<Character> = None;
    let deltas = [(0,-1),(-1,0),(0,1),(1,0)];

    for delta in deltas {
        let adj_pos = Pos(player.position.0 + delta.0, player.position.1 + delta.1);

        for i in 0..players.len() {
            if players[i].position != adj_pos || !players[i].alive || players[i].race == player.race {
                continue;
            }

            if target == None {
                target = Some(players[i]);
            } else if target.unwrap().hits > players[i].hits {
                target = Some(players[i]);
            }
        }
    }

    target
}

fn turn(walls: &Vec<Pos>, players: &mut Vec<Character>, elf_attack_power: usize) -> (bool, bool) {
    // sort players.
    players.sort();

    let dead = players.iter().enumerate().filter(|(_, p)| !p.alive).map(|(idx, _)| idx).collect::<Vec<usize>>();

    for i in 0..players.len() {
        if dead.contains(&i) || players[i].hits == 0 {
            continue;
        }

        let count = (
            players.iter().enumerate().filter(|(_, p)| p.race == Race::Elf && p.alive).count(),
            players.iter().enumerate().filter(|(_, p)| p.race == Race::Goblin && p.alive).count(),
        );

        if count.0 == 0 || count.1 == 0 {
            return (true, count.1 == 0 && count.0 == players.iter().enumerate().filter(|(_, p)| p.race == Race::Elf).count());
        }

        let mut player = players[i];
        let target = has_target(player, &players);
        if target == None {
            // This character needs to move.
            let result = bfs(
                &player.position,
                |p| p.successors(walls, &players.iter().filter(|p| p.alive).map(|p| p.position).collect::<Vec<Pos>>()),
                |p| p.goal(player, &players)
            );

            // println!("{:?}", result);
            if let Some(moves) = result {
                // println!("PLAYER {:?} IS MOVING TO: {:?}", player, moves[1]);
                player.position = moves[1];
                players[i] = player;
            }
        }

        let target = has_target(player, &players);
        if let Some(target) = target {
            // Need to attack
            // println!("PLAYER {:?} CAN TARGET: {:?}", player, target);

            let target_id = players.iter().position(|&p| p == target).unwrap();
            let attack_power = match player.race{
                Race::Elf => elf_attack_power,
                Race::Goblin => 3,
            };

            if players[target_id].hits >= attack_power {
                players[target_id].hits -= attack_power;
            } else {
                players[target_id].hits = 0;
            }

            if players[target_id].hits == 0 {
                players[target_id].alive = false;
                
                if players[target_id].race == Race::Elf && elf_attack_power > 3 {
                    // End game.
                    return (true, false);
                }
            }
        }
    }

    (false, false)
}

fn play(filepath: &str) -> (usize, usize) {
    let contents = fs::read_to_string(filepath).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut characters_orig : Vec<Character> = vec![];
    let mut walls : Vec<Vec<bool>> = vec![];
    let mut walls_coord : Vec<Pos> = vec![];

    for (y, line) in lines.iter().enumerate() {
        let mut wall_line : Vec<bool> = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == 'G' || c == 'E' {
                characters_orig.push(Character {
                    alive: true,
                    race: match c {
                        'E' => Race::Elf,
                        'G' => Race::Goblin,
                        _ => unreachable!(),
                    },
                    hits: 200,
                    position: Pos(x as i32, y as i32),
                });
            }

            wall_line.push(c == '#');
            if c == '#' {
                walls_coord.push(Pos(x as i32, y as i32));
            } 
        }

        walls.push(wall_line);
    }

    let result_1;
    let mut result_2 = 0;

    let mut round = 0;

    let mut characters = characters_orig.clone();

    // First run: Elf has an attack power of 3.
    loop {
        let (finished, _) = turn(&walls_coord, &mut characters, 3);

        if finished {
            result_1 = round * characters.iter().map(|p| p.hits).sum::<usize>();
            break;
        }

        round += 1;
    }

    // Second run: Elf has an AP of 4+.
    let mut attack_power = 4;

    loop {
        let mut round = 0;
        let mut characters = characters_orig.clone();

        loop {
            let (finished, elf_victory) = turn(&walls_coord, &mut characters, attack_power);

            if finished {
                if elf_victory {
                    result_2 = round * characters.iter().map(|p| p.hits).sum::<usize>();
                }
                break;
            }
            round += 1;
        }

        if result_2 > 0 {
            break;
        }

        attack_power += 1
    }

    (result_1, result_2)
}

fn main() {
    let res = play("input.txt");
    println!("#1: {}", res.0);
    println!("#2: {}", res.1);
}

#[test]
fn test2() {
    assert_eq!((27730, 4988), play("input.txt_test2"));
}

#[test]
fn test3() {
    assert_eq!((36334, 29064), play("input.txt_test3"));
}

#[test]
fn test4() {
    assert_eq!((39514, 31284), play("input.txt_test4"));
}

#[test]
fn test5() {
    assert_eq!((27755, 3478), play("input.txt_test5"));
}

#[test]
fn test6() {
    assert_eq!((28944, 6474), play("input.txt_test6"));
}

#[test]
fn test7() {
    assert_eq!((18740, 1140), play("input.txt_test7"));
}
