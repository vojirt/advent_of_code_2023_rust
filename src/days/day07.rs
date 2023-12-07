use std::{fs, cmp::Ordering, collections::HashMap};

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_7.txt")
        .expect("File not found")
        .lines()
        .map(|l: &str| l.to_string())
        .collect::<Vec<String>>();

    let p1 = solve_part_1(input.clone());
    let p2 = solve_part_2(input);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
struct Card {
    value: i32,
}

#[derive(Debug)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard(i32),
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: i32,
    hand_type: HandType,
}

impl Hand {
    fn value(&self) -> i32 {
        match self.hand_type {
            HandType::FiveKind => 500,
            HandType::FourKind => 400,
            HandType::FullHouse => 350,
            HandType::ThreeKind => 300,
            HandType::TwoPair => 200,
            HandType::OnePair => 100,
            HandType::HighCard(c) => c, 
        }
    }

    fn compare_individual(&self, other: &Hand) -> Option<Ordering> {
        for (i, c) in self.cards.iter().enumerate() {
            match c.value.cmp(&other.cards[i].value) {
                Ordering::Greater => return Some(Ordering::Greater),
                Ordering::Less => return Some(Ordering::Less),
                _ => (),
            }
        }
        Some(Ordering::Equal)
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.value() == other.value()
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.value().cmp(&other.value()) {
            Ordering::Greater => Some(Ordering::Greater),
            Ordering::Less => Some(Ordering::Less),
            Ordering::Equal => self.compare_individual(other),
        }
    }
}

fn parse_cards(s: &str, joker: bool) -> Vec<Card> {
    s.chars()
     .map(|c| match c {
        'A' => Card {value: 14},
        'K' => Card {value: 13},
        'Q' => Card {value: 12},
        'J' => match joker {
            false => Card {value: 11},
            true => Card {value: 0}
        },
        'T' => Card {value: 10},
        '9' => Card {value: 9},
        '8' => Card {value: 8},
        '7' => Card {value: 7},
        '6' => Card {value: 6},
        '5' => Card {value: 5},
        '4' => Card {value: 4},
        '3' => Card {value: 3},
        '2' => Card {value: 2},
        _ => panic!("Not a valid card!")
     })
     .collect()
}

fn get_hand_type(cards: &[Card]) -> HandType {
    let mut counts_map = HashMap::<Card, i32>::new();
    let mut num_jokers = 0;
    cards.iter()
        .for_each(|c| {
            if c.value == 0 {
                num_jokers += 1
            } else {
                counts_map.entry((*c).to_owned()).and_modify(|curr| *curr += 1).or_insert(1);
            }
        });
    let mut values: Vec<i32> = counts_map.values().copied().collect::<Vec<i32>>();
    values.sort();

    if values.is_empty() {
        HandType::FiveKind
    } else {
        let first_val = values[values.len()-1] + num_jokers;
        if values.len() > 1 {
            let second_val = values[values.len()-2];
            match (first_val, second_val) {
                (4, 1) => HandType::FourKind,
                (3, 2) => HandType::FullHouse,
                (3, 1) => HandType::ThreeKind,
                (2, 2) => HandType::TwoPair,
                (2, 1) => HandType::OnePair,
                // WHY oh WHYYYY !!!!!
                (1, 1) => HandType::HighCard(0), // HandType::HighCard(cards.iter().max().unwrap().value),
                (_, _) => panic!("Unknown hand type"),
            }
        } else {
            HandType::FiveKind
        }
    }
}

fn solve_part_1(input: Vec<String>) -> i64 {
    let mut hands = input.iter()
        .map(|l| {
            let cards = parse_cards(l.split_whitespace().next().unwrap().trim(), false);
            let bid: i32 = l.split_whitespace().last().unwrap().trim().parse::<i32>().unwrap();
            let hand_type = get_hand_type(&cards);
            Hand {cards, bid, hand_type}
        })
        .collect::<Vec<Hand>>();
    hands.sort_by(|h1, h2| (*h1).partial_cmp(h2).unwrap());
    hands.iter().enumerate().map(|(i, h)| h.bid as i64 * (i as i64 + 1)).sum()
}

fn solve_part_2(input: Vec<String>) -> i64 {
    let mut hands = input.iter()
        .map(|l| {
            let cards = parse_cards(l.split_whitespace().next().unwrap().trim(), true);
            let bid: i32 = l.split_whitespace().last().unwrap().trim().parse::<i32>().unwrap();
            let hand_type = get_hand_type(&cards);
            Hand {cards, bid, hand_type}
        })
        .collect::<Vec<Hand>>();
    hands.sort_by(|h1, h2| (*h1).partial_cmp(h2).unwrap());
    hands.iter().enumerate().map(|(i, h)| h.bid as i64 * (i as i64 + 1)).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input.clone());
        let p2 = solve_part_2(input);
        assert_eq!(p1, 6440);
        assert_eq!(p2, 5905);
    }

    #[test]
    fn reddit_case() {
        let input = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input.clone());
        let p2 = solve_part_2(input);
        assert_eq!(p1, 6592);
        assert_eq!(p2, 6839);
    }
}
