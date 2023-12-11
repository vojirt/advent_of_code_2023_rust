use std::fs;

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_10.txt")
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
struct Location {
    start: bool,
    dist_from_start: i32,
    conn: Vec<Connection>,
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, PartialEq)]
enum Connection {
    North,
    South,
    West,
    East,
}

fn parse_map(input: &Vec<String>) -> (Vec<Vec<Location>>, [usize; 2])  {
    let rows = input.len();
    let cols = input[0].len();

    let mut map = Vec::<Vec<Location>>::new();
    let mut r_start = 0;
    let mut c_start = 0;

    (0..rows).for_each(|y| {
            let mut row_vec = Vec::<Location>::new();
            (0..cols).for_each(|x| {
                let start = matches!(input[y].chars().nth(x).unwrap(), 'S');
                row_vec.push(
                      Location { start, dist_from_start: -1, conn: vec![], row: y, col: x}
                );
                if start {
                    r_start = y;
                    c_start = x;
                }
            });
            map.push(row_vec);
        });

    input.iter()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars()
                .enumerate()
                .for_each(|(x, c)| {
                    match c {
                        '|' => add_connection(&mut map, y, x, &[Connection::North, Connection::South]), 
                        '-' => add_connection(&mut map, y, x, &[Connection::West, Connection::East]), 
                        'L' => add_connection(&mut map, y, x, &[Connection::North, Connection::East]), 
                        'J' => add_connection(&mut map, y, x, &[Connection::North, Connection::West]), 
                        '7' => add_connection(&mut map, y, x, &[Connection::South, Connection::West]), 
                        'F' => add_connection(&mut map, y, x, &[Connection::South, Connection::East]), 
                        '.' => add_connection(&mut map, y, x, &[]), 
                        'S' => {
                            add_connection(&mut map, y, x, &[]);
                            map[y][x].start = true;
                            map[y][x].dist_from_start = 0;
                        },
                        _ => panic!("Invalid character in map!"),
                    }
                });
        });
    (map, [r_start, c_start])
}

fn add_connection(map: &mut [Vec<Location>], row: usize, col: usize, conn: &[Connection]) {
    conn.iter()
        .for_each(|c| {
            match c {
                Connection::North => {
                    map[row][col].conn.push(Connection::North);
                    if row > 0 {
                        let r = &mut map[row-1];
                        if r[col].start {
                            r[col].conn.push(Connection::South);
                        }
                    }
                },
                Connection::South => {
                    map[row][col].conn.push(Connection::South);
                    if let Some(r) = map.get_mut(row+1) {
                        if r[col].start {
                            r[col].conn.push(Connection::North);
                        }
                    }
                },
                Connection::West => {
                    map[row][col].conn.push(Connection::West);
                    if col > 0 {
                        let cl = &mut map[row][col-1]; 
                        if cl.start {
                            cl.conn.push(Connection::East);
                        }
                    }
                },
                Connection::East => {
                    map[row][col].conn.push(Connection::East);
                    if let Some(cl) = map[row].get_mut(col+1) {
                        if cl.start {
                            cl.conn.push(Connection::West);
                        }
                    }
                },
            };
        });
}


fn solve_part_1(input: Vec<String>) -> i32 {
    let (mut map, start) = parse_map(&input);
    
    // I encouter some issues with borrowing mut ref, so I just clone stuff here
    // simple depth-first search with termination assumption of loop
    let mut nodes = vec![start];
    while let Some(loc) = nodes.pop() {
        let current_node = map[loc[0]][loc[1]].clone();
        for c in current_node.conn.iter() {
            let next_node = match c {
                    Connection::North => {
                        if current_node.row > 0 {
                            Some(&mut map[current_node.row-1][current_node.col])
                        } else {
                            None
                        }
                    },
                    Connection::South => {
                        if let Some(r) = map.get_mut(current_node.row+1) {
                            Some(&mut r[current_node.col])
                        } else {
                            None
                        }

                    },
                    Connection::West => {
                        if current_node.col > 0 {
                            Some(&mut map[current_node.row][current_node.col - 1])
                        } else {
                            None
                        }
                    }
                    Connection::East => map[current_node.row].get_mut(current_node.col+1) ,
                };
            if let Some(node) = next_node {
                if !node.start && ((node.dist_from_start < 0) | (node.dist_from_start > current_node.dist_from_start + 1)) {
                    node.dist_from_start = current_node.dist_from_start + 1;
                    nodes.push([node.row, node.col]);
                }
            }
        }
    }
    map.iter()
       .flatten()
       .filter(|n| !n.start)
       .map(|n| n.dist_from_start)
       .max().unwrap()
}

fn solve_part_2(input: Vec<String>) -> i32 {
    let (mut map, start) = parse_map(&input);
    
    let mut nodes = vec![start];
    while let Some(loc) = nodes.pop() {
        let current_node = map[loc[0]][loc[1]].clone();
        for c in current_node.conn.iter() {
            let next_node = match c {
                    Connection::North => {
                        if current_node.row > 0 {
                            Some(&mut map[current_node.row-1][current_node.col])
                        } else {
                            None
                        }
                    },
                    Connection::South => {
                        if let Some(r) = map.get_mut(current_node.row+1) {
                            Some(&mut r[current_node.col])
                        } else {
                            None
                        }

                    },
                    Connection::West => {
                        if current_node.col > 0 {
                            Some(&mut map[current_node.row][current_node.col - 1])
                        } else {
                            None
                        }
                    }
                    Connection::East => map[current_node.row].get_mut(current_node.col+1) ,
                };
            if let Some(node) = next_node {
                if !node.start && ((node.dist_from_start < 0) | (node.dist_from_start > current_node.dist_from_start + 1)) {
                    node.dist_from_start = current_node.dist_from_start + 1;
                    nodes.push([node.row, node.col]);
                }
            }
        }
    }

    let rows = map.len();
    let cols = map[0].len();
    let mut in_mask: Vec<Vec<bool>> = vec![false; rows * cols].as_mut_slice()
        .chunks_mut(cols)
        .map(Vec::<bool>::from)
        .collect::<Vec<_>>();

    // find indexes where the path is "crossing row" and returning 
    // (=> between these ranges its inside)
    let ranges = map.iter()
       .map(|line| {
           let idx = line.iter()
               .filter(|loc| loc.dist_from_start >= 0 && 
                       (loc.conn.contains(&Connection::North) || loc.conn.contains(&Connection::South)))
               .map(|loc| {
                   loc.col
               }).collect::<Vec<usize>>();
            let mut out: Vec<usize> = vec![];
            if idx.len() > 2 {
                out.push(*idx.first().unwrap());
                let init_dir = match line[out[0]].conn.contains(&Connection::North) {
                    true => Connection::North,
                    false => Connection::South,
                };
                (1..idx.len()).for_each(|i| {
                    if line[idx[i]].conn.contains(&init_dir) {
                        out.push(idx[i]);
                    } 
                });
            }
            out
       })
       .collect::<Vec<Vec<usize>>>();

    // debug prints
    // (0..rows).for_each(|y| {
    //     (0..cols).for_each(|x| {
    //         print!("{: ^4}", map[y][x].dist_from_start);
    //     });
    //     println!();
    // });
    // println!();
    // (0..rows).for_each(|y| {
    //     (0..cols).for_each(|x| {
    //         if ranges[y].contains(&x) {
    //             print!("{: ^4}", x);
    //         } else {
    //             print!("{: ^4}", '.');
    //         }
    //     });
    //     println!();
    // });

    ranges.iter()
        .enumerate()
        .filter(|(_, r)| !r.is_empty())
        .for_each(|(y, r)| {
            (0..r.len()-1).for_each(|i| {
                let left = r[i];
                let right = r[i+1];
                if (i+1) % 2 == 1 {
                    (left..right).for_each(|c| {
                        if map[y][c].dist_from_start < 0 {
                            in_mask[y][c] = true
                        }
                    });
                }
            });
        });

    // debug prints
    // println!();
    // (0..rows).for_each(|y| {
    //     (0..cols).for_each(|x| {
    //         match in_mask[y][x] {
    //             true => print!("{: ^4}", 'I'),
    //             false => 
    //                 if map[y][x].dist_from_start >= 0 {
    //                     print!("{: ^4}", '.');
    //                 } else {
    //                     print!("{: ^4}", 'O');
    //                 },
    //         }
    //     });
    //     print!("\n");
    // });

    in_mask.iter()
        .flatten()
        .filter(|b| **b)
        .count() as i32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case_p1() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input);
        assert_eq!(p1, 4);
    }

    #[test]
    fn simple_case_p1_2() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input);
        assert_eq!(p1, 8);
    }

    #[test]
    fn simple_case_p2() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p2 = solve_part_2(input);
        assert_eq!(p2, 4);
    }

    #[test]
    fn simple_case_p2_2() {
        let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p2 = solve_part_2(input);
        assert_eq!(p2, 4);
    }

    #[test]
    fn simple_case_p2_large() {
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p2 = solve_part_2(input);
        assert_eq!(p2, 8);
    }

    #[test]
    fn simple_case_p2_large2() {
        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p2 = solve_part_2(input);
        assert_eq!(p2, 10);
    }
}
