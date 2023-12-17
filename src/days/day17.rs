use std::{fs, collections::HashMap};

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_17.txt")
        .expect("File not found")
        .lines()
        .map(|l: &str| l.to_string())
        .collect::<Vec<String>>();

    let p1 = solve_part_1(input.clone());
    let p2 = solve_part_2(input);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Path {
    straight_count: u8,
    pos: Pos,
    from: Direction,
    is_starting: bool,
    heat_loss: usize,
}


fn parse_map(input: &[String]) -> Vec<Vec<usize>> {
    input.iter()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10).unwrap() as usize
                })
                .collect()
        })
        .collect()
}

fn solve_part_1(input: Vec<String>) -> usize {
    let map = parse_map(&input);
    let mut goal_heat_loss = (map.len()+map[0].len())*9;
    let mut current_positions = vec![Path{pos: Pos {x: 0, y: 0}, is_starting: true, straight_count: 0, heat_loss: 0, from: Direction::North }]; 
    let mut visited = HashMap::<(Pos, Direction, u8), usize>::new();

    while !current_positions.is_empty() {
        let max_ele = current_positions.iter()
                    .min_by(|p1, p2| 
                            p1.heat_loss.cmp(&p2.heat_loss))
                    .unwrap();
        let current_position = current_positions.remove(current_positions.iter().position(|x| x == max_ele).unwrap());
        if current_position.pos.x == map[0].len() as i32 - 1 && current_position.pos.y == map.len() as i32 - 1 {
            if goal_heat_loss >= current_position.heat_loss {
                println!("new best: {} -> {}", goal_heat_loss, current_position.heat_loss);
                goal_heat_loss = current_position.heat_loss;
            } else {
                println!("end : {} ({})", current_position.heat_loss, current_positions.len());
            }
        } else if current_position.heat_loss + (map.len() + map[0].len() - current_position.pos.x as usize- current_position.pos.y as usize) < goal_heat_loss && 
                !(current_position.pos.x == 0 && current_position.pos.y == 0 && !current_position.is_starting){
            match visited.get(&(current_position.pos.clone(), current_position.from.clone(), current_position.straight_count)) {
                Some(x) => {
                    if current_position.heat_loss < *x {
                        add_moves(&current_position, &mut current_positions, &map);
                        let _ = visited.insert((current_position.pos, current_position.from, current_position.straight_count), current_position.heat_loss);
                    }
                },
                None => {
                    add_moves(&current_position, &mut current_positions, &map);
                    visited.insert((current_position.pos, current_position.from, current_position.straight_count), current_position.heat_loss);
                },
            }
        }
    }

    goal_heat_loss
}

fn add_moves(cp: &Path, current_positions: &mut Vec<Path>, map: &[Vec<usize>]) {
    let mut moves = Vec::<Path>::new();
    
    if cp.is_starting {
        current_positions.push( 
            Path { straight_count: 1, 
                pos: Pos { x: cp.pos.x, y: cp.pos.y + 1}, 
                from: Direction::North, 
                is_starting: false, 
                heat_loss: map[cp.pos.y as usize + 1][cp.pos.x as usize] }
        );
        current_positions.push( 
            Path { straight_count: 1, 
                pos: Pos { x: cp.pos.x + 1, y: cp.pos.y }, 
                from: Direction::West, 
                is_starting: false, 
                heat_loss: map[cp.pos.y as usize][cp.pos.x as usize + 1]}
        );
    } else {
        let st = 1;
        match cp.from {
            Direction::North => {
                if cp.pos.y + 1 < map.len() as i32 && cp.straight_count < 3 {
                    moves.push( 
                        Path { straight_count: cp.straight_count + 1, 
                            pos: Pos { x: cp.pos.x, y: cp.pos.y + 1}, 
                            from: Direction::North, 
                            is_starting: false, 
                            heat_loss: 0}
                    );
                }
                if cp.pos.x + 1 < map[0].len() as i32 {
                    moves.push( 
                        Path { straight_count: st, 
                            pos: Pos { x: cp.pos.x + 1, y: cp.pos.y }, 
                            from: Direction::West, 
                            is_starting: false, 
                            heat_loss: 0}
                    );
                }
                if cp.pos.x >= 1 {
                    moves.push( 
                        Path { straight_count: st, 
                            pos: Pos { x: cp.pos.x - 1, y: cp.pos.y }, 
                            from: Direction::East, 
                            is_starting: false, 
                            heat_loss: 0}
                    );
                }
            },
            Direction::South => {
                if cp.pos.y >= 1 && cp.straight_count < 3 {
                    moves.push( 
                        Path { straight_count: cp.straight_count + 1, 
                            pos: Pos { x: cp.pos.x, y: cp.pos.y - 1}, 
                            from: Direction::South, 
                            is_starting: false, 
                            heat_loss: 0}
                    );
                }
                if cp.pos.x + 1 < map[0].len() as i32 {
                    moves.push( 
                        Path { straight_count: st, 
                            pos: Pos { x: cp.pos.x + 1, y: cp.pos.y }, 
                            from: Direction::West, 
                            is_starting: false, 
                            heat_loss: 0}
                    );
                }
                if cp.pos.x >= 1 {
                    moves.push( 
                        Path { straight_count: st, 
                            pos: Pos { x: cp.pos.x - 1, y: cp.pos.y }, 
                            from: Direction::East, 
                            is_starting: false, 
                            heat_loss: 0}
                    );
                }
            },
            Direction::West => { 
                if cp.pos.y + 1 < map.len() as i32 {
                    moves.push( 
                        Path { straight_count: st,
                            pos: Pos { x: cp.pos.x, y: cp.pos.y + 1}, 
                            from: Direction::North, 
                            is_starting: false, 
                            heat_loss: 0}
                    );
                }
                if cp.pos.y >= 1 {
                    moves.push( 
                        Path { straight_count: st,
                            pos: Pos { x: cp.pos.x, y: cp.pos.y - 1}, 
                            from: Direction::South, 
                            is_starting: false, 
                            heat_loss: 0}
                    );
                }
                if cp.pos.x + 1 < map[0].len() as i32 && cp.straight_count < 3 {
                    moves.push( 
                        Path { straight_count: cp.straight_count + 1, 
                            pos: Pos { x: cp.pos.x + 1, y: cp.pos.y }, 
                            from: Direction::West, 
                            is_starting: false, 
                            heat_loss: 0}
                    );
                }
            },
            Direction::East => {
                if cp.pos.y + 1 < map.len() as i32 {
                    moves.push( 
                        Path { straight_count: st,
                            pos: Pos { x: cp.pos.x, y: cp.pos.y + 1}, 
                            from: Direction::North, 
                            is_starting: false, 
                            heat_loss: 0}
                    );
                }
                if cp.pos.y >= 1 {
                    moves.push( 
                        Path { straight_count: st,
                            pos: Pos { x: cp.pos.x, y: cp.pos.y - 1}, 
                            from: Direction::South, 
                            is_starting: false, 
                            heat_loss: 0}
                    );
                }
                if cp.pos.x >= 1 && cp.straight_count < 3 {
                    moves.push( 
                        Path { straight_count: cp.straight_count + 1, 
                            pos: Pos { x: cp.pos.x - 1, y: cp.pos.y }, 
                            from: Direction::East, 
                            is_starting: false, 
                            heat_loss: 0}
                    );
                }
            },
        }
        moves.iter_mut()
            .for_each(|m| {
                m.heat_loss = cp.heat_loss + map[m.pos.y as usize][m.pos.x as usize];
            });
        current_positions.append(&mut moves);
    }
}

fn add_moves_ultra(cp: &Path, current_positions: &mut Vec<Path>, map: &[Vec<usize>]) {
    let mut moves = Vec::<Path>::new();
    
    let st = 4;
    let st_thr = 10;
    if cp.is_starting {
        current_positions.push( 
            Path { straight_count: st as u8, 
                pos: Pos { x: cp.pos.x, y: cp.pos.y + st}, 
                from: Direction::North, 
                is_starting: false, 
                heat_loss: cp.heat_loss + ((cp.pos.y+1) as usize..=(cp.pos.y+st) as usize).fold(0, |acc, y| acc + map[y][cp.pos.x as usize])}
        );
        current_positions.push( 
            Path { straight_count: st as u8, 
                pos: Pos { x: cp.pos.x + st, y: cp.pos.y }, 
                from: Direction::West, 
                is_starting: false, 
                heat_loss: cp.heat_loss + ((cp.pos.x+1) as usize..=(cp.pos.x + st) as usize).fold(0, |acc, x| acc + map[cp.pos.y as usize][x]) }
        );
    } else {
        match cp.from {
            Direction::North => {
                if cp.pos.y + 1 < map.len() as i32 && cp.straight_count < st_thr {
                    moves.push( 
                        Path { straight_count: cp.straight_count + 1, 
                            pos: Pos { x: cp.pos.x, y: cp.pos.y + 1},
                            from: Direction::North, 
                            is_starting: false, 
                            heat_loss: cp.heat_loss + map[cp.pos.y as usize + 1][cp.pos.x as usize]}
                    );
                }
                if cp.pos.x + st < map[0].len() as i32 {
                    moves.push( 
                        Path { straight_count: st as u8, 
                            pos: Pos { x: cp.pos.x + st, y: cp.pos.y }, 
                            from: Direction::West, 
                            is_starting: false, 
                            heat_loss: cp.heat_loss + ((cp.pos.x+1) as usize..=(cp.pos.x + st) as usize).fold(0, |acc, x| acc + map[cp.pos.y as usize][x]) }
                    );
                }
                if cp.pos.x >= st {
                    moves.push( 
                        Path { straight_count: st as u8, 
                            pos: Pos { x: cp.pos.x - st, y: cp.pos.y }, 
                            from: Direction::East, 
                            is_starting: false, 
                            heat_loss: cp.heat_loss + ((cp.pos.x-st) as usize..cp.pos.x as usize).fold(0, |acc, x| acc + map[cp.pos.y as usize][x]) }
                    );
                }
            },
            Direction::South => {
                if cp.pos.y >= 1 && cp.straight_count < st_thr {
                    moves.push( 
                        Path { straight_count: cp.straight_count + 1, 
                            pos: Pos { x: cp.pos.x, y: cp.pos.y - 1}, 
                            from: Direction::South, 
                            is_starting: false, 
                            heat_loss: cp.heat_loss + map[cp.pos.y as usize - 1][cp.pos.x as usize]}
                    );
                }
                if cp.pos.x + st < map[0].len() as i32 {
                    moves.push( 
                        Path { straight_count: st as u8, 
                            pos: Pos { x: cp.pos.x + st, y: cp.pos.y }, 
                            from: Direction::West, 
                            is_starting: false, 
                            heat_loss: cp.heat_loss + ((cp.pos.x+1) as usize..=(cp.pos.x + st) as usize).fold(0, |acc, x| acc + map[cp.pos.y as usize][x]) }
                    );
                }
                if cp.pos.x >= st {
                    moves.push( 
                        Path { straight_count: st as u8, 
                            pos: Pos { x: cp.pos.x - st, y: cp.pos.y }, 
                            from: Direction::East, 
                            is_starting: false, 
                            heat_loss: cp.heat_loss + ((cp.pos.x-st) as usize..cp.pos.x as usize).fold(0, |acc, x| acc + map[cp.pos.y as usize][x]) }
                    );
                }
            },
            Direction::West => { 
                if cp.pos.y + st < map.len() as i32 {
                    moves.push( 
                        Path { straight_count: st as u8,
                            pos: Pos { x: cp.pos.x, y: cp.pos.y + st}, 
                            from: Direction::North, 
                            is_starting: false, 
                            heat_loss: cp.heat_loss + ((cp.pos.y+1) as usize..=(cp.pos.y+st) as usize).fold(0, |acc, y| acc + map[y][cp.pos.x as usize])}
                    );
                }
                if cp.pos.y >= st {
                    moves.push( 
                        Path { straight_count: st as u8,
                            pos: Pos { x: cp.pos.x, y: cp.pos.y - st}, 
                            from: Direction::South, 
                            is_starting: false, 
                            heat_loss: cp.heat_loss + ((cp.pos.y-st) as usize..cp.pos.y as usize).fold(0, |acc, y| acc + map[y][cp.pos.x as usize])}
                    );
                }
                if cp.pos.x + 1 < map[0].len() as i32 && cp.straight_count < st_thr {
                    moves.push( 
                        Path { straight_count: cp.straight_count + 1, 
                            pos: Pos { x: cp.pos.x + 1, y: cp.pos.y }, 
                            from: Direction::West, 
                            is_starting: false, 
                            heat_loss: cp.heat_loss + map[cp.pos.y as usize][cp.pos.x as usize + 1]}
                    );
                }
            },
            Direction::East => {
                if cp.pos.y + st < map.len() as i32 {
                    moves.push( 
                        Path { straight_count: st as u8,
                            pos: Pos { x: cp.pos.x, y: cp.pos.y + st}, 
                            from: Direction::North, 
                            is_starting: false, 
                            heat_loss: cp.heat_loss + ((cp.pos.y+1) as usize..=(cp.pos.y+st) as usize).fold(0, |acc, y| acc + map[y][cp.pos.x as usize])}
                    );
                }
                if cp.pos.y >= st {
                    moves.push( 
                        Path { straight_count: st as u8,
                            pos: Pos { x: cp.pos.x, y: cp.pos.y - st}, 
                            from: Direction::South, 
                            is_starting: false, 
                            heat_loss: cp.heat_loss + ((cp.pos.y-st) as usize..cp.pos.y as usize).fold(0, |acc, y| acc + map[y][cp.pos.x as usize])}
                    );
                }
                if cp.pos.x >= 1 && cp.straight_count < st_thr {
                    moves.push( 
                        Path { straight_count: cp.straight_count + 1, 
                            pos: Pos { x: cp.pos.x - 1, y: cp.pos.y }, 
                            from: Direction::East, 
                            is_starting: false, 
                            heat_loss: cp.heat_loss + map[cp.pos.y as usize][cp.pos.x as usize - 1]}
                    );
                }
            },
        }
        current_positions.append(&mut moves);
    }
}

fn solve_part_2(input: Vec<String>) -> usize {
    let map = parse_map(&input);
    let mut goal_heat_loss = (map.len()+map[0].len())*9;
    let mut current_positions = vec![Path{pos: Pos {x: 0, y: 0}, is_starting: true, straight_count: 0, heat_loss: 0, from: Direction::North }]; 
    let mut visited = HashMap::<(Pos, Direction, u8), usize>::new();

    while !current_positions.is_empty() {
        let max_ele = current_positions.iter()
                    .min_by(|p1, p2| 
                            p1.heat_loss.cmp(&p2.heat_loss))
                    .unwrap();
        let current_position = current_positions.remove(current_positions.iter().position(|x| x == max_ele).unwrap());
        if current_position.pos.x == map[0].len() as i32 - 1 && current_position.pos.y == map.len() as i32 - 1 {
            if goal_heat_loss >= current_position.heat_loss {
                println!("new best: {} -> {}", goal_heat_loss, current_position.heat_loss);
                goal_heat_loss = current_position.heat_loss;
            } else {
                println!("end : {} ({})", current_position.heat_loss, current_positions.len());
            }
        } else if current_position.heat_loss + (map.len() + map[0].len() - current_position.pos.x as usize- current_position.pos.y as usize) < goal_heat_loss && 
                !(current_position.pos.x == 0 && current_position.pos.y == 0 && !current_position.is_starting){
            match visited.get(&(current_position.pos.clone(), current_position.from.clone(), current_position.straight_count)) {
                Some(x) => {
                    if current_position.heat_loss < *x {
                        add_moves_ultra(&current_position, &mut current_positions, &map);
                        let _ = visited.insert((current_position.pos, current_position.from, current_position.straight_count), current_position.heat_loss);
                    }
                },
                None => {
                    add_moves_ultra(&current_position, &mut current_positions, &map);
                    visited.insert((current_position.pos, current_position.from, current_position.straight_count), current_position.heat_loss);
                },
            }
        }
    }

    goal_heat_loss
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input.clone());
        let p2 = solve_part_2(input);
        assert_eq!(p1, 102);
        assert_eq!(p2, 94);
    }
}
