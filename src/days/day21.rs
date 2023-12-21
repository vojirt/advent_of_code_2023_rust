use std::fs;

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_21.txt")
        .expect("File not found")
        .lines()
        .map(|l: &str| l.to_string())
        .collect::<Vec<String>>();

    let p1 = solve_part_1(input.clone(), 64);
    let p2 = solve_part_2(input);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

#[derive(Debug, PartialEq, Eq)]
enum LocType {
    Plots,
    Rock,
}

#[derive(Debug, PartialEq, Eq)]
struct Loc {
    x: i32,
    y: i32,
}

fn parse_map(input: &[String]) -> (Vec<Vec<LocType>>, Loc){
    let mut start_loc = Loc { x: 0, y: 0 }; 
    (input.iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    match c {
                        '#' => LocType::Rock,
                        '.' => LocType::Plots,
                        'S' => {
                            start_loc = Loc {x: x as i32, y: y as i32};
                            LocType::Plots
                        },
                        _ => panic!("Invalid character"),
                    }
                })
                .collect()
        })
        .collect(), start_loc)
}

fn valid_loc(loc: &Loc, map: &[Vec<LocType>]) -> bool {
    if loc.x >= 0 && loc.x < map[0].len() as i32 &&
       loc.y >= 0 && loc.y < map.len() as i32 &&
       map[loc.y as usize][loc.x as usize] == LocType::Plots {
           true
    } else {
        false
    }
}

fn solve_part_1(input: Vec<String>, num_steps: usize) -> usize {
    let (map, start_loc) = parse_map(&input);
    // (0..map.len()).for_each(|y| {
    //     (0..map[0].len()).for_each(|x| {
    //         print!("{:}", match map[y][x] {
    //                 LocType::Plots => '.', 
    //                 LocType::Rock => '#',
    //         });
    //     });
    //     println!();
    // });

    let mut queue = vec![start_loc];

    for step in 0..num_steps {
        let mut loc_queue = vec![];
        for j in 0..queue.len() {
            let current_loc = queue.remove(0);

            let mut moves = vec![Loc {y: current_loc.y + 1, x: current_loc.x}, 
                            Loc {y: current_loc.y, x: current_loc.x - 1}, Loc {y: current_loc.y, x: current_loc.x + 1},
                            Loc {y: current_loc.y - 1, x: current_loc.x}];

            for i in 0..4 {
                let mov = moves.remove(0);
                if valid_loc(&mov, &map) && !loc_queue.contains(&mov) {
                    loc_queue.push(mov);
                }
            }
        }
        queue = loc_queue;

        // println!("====== Step {} ======", step);
        // (0..map.len()).for_each(|y| {
        //     (0..map[0].len()).for_each(|x| {
        //         let l = Loc {x:x as i32, y:y as i32};
        //         let to_print = if queue.contains(&l) { 'O' } else {
        //             match map[y][x] {
        //                     LocType::Plots => '.', 
        //                     LocType::Rock => '#',
        //             }
        //         };
        //         print!("{:}", to_print); 
        //         });
        //     println!();
        // });
    }

    queue.len()
}

fn solve_part_2(input: Vec<String>) -> i32 {
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input.clone(), 6);
        let p2 = solve_part_2(input);
        assert_eq!(p1, 16);
        assert_eq!(p2, 0);
    }
}
