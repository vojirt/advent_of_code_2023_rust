use std::{fs, collections::HashMap};

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_8.txt")
        .expect("File not found")
        .lines()
        .map(|l: &str| l.to_string())
        .collect::<Vec<String>>();

    let p1 = solve_part_1(input.clone());
    let p2 = solve_part_2(input);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

#[derive(Debug)]
struct Element<'a> {
    me: &'a str,
    left: &'a str,
    right: &'a str,
    end_point: bool,
}

enum Instruction {
    Left,
    Right,
}

fn parse_instructions(s: &str) -> Vec<Instruction> {
    s.chars()
     .map(|c| match c {
                'L' => Instruction::Left,
                'R' => Instruction::Right,
                _ => panic!("Incorrect Instruction"),
     })
    .collect()
}

fn parse_map(s: &[String]) -> HashMap<&str, Element> {
    s.iter()
     .map(|l| {
         let key = l.split('=').next().unwrap().trim();
         let values: Vec<&str> = l.split('=').last().unwrap().split(',').collect();
        (key, Element{me: key, left: &values[0].trim()[1..], right: &values[1].trim()[..3], end_point: key.ends_with('Z')})
     }).collect::<HashMap<&str, Element>>()
}

fn solve_part_1(input: Vec<String>) -> i32 {
    let instructions = parse_instructions(&input[0]);
    let map = parse_map(&input[2..]);

    let mut current_element = "AAA";
    let end_element = "ZZZ";
    let mut step_counter = 0;

    while current_element != end_element {
        let instruction_id = step_counter % instructions.len();
        match instructions[instruction_id] {
            Instruction::Left => current_element = map.get(current_element).unwrap().left, 
            Instruction::Right => current_element = map.get(current_element).unwrap().right, 
        }
        step_counter += 1;
    }
    step_counter as i32
}

fn solve_part_2(input: Vec<String>) -> u64 {
    let instructions = parse_instructions(&input[0]);
    let map = parse_map(&input[2..]);

    let mut current_elements: Vec<&Element> = map.iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(_, v)| v)
        .collect::<Vec<&Element>>();

    // Brute force solution, takes too long ofc.
    // let mut step_counter = 0;
    // while current_elements.iter().any(|e| !e.end_point) {
    //     current_elements = match instructions[step_counter % instructions.len()] {
    //         Instruction::Left => current_elements.iter().map(|e2| map.get(e2.left).unwrap()).collect::<Vec<&Element>>(), 
    //         Instruction::Right => current_elements.iter().map(|e2| map.get(e2.right).unwrap()).collect::<Vec<&Element>>(), 
    //     };
    //     step_counter += 1;
    //     if step_counter % 1000000 == 0 {
    //         println!("{step_counter}");
    //     }
    // }
    // step_counter as i32

    // This is probably not generic solution but it is what the AoC wants.
    // The assumption is that each starting point ends only in one unique ending point (which was not
    // explicitly written in the problem description). This significanly simplifies the problem. The
    // other thing is that there exist a loop that any ending point arrives to only to the same ending
    // point and in constant ammount of steps (equal to number of steps from the unique starting point). It is not
    // obvious to my that this property is general or if it only applies to the provided input data.
    
    let path_lengths = current_elements.iter()
        .map(|e| {
            let mut step_counter = 0;
            let mut current_element = *e;
            while !current_element.end_point {
                current_element = match instructions[step_counter % instructions.len()] {
                    Instruction::Left => map.get(current_element.left).unwrap(),
                    Instruction::Right => map.get(current_element.right).unwrap(),
                };
                step_counter += 1;
            };
            step_counter as u64
        })
        .collect::<Vec<u64>>();
    
    lcm(&path_lengths)
}

// from https://github.com/TheAlgorithms/Rust/blob/master/src/math/lcm_of_n_numbers.rs
pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let input2 = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let p1 = solve_part_1(input);
        let p2 = solve_part_2(input2);
        assert_eq!(p1, 2);
        assert_eq!(p2, 6);
    }

    #[test]
    fn simple_case_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input);
        assert_eq!(p1, 6);
    }
}
