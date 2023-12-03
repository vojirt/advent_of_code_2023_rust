use std::{fs, fmt};

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_3.txt")
        .expect("File not found")
        .lines()
        .map(|l: &str| l.to_string())
        .collect::<Vec<String>>();

    let p1 = solve_part_1(input.clone());
    let p2 = solve_part_2(input);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

#[derive(Debug, Clone)]
struct Number {
    number: i32,
    valid: bool,
    locations: Vec<[i32; 2]>,
}

struct Gear<'a> {
    gears: Vec<&'a Number>,
}

#[derive(Clone)]
enum Symbols {
    Number,
    Ignore, 
    Symbol,
    Gear,
}

impl fmt::Debug for Symbols {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Symbols::Number => write!(f, "N"),
            Symbols::Ignore => write!(f, "."),
            Symbols::Symbol => write!(f, "S"),
            Symbols::Gear   => write!(f, "*"),
        }
    }
}


fn solve_part_1(input: Vec<String>) -> i32 {
    let scheme: Vec<Vec<Symbols>> = input.iter()
        .map(|s| {
            s.chars()
             .map(|c| {
                match c {
                    '.'  => Symbols::Ignore,
                    '0'..='9' => Symbols::Number,
                    '*' => Symbols::Gear,
                    _ => Symbols::Symbol,
                }
             })
             .collect()
        })
        .collect();

    let mut numbers: Vec<Number> = Vec::new();
    for y in 0..scheme.len() {
        let mut current_num: String = String::new();
        let mut locs: Vec<[i32; 2]> = Vec::new();
        for x in 0..scheme[0].len() {
            match scheme[y][x] {
                Symbols::Number => {
                    current_num.push(input[y].chars().nth(x).unwrap());
                    locs.push([y as i32, x as i32]);
                },
                _ => {
                    if current_num.len() > 0 {
                        numbers.push(Number { number: current_num.parse::<i32>().unwrap(), valid: is_number_valid(&locs, &scheme), locations: locs});
                        current_num = String::new();
                        locs = Vec::new();
                    }
                }
            }
        }
        if current_num.len() > 0 {
            numbers.push(Number { number: current_num.parse::<i32>().unwrap(), valid: is_number_valid(&locs, &scheme), locations: locs});
        }
    }

    numbers.iter()
        .filter(|n| n.valid)
        .map(|n| n.number)
        .sum()
}

fn is_number_valid(locs: &Vec<[i32; 2]>, scheme: &Vec<Vec<Symbols>>) -> bool {
    locs.iter()
        .any(|loc| {
            for y in (loc[0]-1).max(0)..=(loc[0]+1).min(scheme.len() as i32 - 1) {
                for x in (loc[1]-1).max(0)..=(loc[1]+1).min(scheme[0].len() as i32 - 1) {
                    match scheme[y as usize][x as usize] {
                        Symbols::Symbol | Symbols::Gear => return true,
                        _ => (),
                    }
                }
            }
            false
        })
}

fn solve_part_2(input: Vec<String>) -> i32 {
    let scheme: Vec<Vec<Symbols>> = input.iter()
        .map(|s| {
            s.chars()
             .map(|c| {
                match c {
                    '.'  => Symbols::Ignore,
                    '0'..='9' => Symbols::Number,
                    '*' => Symbols::Gear,
                    _ => Symbols::Symbol,
                }
             })
             .collect()
        })
        .collect();

    let mut numbers: Vec<Number> = Vec::new();
    for y in 0..scheme.len() {
        let mut current_num: String = String::new();
        let mut locs: Vec<[i32; 2]> = Vec::new();
        for x in 0..scheme[0].len() {
            match scheme[y][x] {
                Symbols::Number => {
                    current_num.push(input[y].chars().nth(x).unwrap());
                    locs.push([y as i32, x as i32]);
                },
                _ => {
                    if current_num.len() > 0 {
                        numbers.push(Number { number: current_num.parse::<i32>().unwrap(), valid: is_number_valid(&locs, &scheme), locations: locs});
                        current_num = String::new();
                        locs = Vec::new();
                    }
                }
            }
        }
        if current_num.len() > 0 {
            numbers.push(Number { number: current_num.parse::<i32>().unwrap(), valid: is_number_valid(&locs, &scheme), locations: locs});
        }
    }
    
    let mut gears: Vec<Gear> = Vec::new();
    for y in 0..scheme.len() {
        for x in 0..scheme[0].len() {
            if let Symbols::Gear = scheme[y][x] {
                gears.push(Gear { gears: numbers.iter()
                    .filter(|n| {
                        n.locations.iter()
                            .any(|loc| {
                                ((loc[0] - y as i32).abs() <= 1) & ((loc[1] - x as i32).abs() <= 1)
                            })

                    })
                    .collect::<Vec<&Number>>() 
                });
            }
        }
    }

    gears.iter()
        .filter(|g| g.gears.len() == 2)
        .map(|g| g.gears[0].number * g.gears[1].number)
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input1 = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598.."
            .to_string()
            .split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let p1 = solve_part_1(input1.clone());
        let p2 = solve_part_2(input1);
        assert_eq!(p1, 4361);
        assert_eq!(p2, 467835);
    }
}
