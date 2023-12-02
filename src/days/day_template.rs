use std::fs;

pub fn solve() {
    let input = fs::read_to_string("./inputs/input.txt")
        .expect("File not found")
        .lines()
        .map(|l: &str| l.to_string())
        .collect::<Vec<String>>();

    let p1 = solve_part_1(input.clone());
    let p2 = solve_part_2(input);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

fn solve_part_1(input: Vec<String>) -> u8 {
    0
}

fn solve_part_2(input: Vec<String>) -> u8 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input1 = vec!["".to_string()];
        let input2 = vec!["".to_string()]; 
        let p1 = solve_part_1(input1);
        let p2 = solve_part_2(input2);
        assert_eq!(p1, 0);
        assert_eq!(p2, 0);
    }
}
