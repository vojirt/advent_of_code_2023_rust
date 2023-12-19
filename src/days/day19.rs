use std::{fs, collections::HashMap, ops::Range};

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_19.txt")
        .expect("File not found")
        .lines()
        .map(|l: &str| l.to_string())
        .collect::<Vec<String>>();

    let p1 = solve_part_1(input.clone());
    let p2 = solve_part_2(input);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum PartType {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Comp {
    Greater,
    Less,
}

fn solve_part_1(input: Vec<String>) -> i32 {
    let f_end_id = input.iter().position(|line| line.is_empty()).unwrap();
    let filters: HashMap::<&str, (Vec::<(PartType,  Comp, i32, &str)>, &str)> = input[..f_end_id].iter()
        .map(|line| {
            let name_end_id = line.chars().position(|c| c == '{').unwrap();
            let name = &line[..name_end_id];
            let rules_str: Vec<&str> = line[name_end_id+1..line.len()-1].split(',')
                .map(|rule| rule)
                .collect();
            let rules = rules_str[..rules_str.len()-1].iter()
                .map(|rule| {
                    let cmp_end_id = rule.chars().position(|c| c == ':').unwrap();
                    let part_type = match rule.chars().nth(0).unwrap() {
                        'x' => PartType::X,
                        'm' => PartType::M,
                        'a' => PartType::A,
                        's' => PartType::S,
                         _  => panic!("Invalid part type"),
                    };
                    match rule.chars().nth(1).unwrap() {
                        '>' => (part_type, Comp::Greater, rule[2..cmp_end_id].parse::<i32>().unwrap(), &rule[cmp_end_id+1..]),
                        '<' => (part_type, Comp::Less, rule[2..cmp_end_id].parse::<i32>().unwrap(), &rule[cmp_end_id+1..]),
                        _ => panic!("Invalid comparator"),
                    }
                })
                .collect();
            (name, (rules, rules_str[rules_str.len()-1])) 
        })
        .collect();
    let parts: Vec::<HashMap<PartType, i32>> = input[f_end_id+1..].iter()
        .map(|line| {
            line[1..line.len()-1].split(',')
                .map(|p| {
                    let part_type = match p.chars().nth(0).unwrap() {
                        'x' => PartType::X,
                        'm' => PartType::M,
                        'a' => PartType::A,
                        's' => PartType::S,
                         _  => panic!("Invalid part type"),
                    };
                    (part_type, p[2..].parse::<i32>().unwrap())
                })
                .collect::<HashMap::<PartType, i32>>()
        })
        .collect();
    
    parts.iter()
        .fold(0, |acc, p| {
            if is_part_valid(p, filters.get("in").unwrap(), &filters) {
                acc + p.iter().fold(0, |acc2, (k, v)| acc2 + *v) 
            } else {
                acc
            }
        })
}

fn is_part_valid(p: &HashMap<PartType, i32>, current_filter: &(Vec<(PartType, Comp, i32, &str)>, &str), filters: &HashMap<&str, (Vec<(PartType, Comp, i32, &str)>, &str)>) -> bool {
    let mut next_filter: &str = current_filter.1;
    for filter in current_filter.0.iter() {
        match filter.1 {
            Comp::Less => {
                if p.get(&filter.0).unwrap() < &filter.2 {
                    next_filter = filter.3; 
                    break;
                }
            },
            Comp::Greater => {
                if p.get(&filter.0).unwrap() > &filter.2 {
                    next_filter = filter.3; 
                    break;
                }
            },
        }
    }

    if next_filter == "A" {
        true
    } else if next_filter == "R" {
        false
    } else {
        is_part_valid(p, filters.get(next_filter).unwrap(), filters)
    }
}

fn is_part_valid_ranges(p: &HashMap<PartType, Range<i32>>, current_filter: &(Vec<(PartType, Comp, i32, &str)>, &str), filters: &HashMap<&str, (Vec<(PartType, Comp, i32, &str)>, &str)>) -> i64 {
    let mut current_part_splits = vec![p.clone()]; 
    let mut part_splits_next_filter: Vec<(HashMap<PartType, Range<i32>>, &str)> = vec![];
    for filter in current_filter.0.iter() {
        let mut next_part_splits = vec![];
        for split in current_part_splits {
            let r = split.get(&filter.0).unwrap();
            match filter.1 {
                Comp::Less => {
                    if r.contains(&filter.2) {
                        let mut split_match= split.clone();
                        split_match.insert(filter.0.clone(), r.start..filter.2);
                        let mut split_not_match = split.clone();
                        split_not_match.insert(filter.0.clone(), filter.2..r.end);
                        part_splits_next_filter.push((split_match, filter.3));
                        next_part_splits.push(split_not_match);
                    } else if r.end <= filter.2 {
                        part_splits_next_filter.push((split, filter.3));
                    } else {
                        next_part_splits.push(split);
                    }
                },
                Comp::Greater => {
                    if r.contains(&filter.2) {
                        let mut split_match= split.clone();
                        split_match.insert(filter.0.clone(), filter.2+1..r.end );
                        let mut split_not_match = split.clone();
                        split_not_match.insert(filter.0.clone(), r.start..filter.2+1);
                        part_splits_next_filter.push((split_match, filter.3));
                        next_part_splits.push(split_not_match);
                    } else if r.start > filter.2 {
                        part_splits_next_filter.push((split, filter.3));
                    } else {
                        next_part_splits.push(split);
                    }
                },
            }
        }
        current_part_splits = next_part_splits;
    }

    for split in current_part_splits {
        part_splits_next_filter.push((split, current_filter.1));
    }

    part_splits_next_filter.into_iter()
        .fold(0, |total, (p, next_filter)| {
            if next_filter == "A" {
                total + p.into_iter()
                    .fold(1, |acc, (_, r)| {
                        acc * r.len() as i64 
                    })
            } else if next_filter == "R" {
                total
            } else {
                total + is_part_valid_ranges(&p, filters.get(next_filter).unwrap(), filters)
            }
        })
}

fn solve_part_2(input: Vec<String>) -> i64 {
    let f_end_id = input.iter().position(|line| line.is_empty()).unwrap();
    let filters: HashMap::<&str, (Vec::<(PartType,  Comp, i32, &str)>, &str)> = input[..f_end_id].iter()
        .map(|line| {
            let name_end_id = line.chars().position(|c| c == '{').unwrap();
            let name = &line[..name_end_id];
            let rules_str: Vec<&str> = line[name_end_id+1..line.len()-1].split(',')
                .map(|rule| rule)
                .collect();
            let rules = rules_str[..rules_str.len()-1].iter()
                .map(|rule| {
                    let cmp_end_id = rule.chars().position(|c| c == ':').unwrap();
                    let part_type = match rule.chars().nth(0).unwrap() {
                        'x' => PartType::X,
                        'm' => PartType::M,
                        'a' => PartType::A,
                        's' => PartType::S,
                         _  => panic!("Invalid part type"),
                    };
                    match rule.chars().nth(1).unwrap() {
                        '>' => (part_type, Comp::Greater, rule[2..cmp_end_id].parse::<i32>().unwrap(), &rule[cmp_end_id+1..]),
                        '<' => (part_type, Comp::Less, rule[2..cmp_end_id].parse::<i32>().unwrap(), &rule[cmp_end_id+1..]),
                        _ => panic!("Invalid comparator"),
                    }
                })
                .collect();
            (name, (rules, rules_str[rules_str.len()-1])) 
        })
        .collect();

    let mut part = HashMap::<PartType, Range<i32>>::new();
    part.insert(PartType::X, 1..4001);
    part.insert(PartType::M, 1..4001);
    part.insert(PartType::A, 1..4001);
    part.insert(PartType::S, 1..4001);

    is_part_valid_ranges(&part, filters.get("in").unwrap(), &filters)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input.clone());
        let p2 = solve_part_2(input);
        assert_eq!(p1, 19114);
        assert_eq!(p2, 167409079868000);
    }
}
