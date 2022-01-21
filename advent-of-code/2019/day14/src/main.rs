use std::str::FromStr;
use std::collections::{HashMap, VecDeque};
use scan_fmt::parse::ScanError;

#[macro_use] extern crate scan_fmt;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct ElementQuantity {
    n: usize,
    el: String
}

impl FromStr for ElementQuantity {
    type Err = ScanError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match scan_fmt!(
            s,
            "{} {}",
            usize, String
        ) {
            Err(x) => Err(x),
            Ok((n, l)) => Ok(ElementQuantity{n, el: l})
        }
    }
}

fn load(fp: &str) -> Result<HashMap<ElementQuantity, Vec<ElementQuantity>>, ScanError> {
    let contents = std::fs::read_to_string(fp).expect("file to open");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut hm = HashMap::new();

    for line in lines {
        let line = line.trim_end();
        if line.is_empty() {
            break;
        }

        let parts = line.split(" => ").collect::<Vec<&str>>();
        
        let r: ElementQuantity = ElementQuantity::from_str(parts[1])?;

        let l: Vec<ElementQuantity> = parts[0]
            .split(", ")
            .map(ElementQuantity::from_str)
            .filter(|r| r.is_ok())
            .map(|r| r.unwrap())
            .collect();

        hm.insert(r, l);
    }

    Ok(hm)
}

fn solve(rules: &HashMap<ElementQuantity, Vec<ElementQuantity>>, fuel: usize) -> usize {
    let mut supply : HashMap<String, usize> = HashMap::new();
    let mut orders : VecDeque<ElementQuantity> = VecDeque::new();

    let mut ore_needed = 0;

    orders.push_back(ElementQuantity{n: fuel, el: String::from("FUEL")});

    while !orders.is_empty() {
        let order = orders.pop_front().unwrap();

        if order.el == *"ORE" {
            ore_needed += order.n;
        } else if order.n <= *supply.get(&order.el).or(Some(&0)).unwrap() {
            *supply.entry(order.el).or_insert(0) -= order.n;
        } else {
            let amount_needed = order.n - *supply.get(&order.el).or(Some(&0)).unwrap();
            
            let recipe = rules
                .iter()
                .find(|(c, _)| c.el == order.el)
                .unwrap();

            let batches = (amount_needed as f64 / recipe.0.n as f64).ceil() as usize;

            for ingredient in recipe.1 {
                orders.push_back(ElementQuantity{el: ingredient.el.clone(), n: ingredient.n * batches});
            }

            let leftover_amount = batches * recipe.0.n - amount_needed;
            *supply.entry(order.el).or_insert(0) = leftover_amount;
        }
    }

    ore_needed
}

fn resolve1(fp: &str) -> usize {
    let rules = load(fp).expect("rules");

    solve(&rules, 1)
}

fn resolve2(fp: &str) -> usize {
    let ore_limit = 1000000000000;
    let mut min = 1;
    let mut max = 1000000000000;

    let rules = load(fp).expect("rules");

    while min != max - 1 {
        let z = min + (max - min) / 2;
        let r = solve(&rules,  z);

        if r > ore_limit {
            max = z;
        } else {
            min = z;
        }
    }

    min
}

fn main() {
    println!("#1 {}", resolve1("input.txt"));
    println!("#2 {}", resolve2("input.txt"));
}

#[test]
fn test_1() {
    assert_eq!(165, resolve1("input.txt_test0"));
    assert_eq!(13312, resolve1("input.txt_test1"));
    assert_eq!(180697, resolve1("input.txt_test2"));
    assert_eq!(2210736, resolve1("input.txt_test3"));
    assert_eq!(31, resolve1("input.txt_test4"));
}

#[test]
fn test_2() {
    assert_eq!(82892753, resolve2("input.txt_test1"));
    assert_eq!(5586022, resolve2("input.txt_test2"));
    assert_eq!(460664, resolve2("input.txt_test3"));
}
