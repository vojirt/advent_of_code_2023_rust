use std::{fs, collections::HashMap};

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_16.txt")
        .expect("File not found")
        .lines()
        .map(|l: &str| l.to_string())
        .collect::<Vec<String>>();

    let p1 = solve_part_1(input.clone());
    let p2 = solve_part_2(input);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

enum NodeType {
    Empty,
    MirrorSlash,
    MirrorBackSlash,
    SplitterPipe,
    SplitterDash,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn move_in_dir(&self, dir: &Direction, map: &[Vec<NodeType>]) -> Option<Pos> {
        match dir {
            Direction::North => if self.y > 0 {Some(Pos {x: self.x, y: self.y - 1})} else {None},
            Direction::South => if (self.y as usize) < map.len()-1 {Some(Pos {x: self.x, y: self.y + 1})} else {None},
            Direction::West  => if self.x > 0 {Some(Pos {x: self.x - 1, y: self.y})} else {None},
            Direction::East  => if (self.x as usize) < map[0].len()-1 {Some(Pos {x: self.x + 1, y: self.y})} else {None},
        }
    }
    fn move_on_map(&self, dir: &Direction, map: &[Vec<NodeType>]) -> Vec<(Direction, Option<Pos>)> {
        match map[self.y as usize][self.x as usize] {
            NodeType::Empty => vec![(dir.clone(), self.move_in_dir(dir, map))],
            NodeType::MirrorSlash => {
                match dir {
                    Direction::South => vec![(Direction::West, self.move_in_dir(&Direction::West, map))],
                    Direction::North => vec![(Direction::East, self.move_in_dir(&Direction::East, map))],
                    Direction::East  => vec![(Direction::North, self.move_in_dir(&Direction::North, map))],
                    Direction::West  => vec![(Direction::South, self.move_in_dir(&Direction::South, map))],
                }
            },
            NodeType::MirrorBackSlash => {
                match dir {
                    Direction::South => vec![(Direction::East, self.move_in_dir(&Direction::East, map))],
                    Direction::North => vec![(Direction::West, self.move_in_dir(&Direction::West, map))],
                    Direction::East  => vec![(Direction::South, self.move_in_dir(&Direction::South, map))],
                    Direction::West  => vec![(Direction::North, self.move_in_dir(&Direction::North, map))],
                }
            },
            NodeType::SplitterPipe => {
                match dir {
                    Direction::North | Direction::South => vec![(dir.clone(), self.move_in_dir(dir, map))],
                    Direction::West | Direction::East  => vec![(Direction::North, self.move_in_dir(&Direction::North, map)), 
                                                               (Direction::South, self.move_in_dir(&Direction::South, map))],
                }
            },
            NodeType::SplitterDash => {
                match dir {
                   Direction::West | Direction::East => vec![(dir.clone(), self.move_in_dir(dir, map))],
                   Direction::North | Direction::South => vec![(Direction::East, self.move_in_dir(&Direction::East, map)), 
                                                               (Direction::West, self.move_in_dir(&Direction::West, map))],
                }
            },
        }
    }
}



fn parse_map(input: &[String]) -> Vec<Vec<NodeType>> {
    input.iter()
        .map(|line| {
            line.chars()
                .map(|c| {
                    match c {
                        '.' => NodeType::Empty,
                        '/' => NodeType::MirrorSlash,
                        '\\' => NodeType::MirrorBackSlash,
                        '-' => NodeType::SplitterDash,
                        '|' => NodeType::SplitterPipe,
                        _ => panic!("Unknown symbol in parsing!")
                    }
                })
                .collect()
        })
        .collect()
}

fn solve_part_1(input: Vec<String>) -> i32 {
    let map = parse_map(&input);
    let mut visited = vec![vec![0_i32; map[0].len()]; map.len()];
    let mut loops = HashMap::<(Direction, Pos), bool>::new();
    let mut current_positions = vec![(Direction::East, Pos {x: 0, y: 0})];
    while !current_positions.is_empty() {
        let current_position = current_positions.remove(0);
        visited[(current_position.1).y as usize][(current_position.1).x as usize] += 1;
        if loops.get(&current_position).is_none() {
            let moves = (current_position.1).move_on_map(&current_position.0, &map);
            moves.iter().for_each(|p| {
                if let Some(x) = &p.1 {
                    current_positions.push((p.0.clone(), x.clone())); 
                }
            });
        }
        loops.insert(current_position, true);
    }

    visited.iter()
        .flatten()
        .filter(|v| **v > 0)
        .count() as i32
}

fn solve_part_2(input: Vec<String>) -> i32 {
    let map = parse_map(&input);
    let mut starting_positions = Vec::<(Direction, Pos)>::new();
    (0..map.len()).for_each(|y| {
        starting_positions.push((Direction::East, Pos {x: 0, y: y as i32}));
        starting_positions.push((Direction::West, Pos {x: map[0].len() as i32 -1, y: y as i32}));
    });
    (0..map[0].len()).for_each(|x| {
        starting_positions.push((Direction::South, Pos {x: x as i32, y: 0}));
        starting_positions.push((Direction::North, Pos {x: x as i32, y: map.len() as i32 -1}));
    });
    
    starting_positions.iter()
        .map(|sp| {
            let mut visited = vec![vec![0_i32; map[0].len()]; map.len()];
            let mut loops = HashMap::<(Direction, Pos), bool>::new();
            let mut current_positions = vec![sp.clone()];
            while !current_positions.is_empty() {
                let current_position = current_positions.remove(0);
                visited[(current_position.1).y as usize][(current_position.1).x as usize] += 1;
                if loops.get(&current_position).is_none() {
                    let moves = (current_position.1).move_on_map(&current_position.0, &map);
                    moves.iter().for_each(|p| {
                        if let Some(x) = &p.1 {
                            current_positions.push((p.0.clone(), x.clone())); 
                        }
                    });
                }
                loops.insert(current_position, true);
            }

            visited.iter()
                .flatten()
                .filter(|v| **v > 0)
                .count() as i32
        })
        .max()
        .unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input.clone());
        let p2 = solve_part_2(input);
        assert_eq!(p1, 46);
        assert_eq!(p2, 51);
    }
}
