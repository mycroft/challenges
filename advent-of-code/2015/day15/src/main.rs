use std::fs;
use std::cmp::max;
use regex::Regex;

#[derive(Debug)]
struct Ingredient<'a> {
    name: &'a str,
    capacity: i8,
    durability: i8,
    flavor: i8,
    texture: i8,
    calories: i8,
}

fn score(ingredients : &Vec<Ingredient>, visited: &Vec<u8>) -> (i64, i64) {
    let capacity = max(visited
                    .iter()
                    .enumerate()
                    .map(|(idx, x)| *x as i64 * ingredients[idx].capacity as i64)
                    .sum::<i64>(), 0);

    let durability = max(visited
                    .iter()
                    .enumerate()
                    .map(|(idx, x)| *x as i64 * ingredients[idx].durability as i64)
                    .sum::<i64>(), 0);

    let flavor = max(visited
                    .iter()
                    .enumerate()
                    .map(|(idx, x)| *x as i64 * ingredients[idx].flavor as i64)
                    .sum::<i64>(), 0);

    let texture = max(visited
                    .iter()
                    .enumerate()
                    .map(|(idx, x)| *x as i64 * ingredients[idx].texture as i64)
                    .sum::<i64>(), 0);

    let calories = max(visited
                    .iter()
                    .enumerate()
                    .map(|(idx, x)| *x as i64 * ingredients[idx].calories as i64)
                    .sum::<i64>(), 0);

    let score = max(capacity * durability * flavor * texture, 0);
    let calories_score = if calories == 500 { score } else { 0 };

    (score, calories_score)
}

fn compute(ingredients: &Vec<Ingredient>, visited: &mut Vec<u8>, remaining: usize) -> (i64, i64) {
    if visited.len() == ingredients.len() - 1 {
        visited.push(remaining as u8);
        let (score, calories_score) = score(&ingredients, &visited);
        visited.pop();

        return (score, calories_score);
    }

    let max_for_this_ingredient = remaining - ingredients.len() - visited.len();

    let mut max_recipe = 0;
    let mut max_recipe_calories = 0;

    for number in 0..max_for_this_ingredient {
        visited.push(number as u8);
        let (current_score, current_calories_score) = compute(&ingredients, visited, remaining - number);
        visited.pop();

        if current_score > max_recipe {
            max_recipe = current_score;
        }

        if current_calories_score > max_recipe_calories {
            max_recipe_calories = current_calories_score;
        }
    }

    return (max_recipe, max_recipe_calories);
}

fn main() {
    let mut ingredients : Vec<Ingredient> = vec![];
    let content = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let lines = content.lines();

    let re = Regex::new(r"^(.*): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (\d)$").unwrap();

    for line in lines {
        let cap = re.captures(line).unwrap();

        let ingredient = Ingredient{
            name:       cap.get(1).unwrap().as_str(),
            capacity:   cap.get(2).unwrap().as_str().parse().unwrap(),
            durability: cap.get(3).unwrap().as_str().parse().unwrap(),
            flavor:     cap.get(4).unwrap().as_str().parse().unwrap(),
            texture:    cap.get(5).unwrap().as_str().parse().unwrap(),
            calories:   cap.get(6).unwrap().as_str().parse().unwrap(),
        };

        ingredients.push(ingredient);
    }

    let (score, calories_score) = compute(&ingredients, &mut vec![], 100);
    println!("Part #1: {:?}", score);
    println!("Part #2: {:?}", calories_score);
}
