use std::cmp::max;

#[derive(Copy, Clone, PartialEq, Debug)]
enum EffectType {
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug)]
struct Effect {
    turns: i8,
    effect_type: EffectType,
}

fn turn<'a>(
        is_hard: bool,
        is_player: bool, 
        hp: i16, 
        mana: i16, 
        boss_hp: i16, 
        boss_dmg: i16,
        effects: &Vec<Effect>,
        current_cost: i16,
        found: &'a mut i16) -> (bool, i16) {

    let mut player_armor = 0;
    let mut boss_hp = boss_hp;
    let mut mana = mana;

    // hard mode:
    let mut hp = hp;

    if is_player && is_hard {
        hp -= 1;

        if hp == 0 {
            return (false, 0);
        }
    }

    let mut new_effects : Vec<Effect> = vec![];

    // run effects
    for effect in effects {
        match &effect.effect_type {
            EffectType::Shield => {
                player_armor += 7;
            },
            EffectType::Poison => {
                boss_hp -= 3;
            },
            EffectType::Recharge => {
                mana += 101;
            }
        }

        if effect.turns - 1 > 0 {
            let effect = Effect {
                turns: effect.turns - 1,
                effect_type: effect.effect_type
            };
            
            new_effects.push(effect);
        }
    }

    if *found != 0 && current_cost > *found {
        return (false, 0);
    }

    if boss_hp <= 0 {
        return (true, current_cost);
    }

    let mut has_win = false;
    let mut min_cost = 0;

    // XXX to rework
    let spells = [53, 73, 113, 173, 229];

    if is_player {
        for spell in 0..5 {
            let mut must_continue = false;

            if mana < spells[0] && spell == 0 {
                return (false, 0);
            }

            if mana < spells[spell] {
                continue;
            }

            // XXX to rework
            for effect in &new_effects {
                if effect.effect_type == EffectType::Shield && spell == 2 {
                    must_continue = true
                }
                if effect.effect_type == EffectType::Poison && spell == 3 {
                    must_continue = true
                }
                if effect.effect_type == EffectType::Recharge && spell == 4 {
                    must_continue = true
                }
            }

            if must_continue {
                continue;
            }

            let (win, cost) = match spell {
                0 => {
                    turn(is_hard, false, hp, mana - spells[spell], boss_hp - 4, boss_dmg, &new_effects, current_cost + spells[spell], found)
                },
                1 => {
                    turn(is_hard, false, hp + 2, mana - spells[spell], boss_hp - 2, boss_dmg, &new_effects, current_cost + spells[spell], found)
                },
                2 => {
                    new_effects.push(Effect{turns: 6, effect_type: EffectType::Shield});
                    let (win, cost) = turn(is_hard, false, hp, mana - spells[spell], boss_hp, boss_dmg, &new_effects, current_cost + spells[spell], found);
                    new_effects.pop();

                    (win, cost)
                },
                3 => {
                    new_effects.push(Effect{turns: 6, effect_type: EffectType::Poison});
                    let (win, cost) = turn(is_hard, false, hp, mana - spells[spell], boss_hp, boss_dmg, &new_effects, current_cost + spells[spell], found);
                    new_effects.pop();

                    (win, cost)

                },
                4 => {
                    new_effects.push(Effect{turns: 5, effect_type: EffectType::Recharge});
                    let (win, cost) = turn(is_hard, false, hp, mana - spells[spell], boss_hp, boss_dmg, &new_effects, current_cost + spells[spell], found);
                    new_effects.pop();

                    (win, cost)

                },
                _ => (false, 0)
            };

            if win {
                if !has_win {
                    has_win = true;
                    min_cost = cost;
                }
                if cost < min_cost {
                    min_cost = cost;
                }

                *found = min_cost.clone();
            }
        }

/*
Magic Missile costs 53 mana. It instantly does 4 damage.

Drain costs 73 mana. It instantly does 2 damage and heals you for 2 hit points.

Shield costs 113 mana. It starts an effect that lasts for 6 turns.
While it is active, your armor is increased by 7.

Poison costs 173 mana. It starts an effect that lasts for 6 turns.
At the start of each turn while it is active, it deals the boss 3 damage.

Recharge costs 229 mana. It starts an effect that lasts for 5 turns.
At the start of each turn while it is active, it gives you 101 new mana.
*/

        (has_win, min_cost)  
    } else {
        let player_hp = hp - max(boss_dmg - player_armor, 1);
        if player_hp <= 0 {
            return (false, current_cost);
        }

        turn(is_hard, true, player_hp, mana, boss_hp, boss_dmg, &new_effects, current_cost, found)
    }
}

fn main() {
    let hitpoints = 50;
    let mana = 500;

    let boss_hitpoints = 51;
    let boss_damage = 9;

    let mut found = 0;

    let (_win, cost) = turn(false, true, hitpoints, mana, boss_hitpoints, boss_damage, &vec![], 0, &mut found);
    println!("Part #1: {:?}", cost);


    let mut found = 0;

    let (_win, cost) = turn(true, true, hitpoints, mana, boss_hitpoints, boss_damage, &vec![], 0, &mut found);
    println!("Part #2: {:?}", cost);

}

#[test]
fn example() {
    let mut found = 0;
    assert_eq!((true, 226), turn(false, true, 10, 250, 13, 8, &vec![], 0, &mut found));

    let mut found = 0;
    assert_eq!((true, 641), turn(false, true, 10, 250, 14, 8, &vec![], 0, &mut found));
}
