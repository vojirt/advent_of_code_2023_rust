use std::fs;

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_15.txt")
        .expect("File not found")
        .lines()
        .map(|l: &str| l.to_string())
        .collect::<Vec<String>>();

    let p1 = solve_part_1(&input[0]);
    let p2 = solve_part_2(&input[0]);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}


fn hash_str(x: &str) -> u32 {
    x.chars()
     .fold(0, |acc, c| {
         17*(acc + (c as u32)) % 256
     })
}

fn solve_part_1(input: &str) -> u32 {
    input.split(',')
        .map(hash_str)
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u32,
}

fn focusing_power(boxes: &[Vec<Lens>]) -> u32 {
    boxes.iter()
        .enumerate()
        .map(|(bid, boxi)| {
            boxi.iter()
               .enumerate()
               .map(|(lid, l)| {
                   (bid as u32 +1) * (lid as u32 + 1) * l.focal_length
               })
               .sum::<u32>()
        })
        .sum()
}

fn solve_part_2(input: &str) -> u32 {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    input.split(',')
        .for_each(|cmd| {
            let label: &str;
            let mut focal_length: Option<u32> = None;
            if let Some(id) = cmd.find('=') {
                label = &cmd[..id]; 
                focal_length = cmd[id+1..].parse::<u32>().ok();
            } else if let Some(id) = cmd.find('-') {
                label = &cmd[..id]; 
            } else {
                panic!("Parsing error = invalid command!");
            }

            let box_id = hash_str(label);
            let index = boxes[box_id as usize].iter().position(|b| b.label == label);
            if let Some(fl) = focal_length {
                if let Some(bid) = index {
                    boxes[box_id as usize][bid].focal_length = fl;
                } else {
                    boxes[box_id as usize].push(Lens { label, focal_length: fl });
                }
            } else if let Some(bid) = index {
                boxes[box_id as usize].remove(bid);
            }
        });

    focusing_power(&boxes)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();
        let p1 = solve_part_1(&input);
        let p2 = solve_part_2(&input);
        assert_eq!(p1, 1320);
        assert_eq!(p2, 145);
    }
}
