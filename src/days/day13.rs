use std::fs;

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_13.txt")
        .expect("File not found")
        .lines()
        .map(|l: &str| l.to_string().replace('.', "0").replace('#', "1"))
        .collect::<Vec<String>>();

    let p1 = solve_part_1(input.clone());
    let p2 = solve_part_2(input);

    println!("Part 1 solution = {}", p1); 
    println!("Part 2 solution = {}", p2); 
}

fn get_maps(input: &[String]) -> Vec<&[String]> {
    let mut ret: Vec<&[String]> = vec![];
    let mut start_id = 0;
    input.iter()
        .enumerate()
        .filter(|(_, line)| line.is_empty())
        .for_each(|(i, _)| {
            ret.push(&input[start_id..i]);
            start_id = i+1;
        });
    ret.push(&input[start_id..]);
    ret
}

fn solve_part_1(input: Vec<String>) -> i32 {
    let maps = get_maps(&input);
    maps.iter()
        .fold(0, |acc, map| {
            let vert_refl = get_reflection_vert(map, false).unwrap_or(0); 
            let horiz_refl = get_reflection_horiz(map, false).unwrap_or(0); 
            if vert_refl > horiz_refl {
                acc + vert_refl
            } else {
                acc + 100*horiz_refl
            }
        })
}

fn is_symetric(map: &Vec<u32>, line_id: f32, smudge: bool) -> Option<i32> {
    let mut span = 0.0;
    let mut smudge_num = 0;
    loop {
        let left = (line_id - span).floor() as i32;
        let right = (line_id + span).ceil() as usize;
        if smudge {
            if left < 0 || right > map.len()-1 {
                break;
            }
           
            let diff_str = format!("{:b}", map[left as usize] ^ map[right]);
            let diff = diff_str.chars().filter(|c| *c == '1').count();
            if diff > 1 || (diff == 1 && smudge_num > 0) {
                break;
            }
            
            if diff > 0 {
                smudge_num += 1;
            }
        } else {
            if left < 0 || right > map.len()-1 || map[left as usize] != map[right] {
                break;
            }
        }
        span += 1.0; 
    }
    
    if span < 0.5 || (smudge_num == 0 && smudge) {
        None
    } else {
        Some(span as i32)
    }
}

fn convert_map(map: &&[String]) -> Vec<u32> {
    map.iter()
        .map(|s| u32::from_str_radix(s, 2).expect("Not a binary number!"))
        .collect()
}

fn find_max_symetric(map: Vec<u32>, smudge: bool) -> Option<i32> {
    let mut max_index = 0;
    (0..map.len()-1).for_each(|i| {
        if let Some(x) = is_symetric(&map, i as f32 + 0.5, smudge) {
            if (i as i32 +1) > max_index && (((i + 1) as i32 -x) == 0 || ((i + 1) + x as usize) == map.len())  {
                max_index = (i + 1) as i32;
            }
        }
    });
    if max_index > 0 {
        Some(max_index)
    } else {
        None
    }
}

fn get_reflection_vert(map: &&[String], smudge: bool) -> Option<i32> {
    // Transpose
    let map_norm = &(0..map[0].len()).map(|col| {
            (0..map.len()).fold("".to_string(), |acc, row| {acc + &map[row].chars().nth(col).unwrap().to_string()})
          }).collect::<Vec<String>>();
    let map_converted = convert_map(&&map_norm[..]);
    find_max_symetric(map_converted, smudge)
}

fn get_reflection_horiz(map: &&[String], smudge: bool) -> Option<i32> {
    let map_converted = convert_map(map);
    find_max_symetric(map_converted, smudge)
}

fn solve_part_2(input: Vec<String>) -> i32 {
    let maps = get_maps(&input);
    maps.iter()
        .fold(0, |acc, map| {
            let vert_refl = get_reflection_vert(map, true).unwrap_or(0); 
            let horiz_refl = get_reflection_horiz(map, true).unwrap_or(0); 
            if vert_refl > horiz_refl {
                acc + vert_refl
            } else {
                acc + 100*horiz_refl
            }
        })
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#".replace('.', "0").replace('#', "1").split('\n')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let p1 = solve_part_1(input.clone());
        let p2 = solve_part_2(input);
        assert_eq!(p1, 405);
        assert_eq!(p2, 400);
    }
}
