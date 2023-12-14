use aoc_solutions::{
    setup::{FilterExt, Solution},
    solution, year2023,
};
use std::{env::args, iter::empty, time::Instant};
use utils::ansi::*;

fn main() {
    // Parse command line options
    let year = match args().next() {
        Some(arg) => arg.as_str().parse::<u16>().ok(),
        None => None,
    };

    let day = match args().next_back() {
        Some(arg) => arg.as_str().parse::<u8>().ok(),
        None => None,
    };

    let solutions = empty()
        .chain(solutions())
        .filter(|solution| year == Some(solution.year) || year.is_none())
        .filter(|solution| day == Some(solution.day) || day.is_none())
        .collect::<Vec<_>>();

    for Solution {
        year,
        day,
        input,
        callback,
    } in solutions
    {
        let time = Instant::now();
        let (part_one, part_two) = (callback)(input);
        let elapsed = time.elapsed();

        let mut elapsed_str = format!("{} μs", elapsed.as_micros());

        if elapsed.as_micros() <= 3 {
            elapsed_str += &format!(" ({} ns)", elapsed.as_nanos());
        };

        println!("");
        println!("✨ {BOLD}{YELLOW}{year} Day {day:02}{RESET} ✨");
        println!("");
        println!("Solution to part 1 is: {YELLOW}{part_one}{RESET}");
        println!("Solution to part 2 is: {YELLOW}{part_two}{RESET}");
        println!("Took: {GREEN}{elapsed_str}{RESET}");
    }
}

fn solutions() -> Vec<Solution> {
    vec![solution!(year2023, day01), solution!(year2023, day02)]
}
