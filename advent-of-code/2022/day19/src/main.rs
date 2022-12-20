use std::fs::read_to_string;
use std::collections::HashMap;

#[macro_use] extern crate scan_fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Balance {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
}

impl Balance {
    fn new() -> Self {
        Balance{ ore: 0, clay: 0, obsidian: 0, geode: 0}
    }

    fn can_buy(&self, p: Balance) -> bool {
        self.ore >= p.ore && self.clay >= p.clay && self.obsidian >= p.obsidian
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Blueprint {
    num: usize,
    ore: Balance,
    clay: Balance,
    obsidian: Balance,
    geode: Balance,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct BotInventory {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize
}

impl BotInventory {
    fn new() -> Self {
        BotInventory { ore: 1, clay: 0, obsidian: 0, geode: 0 }
    }

}

impl Blueprint {
    fn buy_robots(&self, balance: &Balance, bots: &BotInventory) -> Vec<(BotInventory, Balance)> {
        let mut result = Vec::new();


        // optimization: if we can build a geode robot, skip others.
        if balance.can_buy(self.geode) {
            result.push(
                (
                    BotInventory { ore: bots.ore, clay: bots.clay, obsidian: bots.obsidian, geode: bots.geode + 1},
                    Balance { ore: balance.ore - self.geode.ore, clay: balance.clay, obsidian: balance.obsidian - self.geode.obsidian, geode: balance.geode }
                )
            );

            return result;
        }

        // optimization: buy robots only if we do not have enough (ie bot number < required ore): No point having robots mining for nothing
        if balance.can_buy(self.ore) && (bots.ore < *[self.ore.ore, self.clay.ore, self.obsidian.ore, self.geode.ore].iter().max().unwrap()) {
            result.push(
                (
                    BotInventory { ore: bots.ore + 1, clay: bots.clay, obsidian: bots.obsidian, geode: bots.geode },
                    Balance { ore: balance.ore - self.ore.ore, clay: balance.clay, obsidian: balance.obsidian, geode: balance.geode }
                )
            );
        }

        if balance.can_buy(self.clay) && (bots.clay < *[self.ore.clay, self.clay.clay, self.obsidian.clay, self.geode.clay].iter().max().unwrap()) {
            result.push(
                (
                    BotInventory { ore: bots.ore, clay: bots.clay + 1, obsidian: bots.obsidian, geode: bots.geode },
                    Balance { ore: balance.ore - self.clay.ore, clay: balance.clay, obsidian: balance.obsidian, geode: balance.geode }
                )
            );
        }

        if balance.can_buy(self.obsidian) && (bots.obsidian < *[self.ore.obsidian, self.clay.obsidian, self.obsidian.obsidian, self.geode.obsidian].iter().max().unwrap()) {
            result.push(
                (
                    BotInventory { ore: bots.ore, clay: bots.clay, obsidian: bots.obsidian + 1, geode: bots.geode },
                    Balance { ore: balance.ore - self.obsidian.ore, clay: balance.clay - self.obsidian.clay, obsidian: balance.obsidian, geode: balance.geode }
                )
            );
        }

        // optimization: do not accumulate too many ores without spending those.
        if balance.ore < 5 {
            result.push((bots.clone(), balance.clone()));
        }

        result
    }
}

impl Balance {
    fn increment(&self, bots: &BotInventory) -> Balance {
        // println!("{}+{}={}", self.ore, bots.ore, self.ore + bots.ore);
        Balance {
            ore: self.ore + bots.ore,
            clay: self.clay + bots.clay,
            obsidian: self.obsidian + bots.obsidian,
            geode: self.geode + bots.geode,
        }
    }
}


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct State{
    blueprint_num: usize,
    bots_inventory: BotInventory,
    balance: Balance,
    remaning: usize
}

fn parse(fp: &str) -> Vec<Blueprint> {
    let contents = read_to_string(fp).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut result = Vec::new();

    for line in lines {
        let (num, ore_price, clay_price, obsidian_price_ore, obsidian_price_clay, geode_price_ore, geode_price_obsidian) = scan_fmt!(
            line,
            "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.",
            usize, usize, usize, usize, usize, usize, usize
        ).unwrap();

        result.push(
            Blueprint {
                num,
                ore: Balance { ore: ore_price, clay: 0, obsidian: 0, geode: 0 },
                clay: Balance { ore: clay_price, clay: 0, obsidian: 0, geode: 0 },
                obsidian: Balance { ore: obsidian_price_ore, clay: obsidian_price_clay, obsidian: 0, geode: 0 },
                geode: Balance { ore: geode_price_ore, clay: 0, obsidian: geode_price_obsidian, geode: 0 },
            }
        );
    }

    result
}


// for a given state, return the max geodes according what we can buy
fn run(cache: &mut HashMap<State, usize>, b: &Blueprint, bots: BotInventory, balance: Balance, remaining: usize) -> usize {
    if remaining == 0 {
        return balance.geode
    }
    
    let current_state = State { blueprint_num: b.num, bots_inventory: bots, balance: balance, remaning: remaining };

    if cache.contains_key(&current_state) {
        return *cache.get(&current_state).unwrap();
    }

    let next_scenarios = b.buy_robots(&balance, &bots);
    
    // Mine stuff with existing robots
    // let balance = balance.increment(&bots);

    // println!("remaining: {} scenarios:{} inventory:{:?} balance:{:?}", remaining, next_scenarios.len(), bots, balance);

    // find the max geode result
    let mut max_geode = balance.geode;
    for next_scenario in next_scenarios {
        let balance = next_scenario.1.increment(&bots);

        let result = run(
            cache,
            b,
            next_scenario.0,
            balance,
            remaining - 1,
        );

        if result > max_geode {
            max_geode = result;
        }
    }

    // Adding the state in the cache.
    cache.insert(
        current_state,
        max_geode,
    );

    max_geode
}

fn run_blueprint(b: &Blueprint, remaining: usize) -> usize{
    let mut known_states = HashMap::new();

    run(
        &mut known_states,
        b,
        BotInventory::new(),
        Balance::new(),
        remaining
    )
}

fn run_step1(fp: &str) -> usize {
    let blueprints = parse(fp);

    blueprints.iter()
        .map(|b| {
            b.num * run_blueprint(b, 24)
        })
        .sum()
}


fn run_step2(fp: &str) -> usize {
    let blueprints = parse(fp);

    (0..3).map(|idx| {
        run_blueprint(&blueprints[idx], 32)
    }).product()
}

fn main() {
    // Runs in 7 seconds in total on my workstation.
    println!("#1 {}", run_step1("input.txt")); // 1349
    println!("#2 {}", run_step2("input.txt")); // 21840
}

// Tests are running in 15 seconds.
#[test]
fn test_step1() {
    assert_eq!(
        33,
        run_step1("input.txt_test")
    );
}

#[test]
fn test_step2() {
    let blueprints = parse("input.txt_test");

    assert_eq!(
        56,
        run_blueprint(&blueprints[0], 32)
    );

    assert_eq!(
        62,
        run_blueprint(&blueprints[1], 32)
    );
}
