use std::{fs, collections::HashSet};

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_11.txt")
        .expect("File not found")
        .lines()
        .map(|l: &str| l.to_string())
        .collect::<Vec<String>>();

    let p1 = solve_part_2(input.clone(), 2);
    let p2 = solve_part_2(input, 1000000);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

fn solve_part_2(input: Vec<String>, factor: i64) -> i64 {
    let mut row_expantion_idx: HashSet<usize> = HashSet::<usize>::new();
    (0..input.len()).for_each(|l| {
        if input[l].chars().all(|c| c == '.') {
            row_expantion_idx.insert(l);
        }
    });
    let mut col_expantion_idx: HashSet<usize> = HashSet::<usize>::new();
    (0..input[0].len()).for_each(|cl| {
        if input.iter().all(|l| (l.chars().nth(cl).unwrap() == '.')) {
            col_expantion_idx.insert(cl);
        }
    });

    let mut galaxies: Vec<(i64, i64)> = vec![];
    (0..input.len()).for_each(|y| {
        (0..input[0].len()).for_each(|x| {
            if input[y].chars().nth(x).unwrap() == '#' {
                galaxies.push((y as i64, x as i64));
            }
        })
    });

    galaxies.iter()
        .enumerate()
        .map(|(i, g1)| {
            galaxies.iter()
                .enumerate()
                .filter(|(j, _)| *j > i)
                .map(|(_, g2)| {
                    let expanded_rows = row_expantion_idx.intersection(
                        &HashSet::<usize>::from_iter(g1.0.min(g2.0) as usize..g1.0.max(g2.0) as usize))
                        .count() as i64;
                    let normal_rows = (g1.0 - g2.0).abs() - expanded_rows;

                    let expanded_cols = col_expantion_idx.intersection(
                        &HashSet::<usize>::from_iter(g1.1.min(g2.1) as usize..g1.1.max(g2.1) as usize))
                        .count() as i64;
                    let normal_cols = (g1.1 - g2.1).abs() - expanded_cols;
                    normal_rows + expanded_rows*factor + normal_cols + expanded_cols*factor
                }).sum::<i64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....".split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_2(input.clone(), 2);
        assert_eq!(p1, 374);
        let p2 = solve_part_2(input.clone(), 10);
        assert_eq!(p2, 1030);
        let p2 = solve_part_2(input, 100);
        assert_eq!(p2, 8410);
    }
}
