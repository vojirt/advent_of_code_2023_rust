use std::{env, io, time::Instant};

mod days;
use days::{day01, day02, day03};

fn main() {
    let day = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .expect("Expects at least one arguments corresponding to day!")
        .trim()
        .to_string()
        .parse::<u8>()
        .expect("Invalid day number!");

    let func: fn() = match day {
        1 => day01::solve,
        2 => day02::solve,
        3 => day03::solve,
        _ => panic!("Day should be in range (1,25) or Day {} is not implemented yet!", day),
    };

    println!("==================== DAY - {} ====================", day);

    let time = Instant::now();
    func();
    let mut elapsed_time = time.elapsed().as_nanos() as f64 / 1000.0;
    let mut time_unit = String::from("µs");
    if elapsed_time > 1000.0 {
        elapsed_time /= 1000.0;
        time_unit = String::from("ms");
    }
    if elapsed_time > 1000.0 {
        elapsed_time /= 1000.0;
        time_unit = String::from("sec");
    }
    println!("  · Elapsed: {:.2} {}", elapsed_time, time_unit);
}
