use std::fs::read_to_string;
use std::collections::HashSet;

type Range = (i128, i128);
type Ingredients = i128;

fn read_input(fp: &str) -> (HashSet<Range>, Vec<Ingredients>) {
    let mut ranges = HashSet::new();
    let mut ingredients = Vec::new();
    let contents = read_to_string(fp).expect("Failed to read file");
    let lines = contents.lines();

    let mut parsing_ranges = true;

    for line in lines {
        if line.trim().is_empty() {
            parsing_ranges = false;
            continue;
        }

        if parsing_ranges {
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].trim().parse::<i128>().expect("Failed to parse range start");
                let end = parts[1].trim().parse::<i128>().expect("Failed to parse range end");
                ranges.insert((start, end));
            }
        } else {
            let ingredient = line.trim().parse::<i128>().expect("Failed to parse ingredient");
            ingredients.push(ingredient);
        }
    }

    (ranges, ingredients)
}

fn check_used_ingredients(ranges: &HashSet<Range>, ingredients: &[Ingredients]) -> Vec<Ingredients> {
    ingredients.iter().cloned().filter(|&ing| {
        ranges.iter().any(|&(start, end)| ing >= start && ing <= end)
    }).collect()
}

fn fresh_ingredients(ranges: &HashSet<Range>) -> usize {
    // It is required to regroup ranges to avoid counting overlaps multiple times
    let mut sorted_ranges: Vec<Range> = ranges.iter().cloned().collect();
    sorted_ranges.sort_by_key(|&(start, _)| start);
    let mut merged_ranges: Vec<Range> = Vec::new();
    for range in sorted_ranges {
        if let Some(last) = merged_ranges.last_mut() {
            if range.0 <= last.1 + 1 {
                last.1 = last.1.max(range.1);
            } else {
                merged_ranges.push(range);
            }
        } else {
            merged_ranges.push(range);
        }
    }

    merged_ranges.iter().map(|&(start, end)| (end - start + 1) as usize).sum()
}

fn main() {
    let (ranges, ingredients) = read_input("input.txt");

    let used_ingredients = check_used_ingredients(&ranges, &ingredients);
    println!("#1: {}", used_ingredients.len());

    let total_fresh = fresh_ingredients(&ranges);
    println!("#2: {}", total_fresh);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_input() {
        let (ranges, ingredients) = read_input("input_test.txt");
        assert_eq!(ranges.len(), 4);
        assert_eq!(ingredients.len(), 6);
    }

    #[test]
    fn test_check_used_ingredients() {
        let (ranges, ingredients) = read_input("input_test.txt");
        assert_eq!(ranges.len(), 4);
        assert_eq!(ingredients.len(), 6);

        let used_ingredients = check_used_ingredients(&ranges, &ingredients);
        assert_eq!(used_ingredients.len(), 3);

        let total_fresh = fresh_ingredients(&ranges);
        assert_eq!(total_fresh, 14);
    }


}