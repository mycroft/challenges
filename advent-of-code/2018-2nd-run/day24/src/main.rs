use std::cmp;
use std::fmt;
use std::hash::{Hash, Hasher};

use std::collections::HashMap;

use scan_fmt::scan_fmt;
use regex::Regex;

extern crate scan_fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum EntityType {
    ImmuneSystem,
    Infection,
}

impl fmt::Display for EntityType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            self 
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum DamageType {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

impl fmt::Display for DamageType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}",
            self 
        )
    }
}

#[derive(Clone, Debug)]
struct Entity {
    id: usize,
    entity_type: EntityType,
    unit_number: i32,
    hit_points: i32,
    damage: i32,
    damage_type: DamageType,
    immunities: Vec<DamageType>,
    weaknesses: Vec<DamageType>,
    initiative: i32,
}

impl Hash for Entity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.entity_type.hash(state);
    }
}

impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.entity_type == other.entity_type
    }
}

impl Eq for Entity {}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: Group {} {} units each with {} hit points (weak to: {:?}; immune to: {:?}) with an attack that does {} {} damage at initiative {} (effective power: {})",
            self.entity_type,
            self.id,
            self.unit_number,
            self.hit_points,
            self.weaknesses,
            self.immunities,
            self.damage,
            self.damage_type,
            self.initiative,
            self.clone().effective_power(),
        )
    }
}

impl Entity {
    fn effective_power(self: Entity) -> i32 {
        self.unit_number * self.damage
    }

    fn would_deal(self: Entity, target: Entity) -> i32 {
        let mut multiply = 1;
        if target.immunities.contains(&self.damage_type) {
            return 0;
        }

        if target.weaknesses.contains(&self.damage_type) {
            multiply = 2;
        }

        multiply * self.effective_power()
    }
}


fn main() {
    let contents = std::fs::read_to_string("input.txt").expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut immune_id = 0;
    let mut infection_id = 0;
    let mut is_immune = false;

    let mut entities: Vec<Entity> = vec![];

    for &line in &lines {
        let line = line.trim_end();

        if line.is_empty() {
            continue;
        }

        if line == "Immune System:" {
            is_immune = true;
            continue;
        } else if line == "Infection:" {
            is_immune = false;
            continue;
        }

        let (unit_number, hit_points, immune_weak, damage, damage_type, initiative) = scan_fmt!(
            line,
            "{} units each with {} hit points {/.*with/} an attack that does {/[0-9]+/} {} damage at initiative {}",
            i32, i32, String, i32, String, i32
        ).unwrap();

        let (entity_type, id) = if is_immune {
            immune_id += 1;
            (EntityType::ImmuneSystem, immune_id)
        } else {
            infection_id += 1;
            (EntityType::Infection, infection_id)
        };

        let entity = Entity {
            id: id,
            entity_type,
            unit_number,
            hit_points,
            damage,
            damage_type: to_damage_type(damage_type),
            weaknesses: str_to(&immune_weak, true),
            immunities: str_to(&immune_weak, false),
            initiative,
        };

        entities.push(entity);
    }

    // Remaining units
    let (immune_count, infection_count) = play(entities.clone(), 0);
    println!("#1 {}", immune_count + infection_count);

    let (mut m, mut n) = (0, 10000);

    while m != n {
        let z = m + (n - m + 1) / 2;
        let entities_cpy = entities.clone();

        let (_immune_count, infection_count) = play(entities_cpy, z);
        if infection_count != 0 {
            m = z;
        } else {
            n = z;
        }

        if n - m <= 100 {
            break;
        }
    }

    for a in m..=n {
        let entities_cpy = entities.clone();

        let res = play(entities_cpy, a);
        if res.1 == 0 {
            println!("#2 {}", res.0);
            break;
        }
    }
}

fn play(entities: Vec<Entity>, boost: i32) -> (i32, i32) {
    let mut entities = entities;
    let debug = false;
/*
During the target selection phase, each group attempts to choose one target.
In decreasing order of effective power, groups choose their targets; in a tie,
the group with the higher initiative chooses first. The attacking group chooses
to target the group in the enemy army to which it would deal the most damage
(after accounting for weaknesses and immunities, but not accounting for whether
the defending group has enough units to actually receive all of that damage).

If an attacking group is considering two defending groups to which it would deal
equal damage, it chooses to target the defending group with the largest effective
power; if there is still a tie, it chooses the defending group with the highest
initiative. If it cannot deal any defending groups damage, it does not choose
a target. Defending groups can only be chosen as a target by one attacking group.
*/

    for n in 0..entities.len() {
        if entities[n].entity_type == EntityType::ImmuneSystem {
            entities[n].damage += boost;
        }
    }

    loop {
        let mut killed_total = 0;

        if debug {
            println!("-- New turn --");

            for n in &entities {
                println!("{} {} has {} units", n.entity_type, n.id, n.unit_number);
            }
        }


        // selection phase
        entities.sort_by(|a, b| {
            match b.clone().effective_power().cmp(&a.clone().effective_power()) {
                cmp::Ordering::Equal => b.clone().initiative.cmp(&a.clone().initiative),
                x => x,
            }
        });

        let mut targets: HashMap<Entity, Entity> = HashMap::new();

        for n in 0..entities.len() {
            let mut possible_targets = vec![];

            for entity in &entities {
                if entity.unit_number <= 0 {
                    continue;
                }

                if entities[n].entity_type == entity.entity_type {
                    continue;
                }

                if targets.iter().map(|(_, v)| v).any(|x| x == entity) {
                    continue;
                }

                if entities[n].clone().would_deal(entity.clone()) == 0 {
                    continue;
                }

                possible_targets.push(entity);
            }

            possible_targets.sort_by(|&a, &b| {
                match entities[n].clone().would_deal(b.clone()).cmp(&entities[n].clone().would_deal(a.clone())) {
                    cmp::Ordering::Equal => {
                        // damage are equal. So, max effective
                        match b.clone().effective_power().cmp(&a.clone().effective_power()) {
                            cmp::Ordering::Equal => {
                                b.clone().initiative.cmp(&a.clone().initiative)
                            },
                            x => x,
                        }
                    },
                    x => x,
                }
            });

            if possible_targets.len() == 0 {
                continue;
            }

            targets.insert(entities[n].clone(), possible_targets[0].clone());
        }

        // attack phase

/*
During the attacking phase, each group deals damage to the target it selected, if any.
Groups attack in decreasing order of initiative, regardless of whether they are part of
the infection or the immune system. (If a group contains no units, it cannot attack.)

The damage an attacking group deals to a defending group depends on the attacking group's
attack type and the defending group's immunities and weaknesses. By default, an attacking
group would deal damage equal to its effective power to the defending group. However, if the
defending group is immune to the attacking group's attack type, the defending group instead takes
no damage; if the defending group is weak to the attacking group's attack type, the defending
group instead takes double damage.

The defending group only loses whole units from damage; damage is always dealt in such a way
that it kills the most units possible, and any remaining damage to a unit that does not immediately
kill it is ignored. For example, if a defending group contains 10 units with 10 hit points each and
receives 75 damage, it loses exactly 7 units and is left with 3 units at full health.
*/

        entities.sort_by(|a, b| {
            b.clone().initiative.cmp(&a.clone().initiative)
        });

        for n in 0..entities.len() {
            let attacker = entities[n].clone();
            let defender = if targets.contains_key(&attacker) {
                targets[&attacker].clone()
            } else {
                continue;
            };

            let dealt_damage = attacker.clone().would_deal(defender.clone());
            let killed = cmp::min(
                (dealt_damage - dealt_damage%defender.hit_points) / defender.hit_points,
                defender.unit_number
            );

            killed_total += killed;

            for m in 0..entities.len() {
                if entities[m].id == defender.id && entities[m].entity_type == defender.entity_type {
                    entities[m].unit_number -= killed;

                    if entities[m].unit_number < 0 {
                        entities[m].unit_number = 0;
                    }
                }
            }

            if debug {
                println!("{} grp {} attacks {} grp {} dealing {} damage, killing {}.",
                    attacker.entity_type,
                    attacker.id,
                    defender.entity_type,
                    defender.id,
                    dealt_damage,
                    killed,
                );
            }
        }

        // final
        let (n, m) = (
            entities.iter().filter(|x| x.entity_type == EntityType::ImmuneSystem && x.unit_number > 0).count(),
            entities.iter().filter(|x| x.entity_type == EntityType::Infection && x.unit_number > 0).count()
        );

        if n == 0 || m == 0 || killed_total == 0 {
            break;
        }
    }

    (
        entities.iter().filter(|x| x.entity_type == EntityType::ImmuneSystem).map(|x| x.unit_number).sum::<i32>(),
        entities.iter().filter(|x| x.entity_type == EntityType::Infection).map(|x| x.unit_number).sum::<i32>(),
    )
}

fn str_to(s: &str, is_weak: bool) -> Vec<DamageType> {
    let mut search_for = "immune";
    if is_weak {
        search_for = "weak";
    }
        
    let re = Regex::new(format!(".*{} to ([^;)]*)", search_for).as_str()).unwrap();
    let caps = re.captures(s);

    if caps.is_none() {
        return vec![];
    }

    let caps = caps.unwrap();

    caps[1]
        .split(',')
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| to_damage_type(x.trim().to_string()))
        .collect::<Vec<DamageType>>()
}

fn to_damage_type(s: String) -> DamageType {
    match s.as_str() {
        "bludgeoning" => DamageType::Bludgeoning,
        "cold" => DamageType::Cold,
        "fire" => DamageType::Fire,
        "radiation" => DamageType::Radiation,
        "slashing" => DamageType::Slashing,
        _ => unimplemented!(),
    }
}