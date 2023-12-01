use std::fs;

pub fn solve() {
    let input: Vec<String> = fs::read_to_string("./inputs/input_1.txt")
        .expect("File not found")
        .lines()
        .map(|l| l.to_string())
        .collect();


    let p1 = solve_part_1(input.clone());
    let p2 = solve_part_2(input);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

fn solve_part_1(input: Vec<String>) -> i32 {
    input.iter()
        .map(|l| {
                l.chars()
                 .filter(|c| c.is_numeric())
                 .collect()
            })
        .collect::<Vec<Vec<char>>>()
        .iter()
        .filter(|v| !v.is_empty())
        .map(|v| {
            let id = match v.len() {
                1 => 0,
                _ => v.len()-1,
            };
            (String::from(v[0]) + &v[id].to_string()).parse::<i32>().unwrap()
    }).sum()
}

fn solve_part_2(input: Vec<String>) -> i32 {
    let strnum = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let strnum_int = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let input_replaced = input.iter()
        .map(|l| {
            let mut str_num = String::new();
            for (i, c) in l.chars().enumerate(){
                match c.is_numeric() {
                    true => str_num.push(c),
                    false => {
                        for (j, v) in strnum.iter().enumerate() {
                            if let Some(x) = l[i..].find(v) {
                                if x == 0 {
                                    str_num.push_str(strnum_int[j]);
                                    break;
                                }
                            };
                        }
                    }
                };
            }
            str_num.chars().collect()
        }).collect();
    solve_part_1(input_replaced)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input1 = ["1abc2" ,"pqr3stu8vwx" ,"a1b2c3d4e5f" ,"treb7uchet"]
            .iter()
            .map(|l| l.to_string())
            .collect();
        let p1 = solve_part_1(input1);
        assert_eq!(p1, 142);

        let input2 = ["two1nine" ,"eightwothree" ,"abcone2threexyz" ,"xtwone3four" ,"4nineeightseven2" ,"zoneight234" ,"7pqrstsixteen"]
            .iter()
            .map(|l| l.to_string())
            .collect();
        let p2 = solve_part_2(input2);
        assert_eq!(p2, 281);
    }
}
