use std::{fs, collections::HashMap};
use pathfinding::directed::count_paths::count_paths;
use std::ops::Range;

#[macro_use] extern crate scan_fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    LesserThan,
    GreaterThan,
    None,
}

#[derive(Debug)]
struct Rule {
    variable: String,
    op: Operator,
    value: u32,
    target: String,
}

#[derive(Debug)]
struct Node {
    label: String,
    rules: Vec<Rule>,
}

#[derive(Debug)]
struct InitState {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl From<&str> for Rule {
    fn from(item: &str) -> Self {
        let (variable, operator, value, target) = scan_fmt_some!(item, "{/[a-z]+/}{[<>]}{/[0-9]+/}:{/[a-zAR]+/}", String, char, u32, String);

        let op = match operator.unwrap() {
            '<' => Operator::LesserThan,
            '>' => Operator::GreaterThan,
            _ => unreachable!(),
        };

        Rule {
            variable: variable.unwrap(),
            op,
            value: value.unwrap(),
            target: target.unwrap(),
        }
    }
}

impl From<&str> for Node {
    fn from(item: &str) -> Self {
        let mut default = String::new();
        let (label, rules) = scan_fmt_some!(item, "{[^{}]}{{{}}}", String, String);
        let mut final_rules = Vec::new();

        let rules = rules.unwrap();
        let rules = rules.split(',').collect::<Vec<&str>>();
        
        for (idx, &rule) in rules.iter().enumerate() {
            if idx == rules.len() - 1 {
                default = rule.to_string();
            } else {
                final_rules.push(rule.into());
            }
        }

        final_rules.push(Rule { variable: "s".to_string(), op: Operator::None, value: 0, target: default });

        Node {
            label: label.unwrap(),
            rules: final_rules,
        }
    }
}

fn read_file(fp: &str) -> (HashMap<String, Node>, Vec<InitState>){
    let contents = fs::read_to_string(fp).expect("A file to open");
    let lines = contents.lines();

    let mut rules: HashMap<String, Node> = HashMap::new();
    let mut init_states = Vec::new();

    let mut is_rules = true;

    for line in lines {
        if line.is_empty() {
            is_rules = false;
            continue;
        }

        if is_rules {
            let node: Node = line.into();
            rules.insert(node.label.clone(), node);
        } else {
            let (x, m, a, s) = scan_fmt_some!(line, "{{x={},m={},a={},s={}}}", u32, u32, u32, u32);
            init_states.push(InitState { x: x.unwrap(), m: m.unwrap(), a: a.unwrap(), s: s.unwrap() });
        }
    }

    (rules, init_states)
}

fn solve1(rules: &HashMap<String, Node>, states: &Vec<InitState>) -> u32 {
    let mut result = 0;

    for state in states {
        let mut current_node = String::from("in");

        loop {
            if current_node == *"A" {
                result += state.x + state.m + state.a + state.s;
                break;
            }

            if current_node == *"R" {
                break;
            }

            let node = rules.get(&current_node).unwrap();
            for rule in &node.rules {
                let var = match rule.variable.as_str() {
                    "x" => state.x,
                    "m" => state.m,
                    "a" => state.a,
                    "s" => state.s,
                    _ => unreachable!(),
                };
    
                if (rule.op == Operator::LesserThan && var < rule.value) || (rule.op == Operator::GreaterThan && var > rule.value) || rule.op == Operator::None {
                    current_node = rule.target.clone();
                    break;
                }
            }
        }
    }

    result
}

fn solve2(rules: &HashMap<String, Node>) -> usize {
    let mut result: usize = 0;

    let success =  |(n, cons): &(String, Vec<Range<isize>>)| {
        *n == *"A" && {
            result += cons
                .iter()
                .fold(1usize, |prod, c| prod * (c.end as usize - c.start as usize));
            true
        }
    };

    let successors = |(from, cons): &(String, Vec<Range<isize>>)| {
        let mut neigh = Vec::new();

        // xmas
        let get_index = |variable: &String| match variable.as_str() {
            "x" => 0,
            "m" => 1,
            "a" => 2,
            "s" => 3,
            _ => unreachable!(),
        };

        let mut conn = cons.clone();
        for (rule_from, node) in rules {

            if from != rule_from {
                continue;
            }

            for rule in &node.rules {
                let mut cont = conn.clone();

                match rule.op {
                    Operator::LesserThan => {
                        cont[get_index(&rule.variable)] = cont[get_index(&rule.variable)].start..(rule.value as isize);
                    }
                    Operator::GreaterThan => {
                        cont[get_index(&rule.variable)] = (rule.value as isize + 1)..cont[get_index(&rule.variable)].end;
                    },
                    Operator::None => {},
                }

                //println!("{} -> {:?}", rule.target, cont);

                neigh.push((rule.target.clone(), cont.clone()));

                match rule.op {
                    Operator::GreaterThan => {
                        conn[get_index(&rule.variable)] = conn[get_index(&rule.variable)].start..(rule.value as isize + 1);
                    }
                    Operator::LesserThan => {
                        conn[get_index(&rule.variable)] = (rule.value as isize)..conn[get_index(&rule.variable)].end;
                    },
                    Operator::None => {},
                }
            }
        }

        neigh
    };
    
    count_paths(("in".to_string(), vec![1..4001; 4]), successors, success);

    result
}

fn main() {
    let (rules, states) = read_file("input.txt");

    println!("#1 {}", solve1(&rules, &states));
    println!("#2 {}", solve2(&rules));
}
