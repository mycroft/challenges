#[derive(Debug)]
struct Weapon {
    cost: i16,
    damage: i16,
}

#[derive(Debug)]
struct Armor {
    cost: i16,
    armor: i16,
}

#[derive(Debug)]
struct Ring {
    cost: i16,
    damage: i16,
    armor: i16,
}

fn fight(armor: i16, damage: i16) -> bool {
    // input
    let mut boss_hitpoints = 109;
    let boss_damage = 8;
    let boss_armor = 2;

    let mut hitpoints = 100;

    loop {
        boss_hitpoints = boss_hitpoints - (damage - boss_armor);
        if boss_hitpoints <= 0 {
            return true;
        }

        hitpoints = hitpoints - (boss_damage - armor);
        if hitpoints <= 0 {
            return false;
        }
    }
}

fn main() {
    let mut weapons = vec![];
    weapons.push(Weapon { cost: 8, damage: 4 });
    weapons.push(Weapon {
        cost: 10,
        damage: 5,
    });
    weapons.push(Weapon {
        cost: 25,
        damage: 6,
    });
    weapons.push(Weapon {
        cost: 40,
        damage: 7,
    });
    weapons.push(Weapon {
        cost: 74,
        damage: 8,
    });

    let mut armors = vec![];
    armors.push(Armor { cost: 0, armor: 0 });
    armors.push(Armor { cost: 13, armor: 1 });
    armors.push(Armor { cost: 31, armor: 2 });
    armors.push(Armor { cost: 53, armor: 3 });
    armors.push(Armor { cost: 75, armor: 4 });
    armors.push(Armor {
        cost: 102,
        armor: 5,
    });

    let mut rings = vec![];
    rings.push(Ring {
        cost: 25,
        damage: 1,
        armor: 0,
    });
    rings.push(Ring {
        cost: 50,
        damage: 2,
        armor: 0,
    });
    rings.push(Ring {
        cost: 100,
        damage: 3,
        armor: 0,
    });
    rings.push(Ring {
        cost: 20,
        damage: 0,
        armor: 1,
    });
    rings.push(Ring {
        cost: 40,
        damage: 0,
        armor: 2,
    });
    rings.push(Ring {
        cost: 80,
        damage: 0,
        armor: 3,
    });
    rings.push(Ring {
        cost: 0,
        damage: 0,
        armor: 0,
    });
    rings.push(Ring {
        cost: 0,
        damage: 0,
        armor: 0,
    });

    let ring_number = rings.len();

    let mut min_cost = 0;
    let mut max_cost = 0;

    for weapon in weapons {
        for armor in &armors {
            for ring1 in 0..ring_number {
                for ring2 in 0..ring_number {
                    if ring1 == ring2 {
                        continue;
                    }

                    let cost = weapon.cost + armor.cost + rings[ring1].cost + rings[ring2].cost;
                    let armor_val = armor.armor + rings[ring1].armor + rings[ring2].armor;
                    let damage = weapon.damage + rings[ring1].damage + rings[ring2].damage;

                    let win = fight(armor_val, damage);

                    if win && (min_cost == 0 || min_cost > cost) {
                        min_cost = cost;
                    }

                    if !win && cost > max_cost {
                        max_cost = cost;
                    }
                }
            }
        }
    }

    println!("Part #1: {}", min_cost);
    println!("Part #2: {}", max_cost);
}
