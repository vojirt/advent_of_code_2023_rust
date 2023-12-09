use std::fs;

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_9.txt")
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
            predict_next_value(line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect())
        })
        .sum()
}

fn predict_next_value(orig_data: Vec<i64>) -> i64 {
    let mut pyramid: Vec<Vec<i64>> = vec![orig_data];
    
    while pyramid.last().unwrap().iter().any(|i| *i != 0) {
        pyramid.push(
            pyramid.last().unwrap().windows(2)
                .map(|slice| slice[1] - slice[0])
                .collect::<Vec<i64>>()
            );
    }
    (0..pyramid.len()-1).rev().fold(0_i64, |acc, i| {
        acc + pyramid[i].last().unwrap()  
    })
}

fn solve_part_2(input: Vec<String>) -> i64 {
    input.iter()
        .map(|line| {
            predict_previous_value(line.split_whitespace()
                .map(|s| s.parse::<i64>().unwrap())
                .collect())
        })
        .sum()
}

fn predict_previous_value(orig_data: Vec<i64>) -> i64 {
    let mut pyramid: Vec<Vec<i64>> = vec![orig_data];
    
    while pyramid.last().unwrap().iter().any(|i| *i != 0) {
        pyramid.push(
            pyramid.last().unwrap().windows(2)
                .map(|slice| slice[1] - slice[0])
                .collect::<Vec<i64>>()
            );
    }
    (0..pyramid.len()-1).rev().fold(0_i64, |acc, i| {
        pyramid[i].first().unwrap() - acc
    })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input.clone());
        let p2 = solve_part_2(input);
        assert_eq!(p1, 114);
        assert_eq!(p2, 2);
    }
}
