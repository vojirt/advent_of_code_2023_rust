use std::{fs, collections::HashMap};

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_12.txt")
        .expect("File not found")
        .lines()
        .map(|l: &str| l.to_string())
        .collect::<Vec<String>>();

    let p1 = solve_part_1(input.clone());
    let p2 = solve_part_2(input);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

fn solve_part_1(input: Vec<String>) -> i64 {
    input.iter()
        .map(|line| {
            let (pattern, num_brokens) = parse_line(line);
            possible_line_arrangements(pattern, &num_brokens)
        })
        .sum()
}

fn parse_line(line: &str) -> (&str, Vec<i64>) {
    let mut line_iter = line.split_whitespace();
    let pattern = line_iter.next().unwrap();
    let num_brokens: Vec<i64>  = line_iter.next().unwrap()
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect();
    (pattern, num_brokens)
}

fn possible_line_arrangements(pattern: &str, num_brokens: &[i64]) -> i64 {
    let blocks: Vec<&str> = pattern.split('.').filter(|s| !s.is_empty()).collect();
    let mut counter = 0;
    let mut cache = HashMap::<(String, usize, usize), i64>::new(); 
    recursive_match(&blocks, num_brokens, &mut counter, &mut cache);
    counter
}

fn recursive_match(blocks: &[&str], num_brokens: &[i64], counter: &mut i64, cache: &mut HashMap<(String, usize, usize), i64>) {
    //end conditions
    if blocks.is_empty() {
        // valid ? all broken tokens were matched
        if num_brokens.is_empty() {
            *counter += 1;
        }
        return
    } else if num_brokens.is_empty() {
        // valid ? rest of input does not contain '#'
        if !blocks.iter().any(|ss| ss.chars().any(|c| c == '#')) {
            *counter += 1;
        }
        return
    }

    if let Some(x) = cache.get(&(blocks[0].to_owned(), blocks.len(), num_brokens.len())) {
        *counter += x;
        return 
    }

    let local_counter: i64 = *counter;

    //discard block if we can
    if !blocks[0].chars().any(|c| c == '#') {
        recursive_match(&blocks[1..], num_brokens, counter, cache);
    }

    //expand pattern
    let mut block_start = 0;
    let mut pattern = String::from('#').repeat(num_brokens[0] as usize);
    pattern.push('.');
    let mut block_to_expand = String::from(blocks[0]);
    block_to_expand.push('.');

    while block_start < block_to_expand.len() {
        match first_wildcard_match(&block_to_expand[block_start..], &pattern) {
            Some(x) => {
                if block_to_expand[block_start..block_start+x].chars().any(|c| c == '#') {
                    break
                }
                let block_split = block_start + x + pattern.len(); 
                if  block_split < block_to_expand.len() {
                    let mut tmp: Vec<&str> = Vec::from(&blocks[1..]);
                    if block_split < blocks[0].len() {
                        tmp.insert(0, &blocks[0][block_split..]);
                    }
                    recursive_match(&tmp, &num_brokens[1..], counter, cache);
                    if blocks[0][block_start+x..].starts_with('#') {
                        break
                    }
                } else {
                    recursive_match(&blocks[1..], &num_brokens[1..], counter, cache);
                    break
                }
                let mut consecutive = 0;
                for i in x..x+pattern.len() {
                    if block_to_expand[block_start..].chars().nth(i).unwrap() == '#' {
                        consecutive += 1;
                    } else {
                        break
                    }
                }
                block_start = block_start + x + 1 + consecutive;
            },
            None => break,
        }
    }

    let diff = *counter - local_counter;
    if let Some(x) = cache.insert((blocks[0].to_owned(), blocks.len(), num_brokens.len()), diff) {
        if x != diff {
            println!("{} -> {}", diff, x);
            panic!("Updating cache with different values");
        }
    }
}

fn match_wildcard(str_wild: &str, pattern: &str) -> bool {
    str_wild.chars()
        .enumerate()
        .all(|(i, c)| {
            match c {
                '?' => true,
                c => pattern.chars().nth(i).unwrap() == c,
            }
        })
}

fn first_wildcard_match(str_wild: &str, pattern: &str) -> Option<usize> {
    if str_wild.len() < pattern.len() {
        None
    } else { 
        let mut ret: Option<usize> = None;
        for i in 0..str_wild.len()-pattern.len() + 1 { 
            if match_wildcard(&str_wild[i..i+pattern.len()], pattern) {
                ret = Some(i);
                break;
            }
        }
        ret
    }
}

fn parse_line_augmented(line: &str) -> (String, Vec<i64>) {
    let mut line_iter = line.split_whitespace();
    let pattern_orig = line_iter.next().unwrap();
    let mut pattern = pattern_orig.to_string();

    let num_brokens_orig: Vec<i64>  = line_iter.next().unwrap()
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect();

    let mut num_brokens: Vec<i64>  = num_brokens_orig.clone();
    for _ in 0..4{
        pattern.push('?');
        pattern += pattern_orig;
        num_brokens_orig.iter().for_each(|i| num_brokens.push(*i));
    }
    (pattern, num_brokens)
}

fn solve_part_2(input: Vec<String>) -> i64 {
    input.iter()
        .map(|line| {
            let (pattern, num_brokens) = parse_line_augmented(line);
            possible_line_arrangements(&pattern, &num_brokens)
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
???.??????.????...?. 3,3
?????...#?? 5,1
#??.#.??????##? 1,1,7
????.??#?.?.????# 3,4,1,1
??###?##.??????#??#. 8,1,2,2
?#?#??????#?? 4,1,1
??#?##???#?? 1,3,3".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let input_orig = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();


        let p1 = solve_part_1(input.clone());
        let p2 = solve_part_2(input_orig);
        assert_eq!(p1, 61);
        assert_eq!(p2, 525152);
    }
}
