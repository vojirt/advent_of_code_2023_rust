use std::{fs, collections::HashMap};

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_14.txt")
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
enum LocType {
    Empty, 
    Rounded,
    Cube,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Loc{
    inside: LocType,
    row: usize,
    col: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum TiltType {
    North,
    West,
    South,
    East,
}

fn print_platform(platform: &[Vec<Loc>]) {
    println!();
    (0..platform.len()).for_each(|y| {
        (0..platform[0].len()).for_each(|x| {
            // print!("{: ^4}", map[y][x].dist_from_start);
            print!("{:}", match platform[y][x].inside {
                    LocType::Empty => '.', 
                    LocType::Rounded => 'O',
                    LocType::Cube => '#',
            });
        });
        println!();
    });
}

fn solve_part_1(input: Vec<String>) -> i32 {
    let mut platform: Vec<Vec<Loc>> = parse_platform(&input);  
    tilt_platform(&mut platform, &TiltType::North);
    compute_total_beam_load(&platform, TiltType::North)
}

fn compute_total_beam_load(platform: &[Vec<Loc>], tilt_dir: TiltType) -> i32 {
    match tilt_dir {
        TiltType::North => {
            platform.iter()
                .enumerate()
                .fold(0, |acc, (y, row)| {
                    let count = row.iter()
                        .filter(|l| l.inside == LocType::Rounded)
                        .count();
                    acc + ((platform.len() - y) * count) as i32 
                })
        },
        _ => panic!("Not Implemented"),
    }
}

fn fill_interval(platform: &mut [Vec<Loc>], tilt_dir: &TiltType, interval_start: usize, interval_end: usize, num_to_fill: usize, rowcol: usize) {
    match tilt_dir {
        TiltType::North => {
            (interval_start..interval_start+num_to_fill).for_each(|y| platform[y][rowcol].inside = LocType::Rounded);
            (interval_start+num_to_fill..interval_end).for_each(|y| platform[y][rowcol].inside = LocType::Empty);
        },
        TiltType::South => {
            (interval_start..interval_end-num_to_fill).for_each(|y| platform[y][rowcol].inside = LocType::Empty);
            (interval_end-num_to_fill..interval_end).for_each(|y| platform[y][rowcol].inside = LocType::Rounded);
        },
        TiltType::West => {
            (interval_start..interval_start+num_to_fill).for_each(|x| platform[rowcol][x].inside = LocType::Rounded);
            (interval_start+num_to_fill..interval_end).for_each(|x| platform[rowcol][x].inside = LocType::Empty);
        },
        TiltType::East => {
            (interval_start..interval_end-num_to_fill).for_each(|x| platform[rowcol][x].inside = LocType::Empty);
            (interval_end-num_to_fill..interval_end).for_each(|x| platform[rowcol][x].inside = LocType::Rounded);
        },
    };
}

fn tilt_platform(platform: &mut [Vec<Loc>], tilt_dir: &TiltType) {
    match tilt_dir {
        TiltType::North | TiltType::South => {
            (0..platform[0].len()).for_each(|x| {
                let mut blocks: Vec<usize> = (0..platform.len()).filter(|y| platform[*y][x].inside == LocType::Cube).collect();

                if blocks.is_empty() {
                    let num_to_fill = (0..platform.len()).filter(|y| platform[*y][x].inside == LocType::Rounded).count();
                    fill_interval(platform, tilt_dir, 0, platform.len(), num_to_fill, x);
                } else {
                    let interval_start = 0;
                    let interval_end = blocks[0];
                    let num_to_fill = (interval_start..interval_end).filter(|y| platform[*y][x].inside == LocType::Rounded).count();
                    fill_interval(platform, tilt_dir, interval_start, interval_end, num_to_fill, x);

                    for i in 0..blocks.len()-1 {
                        let interval_start= blocks[i] + 1;
                        let interval_end = blocks[i+1];
                        let num_to_fill = (interval_start..interval_end).filter(|y| platform[*y][x].inside == LocType::Rounded).count();
                        fill_interval(platform, tilt_dir, interval_start, interval_end, num_to_fill, x);
                    }

                    let interval_start= blocks[blocks.len()-1]+1;
                    let interval_end = platform.len();
                    let num_to_fill = (interval_start..interval_end).filter(|y| platform[*y][x].inside == LocType::Rounded).count();
                    fill_interval(platform, tilt_dir, interval_start, interval_end, num_to_fill, x);
                }
            });
        },
        TiltType::West | TiltType::East => {
            (0..platform.len()).for_each(|y| {
                let mut blocks: Vec<usize> = (0..platform[0].len()).filter(|x| platform[y][*x].inside == LocType::Cube).collect();

                if blocks.is_empty() {
                    let num_to_fill = (0..platform[0].len()).filter(|x| platform[y][*x].inside == LocType::Rounded).count();
                    fill_interval(platform, tilt_dir, 0, platform[0].len(), num_to_fill, y);
                } else {
                    let interval_start = 0;
                    let interval_end = blocks[0];
                    let num_to_fill = (interval_start..interval_end).filter(|x| platform[y][*x].inside == LocType::Rounded).count();
                    fill_interval(platform, tilt_dir, interval_start, interval_end, num_to_fill, y);

                    for i in 0..blocks.len()-1 {
                        let interval_start= blocks[i] + 1;
                        let interval_end = blocks[i+1];
                        let num_to_fill = (interval_start..interval_end).filter(|x| platform[y][*x].inside == LocType::Rounded).count();
                        fill_interval(platform, tilt_dir, interval_start, interval_end, num_to_fill, y);
                    }

                    let interval_start= blocks[blocks.len()-1]+1;
                    let interval_end = platform.len();
                    let num_to_fill = (interval_start..interval_end).filter(|x| platform[y][*x].inside == LocType::Rounded).count();
                    fill_interval(platform, tilt_dir, interval_start, interval_end, num_to_fill, y);
                }
            });
        },
    }
}

fn parse_platform(input: &[String]) -> Vec<Vec<Loc>> {
    input.iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    Loc { inside: match c {
                            'O' => LocType::Rounded,
                            '#' => LocType::Cube,
                            '.' => LocType::Empty,
                            _ => panic!("Unrecognizable input char"),
                        }, row: y, col: x }
                })
                .collect()
        })
        .collect()
}

fn solve_part_2(input: Vec<String>) -> i32 {
    let mut platform: Vec<Vec<Loc>> = parse_platform(&input);  
    let cycle = vec![TiltType::North, TiltType::West, TiltType::South, TiltType::East];
    let total_iter = 1000000000; 
    let mut support = vec![];
    let mut answers: HashMap<usize, i32> = HashMap::new();
    for _ in 0..total_iter {
        cycle.iter()
            .for_each(|cycle_type| { 
                tilt_platform(&mut platform, cycle_type);
            });
        support.push(compute_total_beam_load(&platform, TiltType::North));

        // detect cycle
        if support.len() > 6 {
            if let Some(x_) = support[..support.len()-1].iter().rposition(|x| *x == *support.iter().last().unwrap()) {
                let x = x_ + 1;
                let pattern_len = support.len() - x;
                if pattern_len > 1 && support.len()> 2*pattern_len && support[x-pattern_len..x] == support[x..] {
                    let answer = support[x + (total_iter-x-1) % pattern_len];
                    if let Some(x) = answers.insert(pattern_len, answer) {
                        if x == answer && pattern_len >= *answers.keys().max().unwrap()  {
                            support.push(answer);
                            break;
                        }
                    };
                }
            }
        }
    }
    *support.last().unwrap() 
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input.clone());
        let p2 = solve_part_2(input);
        assert_eq!(p1, 136);
        assert_eq!(p2, 64);
    }
}
