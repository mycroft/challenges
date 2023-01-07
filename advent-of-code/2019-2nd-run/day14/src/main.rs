use std::fs::read_to_string;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pair {
    component: String,
    number: isize,
}

#[derive(Debug)]
struct Recipe {
    pair: Pair,
    required: Vec<Pair>
}

impl From<&str> for Pair {
    fn from(value: &str) -> Self {
        let pairs = value.split(" ").collect::<Vec<&str>>();

        Self {
            component: pairs[1].to_string(),
            number: pairs[0].parse::<isize>().unwrap()
        }
    }
}

fn parse(fp: &str) -> Vec<Recipe> {
    let contents = read_to_string(fp).expect("An input file");
    let lines = contents.lines().collect::<Vec<&str>>();
    let mut result = Vec::new();

    for line in lines {
        let parts = line.split(" => ").collect::<Vec<&str>>();
        let parts_from = parts[0].split(", ").collect::<Vec<&str>>();
        let parts_to = parts[1];

        result.push(Recipe{
            pair: parts_to.into(),
            required: parts_from.iter().map(|&x| x.into()).collect()
        });
    }

    result
}

fn find_required_ore(recipes: &Vec<Recipe>, num: isize) -> isize {
    let mut expected = [Pair{component: "FUEL".to_string(), number: num}].to_vec();
    let mut supply: HashMap<String, isize> = HashMap::new();
    let mut ore = 0;

    while expected.len() > 0 {
        let el = expected.pop().unwrap();
        let recipe = recipes.iter().filter(|recipe| recipe.pair.component == el.component).nth(0).unwrap();

        let supply_element = supply.get(&recipe.pair.component).or(Some(&0)).unwrap();
        //let mut required = 0;
        //while (required * recipe.pair.number + supply_element) < (el.number) {
        //    required += 1;
        //}

        let required = ((el.number - supply_element) as f64 / (recipe.pair.number) as f64).ceil() as isize;

        //if required != required2 {
        //    println!("{required} {required2}");
        //}
        


        *supply.entry(recipe.pair.component.to_owned()).or_insert(0) = (required * recipe.pair.number + supply_element) - el.number;

        for component in &recipe.required {
            if component.component == "ORE".to_string() {
                ore += component.number * required;
            } else {
                expected.push(Pair{
                    component: component.component.clone(),
                    number: required * component.number,
                });
            }
        }
    }

    ore
}

fn find_fuel(recipes: &Vec<Recipe>) -> isize {
    let ore = 1000000000000i64;
    let mut min = 1;
    let mut max = ore;

    while min != max {
        let n = (max + min) / 2;

        let res = find_required_ore(recipes, n as isize) as i64;
        if res > ore {
            max = n;
        } else {
            min = n;
        }

        if max <= min + 1 {
            return min as isize;
        }
    }

    0
}

fn main() {
    let recipes = parse("input.txt");
    println!("#1 {}", find_required_ore(&recipes, 1)); // 612880
    println!("#2 {}", find_fuel(&recipes)); // 2509120
}

#[test]
fn test_sample() {
    let recipes = parse("input.txt_test0");
    assert_eq!(
        165,
        find_required_ore(&recipes, 1)
    );

    let recipes = parse("input.txt_test1");
    assert_eq!(
        13312,
        find_required_ore(&recipes, 1)
    );

    let recipes = parse("input.txt_test2");
    assert_eq!(
        180697,
        find_required_ore(&recipes, 1)
    );

    let recipes = parse("input.txt_test3");
    assert_eq!(
        2210736,
        find_required_ore(&recipes, 1)
    );
}

#[test]
fn test_sample_step2() {
    let recipes = parse("input.txt_test1");
    assert_eq!(
        82892753,
        find_fuel(&recipes)
    );

    let recipes = parse("input.txt_test2");
    assert_eq!(
        5586022,
        find_fuel(&recipes)
    );

    let recipes = parse("input.txt_test3");
    assert_eq!(
        460664,
        find_fuel(&recipes,)
    );
}