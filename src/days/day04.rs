use std::{fs, str::FromStr, collections::{BTreeSet, BTreeMap}};

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_4.txt")
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
struct Card {
    id: i32,
    winning_num: BTreeSet<i32>,
    my_num: BTreeSet<i32>,
}

impl Card {
    fn get_matches(&self) -> u32 {
        self.winning_num.intersection(&self.my_num).count() as u32
    }

    fn get_points(&self) -> i32 {
        let num_matches = self.get_matches();
        if num_matches > 0 {
            2_i32.pow(num_matches-1)
        } else {
            0
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CardParsingError;

impl FromStr for Card {
    type Err = CardParsingError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(':').collect();
        let id = split[0].split_whitespace().collect::<Vec<_>>()[1].trim().parse::<i32>().unwrap();

        let closure_parse_numbers = |nums: &String| -> BTreeSet<i32> { 
            nums.split_whitespace().map(|s| {
                    s.trim().parse::<i32>().unwrap()
                })
                .collect() 
        };
        let num_splits = split[1].split('|').map(|s| s.to_owned()).collect::<Vec<String>>();
        Ok(Card { id , winning_num: closure_parse_numbers(&num_splits[0]), my_num: closure_parse_numbers(&num_splits[1]) })
    }
}


fn solve_part_1(input: Vec<String>) -> i32 {
    input.iter()
        .map(|s| -> Card {
            match Card::from_str(s) {
                Ok(x) => x,
                _ => panic!("Not a valid Card"),
            }
        })
        .map(|c| c.get_points())
        .sum()
}

fn solve_part_2(input: Vec<String>) -> i32 {
    let cards = input.iter()
        .map(|s| -> Card {
            match Card::from_str(s) {
                Ok(x) => x,
                _ => panic!("Not a valid Card"),
            }
        })
        .collect::<Vec<Card>>();
    let mut card_counts: BTreeMap<u32, i32> = cards.iter().map(|c| (c.id as u32, 1)).collect();
    cards.iter()
        .for_each(|c| {
            let v = card_counts.get(&(c.id as u32)).unwrap().to_owned();
            (1..=c.get_matches()).for_each(|i| {
                *card_counts.get_mut(&((c.id as u32)+ i)).unwrap() += v;
            });
        });
    card_counts.iter()
        .map(|c| c.1)
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input1 = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let p1 = solve_part_1(input1.clone());
        let p2 = solve_part_2(input1);
        assert_eq!(p1, 13);
        assert_eq!(p2, 30);
    }
}
