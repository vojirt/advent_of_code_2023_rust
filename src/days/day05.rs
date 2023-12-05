use std::fs;

pub fn solve() {
    let input = fs::read_to_string("./inputs/input_5.txt")
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
struct Range {
    start_dst: i64,
    start_src: i64,
    length: i64,
}

#[derive(Debug, Clone)]
struct XRange {
    start: i64,
    length: i64,
}

impl XRange {
    fn end(&self) -> i64 {
        self.start + self.length
    }
}

impl Range {
    fn end(&self) -> i64 {
        self.start_src + self.length
    }

    fn transform(&self, x:i64) -> Option<i64> {
        let offset: i64 = x - self.start_src;
        if (offset >= 0) & (offset < self.length) {
            Some(self.start_dst+offset)
        } else { 
            None
        }
    }

    fn transform_range(&self, x: &XRange) -> (Option<XRange>, Vec<XRange>) {
        let mut res: Vec<XRange> = vec![];
        let mut valid = None;
        
        if x.end() <= self.start_src {
            res.push(x.clone());
        }

        if x.start >= self.end() {
            res.push(x.clone());
        }

        if (x.start < self.start_src)
            & ( x.end() > self.start_src) {
            res.push(XRange { start: x.start, length: self.start_src - x.start });
            valid = Some(XRange { start: self.start_dst, length: (x.end() - self.start_src).min(self.length) });
            if x.end() > self.end() {
                res.push(XRange { start: self.end(), length: x.end() - self.end() });
            }
        }

        if (x.start >= self.start_src) 
            & (x.start < self.end()) {
            let offset = x.start - self.start_src; 
            valid = Some(XRange { start: self.start_dst + offset, length: (x.length).min(self.end() - x.start) });
            if x.end() > self.end() {
                res.push(XRange { start: self.end(), length: x.end() - self.end() });
            }
        }

        (valid, res)
    }

    fn transform_ranges(&self, x: &[XRange]) -> (Vec<XRange>, Vec<XRange>) {
        let mut mapped_vec: Vec<XRange> = vec![];
        let mut rest_vec: Vec<XRange> = vec![];
        x.iter()
         .for_each(|xr| {
             let (o1, mut rest) = self.transform_range(xr);
             if let Some(x) = o1 {
                 mapped_vec.push(x);
             }
            rest_vec.append(&mut rest);
         });
        (mapped_vec, rest_vec)
    }

}

#[derive(Debug)]
struct Almanac {
    seeds_ids: Vec<i64>,
    mappings: Vec<(String, String, Vec<Range>)>,
}

fn parse(input: Vec<String>) -> Almanac {
    let seed_ids = input[0].split(':').last().unwrap().split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
    let input_map = &input[2..];
    let mut split_idx: Vec<usize> = input_map.iter()
        .enumerate()
        .filter(|(_, s)| s.is_empty())
        .map(|(i, _)| i+1)
        .collect();
    split_idx.insert(0, 0);
    split_idx.push(input_map.len()+1);
    
    let mut map: Vec<(String, String, Vec<Range>)> = Vec::new();
    for i in 0..split_idx.len()-1 {
        parse_block(&input_map[split_idx[i]..split_idx[i+1]-1], &mut map);
    }
    Almanac { seeds_ids: seed_ids, mappings: map}
}

fn parse_block(i: &[String], map: &mut Vec<(String, String, Vec<Range>)>) {
   let map_str: Vec<&str> = i[0].split_whitespace().next().unwrap().split('-').collect();
   let ranges: Vec<Range> = i[1..].iter()
       .map(|s| {
           let ints: Vec<i64> = s.split_whitespace()
               .map(|ss| ss.parse::<i64>().unwrap())
               .collect();
           Range { start_dst: ints[0], start_src: ints[1], length: ints[2] }
       })
       .collect();

   map.push((map_str[0].to_owned(), map_str[map_str.len()-1].to_owned(), ranges));
}

fn solve_part_1(input: Vec<String>) -> i64 {
    let almanac = parse(input); 
    almanac.seeds_ids.iter()
        .map(|sid| {
            let m = almanac.mappings.iter()
                .fold(*sid, |acc, x| {
                    let rs = &x.2;
                    let mut res = acc;
                    for r in rs {
                        if let Some(x) = r.transform(acc) {
                            res = x
                        }
                    }
                    res
                });
            m
        })
        .min()
        .unwrap()
}

fn solve_part_2(input: Vec<String>) -> i64 {
    let almanac = parse(input); 
    // First brute force solution, tooked ~ 80 min 
    // almanac.seeds_ids.iter().enumerate().filter(|(i, v)| i % 2 == 0)
    //     .map(|(i, start)| {
    //         println!("{}..{}", *start, *start+almanac.seeds_ids[i+1]);
    //         ((*start)..*start+almanac.seeds_ids[i+1]).map(|sid| {
    //                 let m = almanac.mappings.iter()
    //                     .fold(sid, |acc, x| {
    //                         let rs = &x.2;
    //                         let mut res = acc;
    //                         for r in rs {
    //                             if let Some(x) = r.transform(acc) {
    //                                 res = x
    //                             }
    //                         }
    //                         res
    //                     });
    //                 m
    //             })
    //             .min()
    //             .unwrap()
    //     })
    //     .min()
    //     .unwrap()
    
    almanac.seeds_ids.iter().enumerate().filter(|(i, _)| i % 2 == 0)
        .map(|(i, start)| {
            let mut mapped_ranges: Vec<XRange> = vec![XRange {start: (*start), length: almanac.seeds_ids[i+1]}];
            almanac.mappings.iter()
                .for_each(|vr| {
                    let rs = &vr.2;
                    let mut mv: Vec<XRange> = vec![];
                    rs.iter()
                      .for_each(|r| {
                        if !mapped_ranges.is_empty() {
                            let (mut a, b) = r.transform_ranges(&mapped_ranges);
                            mv.append(&mut a);
                            mapped_ranges = b;
                        }
                    });
                    mapped_ranges.append(&mut mv);
                });
            mapped_ranges.iter()
                .map(|xr| xr.start)
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_case() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4".split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

        let p1 = solve_part_1(input.clone());
        let p2 = solve_part_2(input);
        assert_eq!(p1, 35);
        assert_eq!(p2, 46);
    }
}
