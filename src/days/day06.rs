use std::fs;

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_6.txt")
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
struct Race {
    time: u64,
    distance: u64,
}
impl Race {
    fn num_solutions(&self) -> u64 {
        let hold = 1..=self.time-1;
        hold.into_iter()
            .map(|t| (self.time-t)*t)
            .filter(|&d| d > self.distance)
            .count() as u64
    }
}

fn parse_row(s: &str) -> Vec<u64> {
    s.split(':').last().unwrap()
     .split_whitespace()
     .map(|s| s.parse::<u64>().unwrap())
     .collect()
}

fn solve_part_1(input: Vec<String>) -> u64 {
    let times: Vec<u64> = parse_row(&input[0]);
    let distance: Vec<u64> = parse_row(&input[1]);
    let mut races: Vec<Race> = vec![];
    for i in 0..times.len() {
        races.push(Race { time: times[i], distance: distance[i] });
    }
    races.iter().map(|r| r.num_solutions()).product()
}

fn solve_part_2(input: Vec<String>) -> u64 {
    let i1: String = input[0].chars().filter(|c| !c.is_whitespace()).collect();
    let i2: String = input[1].chars().filter(|c| !c.is_whitespace()).collect();
    let time: f64 = parse_row(&i1)[0] as f64;
    let distance: f64 = parse_row(&i2)[0] as f64;

    // Brute force solution
    // let r = Race { time : times, distance}; 
    // r.num_solutions()
   
    // solving quadratic equation: velocity * (time_total - time_hold) > distance, where velocity = time_hold
    // up to some corner cases
    ((-time - (time.powi(2) - 4_f64*distance).sqrt()) / -2_f64).ceil() as u64 - 
    ((-time + (time.powi(2) - 4_f64*distance).sqrt()) / -2_f64).ceil() as u64
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = "Time:      7  15   30
Distance:  9  40  200".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input.clone());
        let p2 = solve_part_2(input);
        assert_eq!(p1, 288);
        assert_eq!(p2, 71503);
    }
}
