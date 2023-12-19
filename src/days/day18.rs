use std::{fs, collections::HashMap};

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_18.txt")
        .expect("File not found")
        .lines()
        .map(|l: &str| l.to_string())
        .collect::<Vec<String>>();

    let p1 = solve_part_1(input.clone());
    let p2 = solve_part_2(input);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Empty,
    Start,
}

#[derive(Debug)]
struct Instruction {
    dir: Direction,
    num_steps: u32,
    color: u32, 
}

fn solve_part_1(input: Vec<String>) -> i64 {
    let mut max_y:i64 = 0;
    let mut max_x:i64 = 0;
    let mut min_y:i64 = 0;
    let mut min_x:i64 = 0;
    let mut x:i64 = 0;
    let mut y:i64 = 0;
    let instructions = input.iter()
        .map(|line| {
            let mut line_split = line.split_whitespace();
            let instr = Instruction {dir: match line_split.next().unwrap() {
                    "D" => Direction::Down,
                    "U" => Direction::Up,
                    "L" => Direction::Left,
                    "R" => Direction::Right,
                    _ => panic!("Invalid direction!"),
                },
                num_steps: line_split.next().unwrap().parse::<u32>().unwrap(),
                color: u32::from_str_radix(&line_split.next().unwrap()[2..8], 16).unwrap(),
            };
            match instr.dir {
                Direction::Down => y += instr.num_steps as i64,
                Direction::Up => y -= instr.num_steps as i64,
                Direction::Left => x -= instr.num_steps as i64,
                Direction::Right => x += instr.num_steps as i64,
                _ => (),
            };
            max_y = max_y.max(y);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            min_x = min_x.min(x);

            instr
        })
        .collect::<Vec<Instruction>>();
    let empty_instruction = Instruction {dir: Direction::Empty, num_steps: 0, color: 0};
    let start_instruction = Instruction {dir: Direction::Start, num_steps: 0, color: 0};
    let mut map = vec![vec![&empty_instruction; (max_x-min_x+1) as usize]; (max_y-min_y+1) as usize];
    let mut map_dig = vec![vec![false; (max_x-min_x+1) as usize]; (max_y-min_y+1) as usize];

    let mut current_loc = (min_y.abs() as usize, min_x.abs() as usize);
    map[current_loc.0][current_loc.1] = &start_instruction;
    map_dig[current_loc.0][current_loc.1] = true;
    instructions.iter()
        .for_each(|instr| {
            match instr.dir {
                Direction::Down => {
                    (current_loc.0..=current_loc.0+instr.num_steps as usize).for_each(|y| {
                        map[y][current_loc.1] = &instr;
                        map_dig[y][current_loc.1] = true;
                    });
                    current_loc.0 += instr.num_steps as usize;
                },
                Direction::Up => {
                    (current_loc.0-instr.num_steps as usize..=current_loc.0).for_each(|y| {
                        map[y][current_loc.1] = &instr;
                        map_dig[y][current_loc.1] = true;
                    });
                    current_loc.0 -= instr.num_steps as usize;
                },
                Direction::Left => {
                    (current_loc.1-instr.num_steps as usize..current_loc.1).for_each(|x| {
                        map[current_loc.0][x] = &instr;
                        map_dig[current_loc.0][x] = true;
                    });
                    current_loc.1 -= instr.num_steps as usize;
                },
                Direction::Right => {
                    (current_loc.1+1..=current_loc.1+instr.num_steps as usize).for_each(|x| {
                        map[current_loc.0][x] = &instr;
                        map_dig[current_loc.0][x] = true;
                    });
                    current_loc.1 += instr.num_steps as usize;
                },
                _ => (),
            }
        });

    // (0..map.len()).for_each(|y| {
    //     (0..map[0].len()).for_each(|x| {
    //         print!("{:}", match map_dig[y][x]{
    //             false => '.',
    //             true => '#',
    //         });
    //     });
    //     println!();
    // });

    let ranges = map.iter()
       .map(|line| {
           let idx = line.iter()
               .enumerate()
               .filter(|(_, loc)| loc.dir != Direction::Empty && 
                       (loc.dir == Direction::Up || loc.dir == Direction::Down))
               .map(|(x, _)| x).collect::<Vec<usize>>();

            let mut out: Vec<usize> = vec![];
            if idx.len() > 1 {
                out.push(idx[0]);
                let mut init_dir = match line[out[0]].dir == Direction::Down {
                    true => Direction::Down,
                    false => Direction::Up,
                };
                (1..idx.len()).for_each(|i| {
                    if line[idx[i]].dir != init_dir {
                        out.push(idx[i]);
                        init_dir = line[idx[i]].dir.clone(); 
                    } 
                });
            }
            out
       })
       .collect::<Vec<Vec<usize>>>();


    ranges.iter()
        .enumerate()
        .filter(|(_, r)| !r.is_empty())
        .for_each(|(y, r)| {
            (0..r.len()-1).for_each(|i| {
                let left = r[i];
                let right = r[i+1];
                if (i+1) % 2 == 1 {
                    (left..right).for_each(|c| {
                        map_dig[y][c] = true;
                    });
                }
            });
        });

    // (0..map.len()).for_each(|y| {
    //     (0..map[0].len()).for_each(|x| {
    //         print!("{:}", match map_dig[y][x]{
    //             false => '.',
    //             true => '#',
    //         });
    //     });
    //     println!();
    // });

    map_dig.iter()
        .flatten()
        .fold(0, |acc, c| {
            acc + if *c {1} else {0}
        })
}

fn solve_part_2(input: Vec<String>) -> i64 {
    let mut map = HashMap::<i64, Vec::<(i64, Direction)>>::new();
    let mut map_vertical = HashMap::<i64, i64>::new();
    let mut current_loc: (i64, i64) = (0, 0);
    input.iter()
        .for_each(|line| {
            // let mut line_split = line.split_whitespace();
            // let dir =  match line_split.next().unwrap() {
            //         "D" => Direction::Down,
            //         "U" => Direction::Up,
            //         "L" => Direction::Left,
            //         "R" => Direction::Right,
            //         _ => panic!("Invalid direction!"),
            //     };
            // let num_steps = line_split.next().unwrap().parse::<i64>().unwrap();

            let linestr= line.split_whitespace().last().unwrap();
            let dir = match linestr.chars().nth(7).unwrap() {
                    '0' => Direction::Right,
                    '1' => Direction::Down,
                    '2' => Direction::Left,
                    '3' => Direction::Up,
                    _ => panic!("Invalid direction!"),
                };
            let num_steps = i64::from_str_radix(&linestr[2..7], 16).unwrap();

            match dir {
                Direction::Down => {
                    (current_loc.0..=current_loc.0+num_steps).for_each(|y| {
                        if let Some(val) = map.get_mut(&y) {
                            val.push((current_loc.1, Direction::Down));
                        } else {
                            map.insert(y, vec![(current_loc.1, Direction::Down)]);
                        }
                    });
                    current_loc.0 += num_steps;
                },
                Direction::Up => {
                    (current_loc.0-num_steps..=current_loc.0).for_each(|y| {
                        if let Some(val) = map.get_mut(&y) {
                            val.push((current_loc.1, Direction::Up));
                        } else {
                            map.insert(y, vec![(current_loc.1, Direction::Up)]);
                        }
                    });
                    current_loc.0 -= num_steps;
                },
                Direction::Left => {
                    current_loc.1 -= num_steps;
                },
                Direction::Right => {
                    current_loc.1 += num_steps;
                },
                _ => (),
            }
        });

    map.iter()
       .fold(0, |acc, (y, vec_)| {
            let mut vec = vec_.clone();
            vec.sort_by(|a, b| (a.0).cmp(&b.0));
            vec.dedup();

            let mut c = 0;
            let mut indicator = 1;
            let mut init_dir = vec[0].1.clone();

            (1..vec.len()).for_each(|i| {
                if vec[i].1 != init_dir {
                    init_dir = vec[i].1.clone(); 
                    if indicator % 2 == 1 {
                        c += vec[i].0 - vec[i-1].0 + 1;
                    } else {
                        c += 1
                    }
                    indicator += 1;
                } 
                else if indicator % 2 == 1 || vec[i].1 == vec[i-1].1 {
                    c += vec[i].0 - vec[i-1].0;
                }
            });
            // dbg!(&vec);
            // println!("{y} : {c}");
            acc + c
       })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input.clone());
        let p2 = solve_part_2(input);
        assert_eq!(p1, 62);
        // assert_eq!(p2, 62);
        assert_eq!(p2, 952408144115);
    }
}
