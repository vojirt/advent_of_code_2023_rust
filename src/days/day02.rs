use std::{fs, str::FromStr};

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_2.txt")
        .expect("File not found")
        .lines()
        .map(|l: &str| l.to_string())
        .collect::<Vec<String>>();

    let p1 = solve_part_1(input.clone(), [12, 13, 14]);
    let p2 = solve_part_2(input);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

#[derive(Debug, Clone)]
struct Game {
    id: i32,
    instances: Vec<Vec<i32>>,
}

impl Game {
    fn max_cubes(&self) -> Vec<i32> {
        let mut max_v: Vec<i32> = vec![0, 0, 0];
        for inst in self.instances.iter() {
            (0..3).for_each(|i| {
                max_v[i] = std::cmp::max(max_v[i], inst[i]);
            });
        }
        max_v
    }
}

#[derive(Debug, PartialEq, Eq)]
struct GameParsingError;

impl FromStr for Game {
    type Err = GameParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(':').collect();
        let id = split[0].split(' ').collect::<Vec<_>>()[1].trim().parse::<i32>().unwrap();
        let instances = split[1].split(';')
            .map(|ins| {
                let cubes = ins.split(',').collect::<Vec<&str>>();
                let mut cubes_counts = [0, 0, 0];
                for c in cubes {
                    let count_type = c.trim().split(' ').collect::<Vec<&str>>();
                    let count = count_type[0].parse::<i32>().unwrap();
                    let type_id: usize = match count_type[1] {
                                "red" => 0, 
                                "green" => 1,
                                "blue" => 2,
                                _ => panic!("Not a valid cube type"),
                    };
                    cubes_counts[type_id] = std::cmp::max(count, cubes_counts[type_id]);
                }
                Vec::from(cubes_counts)
            })
            .collect();
        Ok(Game { id, instances })
    }
}

fn solve_part_1(input: Vec<String>, cubes_limit: [i32; 3]) -> i32 {
    input.iter()
        .map(|s| -> Game {
            match Game::from_str(s) {
                Ok(x) => x,
                _ => panic!("Not a valid Game"),
            }
        })
        .filter(|g| {
            let max_cubes = g.max_cubes();
            let mut flag = true;
            (0..3).for_each(|i| {
                flag &= max_cubes[i] <= cubes_limit[i]
            });
            flag
        })
        .map(|g| g.id)
        .sum()
}

fn solve_part_2(input: Vec<String>) -> i32 {
    input.iter()
        .map(|s| -> Game {
            match Game::from_str(s) {
                Ok(x) => x,
                _ => panic!("Not a valid Game"),
            }
        })
        .map(|g| {
            let max_cubes = g.max_cubes();
            let mut prod = 1;
            (0..3).for_each(|i| {
                prod *= max_cubes[i];
            });
            prod
        })
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input1 = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string().split('\n').map(|s| s.to_string()).collect::<Vec<String>>();

        let p1 = solve_part_1(input1.clone(), [12, 13, 14]);
        let p2 = solve_part_2(input1);
        assert_eq!(p1, 8);
        assert_eq!(p2, 2286);
    }
}
