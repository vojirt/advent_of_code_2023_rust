pub fn solve() {
    let p1 = solve_part_1();
    let p2 = solve_part_2();

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

fn solve_part_2() -> u8 {
    0
}

fn solve_part_1() -> u8 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let p1 = solve_part_1();
        let p2 = solve_part_2();
        assert_eq!(p1, 0);
        assert_eq!(p2, 0);
    }
}
