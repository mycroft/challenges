fn main() {
    let res = compute_file("input.txt");
    println!("#1 {}", magnitude(&res));

    // Part 2

    let exprs = get_from_file("input.txt");
    println!("#2 {}", get_max_magnitude(&exprs));
}

fn get_max_magnitude(exprs: &[Vec<(usize, usize)>]) -> usize {
    let mut magnitude_max = 0;

    for i in 0..exprs.len() {
        for j in i+1..exprs.len() {
            let current_magnitude = magnitude(&add(&exprs[i], &exprs[j]));
            if current_magnitude > magnitude_max {
                magnitude_max = current_magnitude;
            }

            let current_magnitude = magnitude(&add(&exprs[j], &exprs[i]));
            if current_magnitude > magnitude_max {
                magnitude_max = current_magnitude;
            }
        }
    }

    magnitude_max
}

fn magnitude(v: &[(usize, usize)]) -> usize {
    if v.is_empty() {
        return 0;
    }

    let mut v = v.to_owned();

    loop {
        let current_inner_level = v.iter().map(|(_, v)| *v).max().unwrap();

        let idx = has_exploding(&v, current_inner_level).unwrap();
        if v[idx].1 != v[idx+1].1 {
            unreachable!();
        }

        let left_v = v[idx];
        let right_v= v[idx+1];

        // Remove right element
        v.remove(idx+1);

        // left element goes lower and gets a new value
        v[idx].1 -= 1;
        v[idx].0 = left_v.0 * 3 + right_v.0 * 2;

        if v.len() == 1 {
            break
        }
    }

    v[0].0
}

fn get_from_file(fp: &str) -> Vec<Vec<(usize, usize)>> {
    let contents = std::fs::read_to_string(fp).expect("file");
    let lines = contents.lines().collect::<Vec<&str>>();

    let mut res = vec![];
    for line in lines {
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }

        res.push(parse(line));
    }

    res
}

fn compute_file(fp: &str) -> Vec<(usize, usize)> {
    let exprs = get_from_file(fp);

    let mut res = vec![];
    for expr in &exprs {
        // First line:
        if res.is_empty() {
            res = expr.clone();
            continue;
        }

        res = add(&res, expr);
    }

    res
}

fn has_exploding(v: &[(usize, usize)], level: usize) -> Option<usize> {
    let mut inner = None;

    for (idx, el) in v.iter().enumerate() {
        if el.1 == level {
            inner = Some(idx);
            break;
        }
    }

    inner
}

// Explode once
// find the left most node at level 5. Replace it by 0 and remove its right node.
// Add the value to the left node if possible, and add the right node value to its rightest node.
fn explode_once(v: &mut Vec<(usize, usize)>) -> bool {
    let inner_5 = has_exploding(v, 5);

    if inner_5.is_none() {
        return false;
    }

    let idx = inner_5.unwrap();

    let left_v = v[idx];
    let left_r = v[idx+1];

    // Replace left element by 0 at upper level
    v[idx] = (0, left_v.1 - 1);

    // Remove right element
    v.remove(idx+1);

    // Add left_v.0 to idx-1
    if idx > 0 {
        v[idx-1].0 += left_v.0;
    }

    // Add right_v.0 to idx+1
    if idx+1 < v.len() {
        v[idx+1].0 += left_r.0;
    }

    true
}

fn has_split(v: &[(usize, usize)]) -> Option<usize> {
    let mut large_number = None;

    for (idx, el) in v.iter().enumerate() {
        if el.0 >= 10 {
            large_number = Some(idx);
            break;
        }
    }

    large_number
}

// Split once
// Find any number with a value >= 10, and explse it in two.
// Eg: 10 becomes [5, 5], 11 becomes [5, 6], 12 becomes [6, 6] and so on.
fn split_once(v: &mut Vec<(usize, usize)>) -> bool {
    let large_number = has_split(v);

    if large_number.is_none() {
        return false;
    }

    let idx = large_number.unwrap();
    let orig_number = v[idx].0;

    // Replace node by value divided by two (larger number)
    v[idx].0 = orig_number - (orig_number / 2);
    v[idx].1 += 1;

    // Insert the left number part
    v.insert(idx, (orig_number/2, v[idx].1));

    true
}

// Takes a snaifish number as input, output the vec of numbers + levels.
fn parse(s: &str) -> Vec<(usize, usize)> {
    let mut res = vec![];

    let mut current_level = 0;
    let mut current_number: Option<usize> = None;

    for c in s.chars() {
        match c {
            '[' => {
                current_level += 1;
            }
            ']' => {
                if current_number.is_some() {
                    res.push((current_number.unwrap(), current_level));
                    current_number = None;
                }
                current_level -= 1;
            }
            n if ('0'..='9').contains(&n) => {
                if current_number.is_none() {
                    current_number = Some(0);
                }

                current_number = Some(current_number.unwrap() * 10);
                current_number = Some(current_number.unwrap() + n.to_digit(10).expect("number") as usize);
            },
            ',' => {
                if current_number.is_some() {
                    res.push((current_number.unwrap(), current_level));
                    current_number = None;
                }
            },
            _ => {
                println!("Reached character: {}", c);
                unreachable!()
            }
        };
    }

    res
}

// Addition
fn add(v0: &[(usize, usize)], v1: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut res : Vec<(usize, usize)> = vec![];

    // for each element in v0, add in res with an upper level
    for el in v0.iter() {
        let mut v = *el;
        v.1 += 1;
        res.push(v);
    }

    // same for v1
    for el in v1.iter() {
        let mut v = *el;
        v.1 += 1;
        res.push(v);
    }

    // Then apply reduce.
    loop {
        let mut op_res = explode_once(&mut res);
        if op_res {
            continue;
        }

        op_res = split_once(&mut res);
        if op_res {
            continue;
        }

        // Nothing was done

        break;
    }

    res
}

// Some tests to make things easier later.
#[test]
fn test_parsing() {
    assert_eq!(vec![(1, 1), (2, 1)], parse("[1,2]"));
    assert_eq!(vec![(1, 2), (2, 2), (3, 1)], parse("[[1,2],3]"));
    assert_eq!(
        vec![(3, 2), (2, 3), (1, 4), (7, 5), (3, 5), (6, 2), (5, 3), (4, 4), (3, 5), (2, 5)],
        parse("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")
    )
}

#[test]
fn test_exploding() {
    let test_values = [
        ("[[[[0,9],2],3],4]", "[[[[[9,8],1],2],3],4]"),
        ("[7,[6,[5,[7,0]]]]", "[7,[6,[5,[4,[3,2]]]]]"),
        ("[[6,[5,[7,0]]],3]", "[[6,[5,[4,[3,2]]]],1]"),
        ("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"),
        ("[[3,[2,[8,0]]],[9,[5,[7,0]]]]", "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
    ];

    for test_value in test_values {
        let mut test = parse(test_value.1);
        explode_once(&mut test);
        assert_eq!(parse(test_value.0), test);
    }
}

#[test]
fn test_split() {
    let test_values = [
        ("[1,[5,5]]", "[1,10]"),
        ("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]"),
        ("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", "[[[[0,7],4],[15,[0,13]]],[1,1]]"),
    ];
    for test_value in test_values {
        let mut test = parse(test_value.1);
        split_once(&mut test);
        assert_eq!(parse(test_value.0), test);
    }
}

#[test]
fn test_addition() {
    let test_values = [
        // result, v_1, v2
        ("[[1,2],[[3,4],5]]", "[1,2]", "[[3,4],5]"),
        ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", "[[[[4,3],4],4],[7,[[8,4],9]]]", "[1,1]"),
    ];
    for test_value in test_values {
        let v0 = parse(test_value.1);
        let v1 = parse(test_value.2);

        let res = add(&v0, &v1);
        assert_eq!(parse(test_value.0), res);
    }
}

#[test]
fn test_compute_file() {
    let test_values = [
        ("[[[[1,1],[2,2]],[3,3]],[4,4]]", "input.txt_test0"),
        ("[[[[3,0],[5,3]],[4,4]],[5,5]]", "input.txt_test1"),
        ("[[[[5,0],[7,4]],[5,5]],[6,6]]", "input.txt_test2"),
        ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", "input.txt_test3"),
    ];

    for test_value in test_values {
        let res = compute_file(test_value.1);
        assert_eq!(parse(test_value.0), res);
    }
}

#[test]
fn test_magnitude() {
    let test_values = [
        (143usize, "[[1,2],[[3,4],5]]"),
        (1384, "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),
        (445, "[[[[1,1],[2,2]],[3,3]],[4,4]]"),
        (791, "[[[[3,0],[5,3]],[4,4]],[5,5]]"),
        (1137, "[[[[5,0],[7,4]],[5,5]],[6,6]]"),
        (3488, "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
    ];

    for test_value in test_values {
        let res = magnitude(&parse(test_value.1));
        assert_eq!(res, test_value.0);
    }
}

#[test]
fn test_mega() {
    // Part 1
    let res = magnitude(&compute_file("input.txt_test4"));
    assert_eq!(4140, res);

    // Part 2
    let res= get_max_magnitude(&get_from_file("input.txt_test4"));
    assert_eq!(3993, res)
}