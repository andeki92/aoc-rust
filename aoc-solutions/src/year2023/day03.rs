//! # Day 3:

use std::collections::{HashMap, HashSet};

use utils::point::Point;

type SolutionType = usize;

#[derive(Debug)]
pub struct Schematic {
    symbols: Vec<(char, Point)>,
    numbers_lookup: HashMap<Point, usize>,
}

pub fn input(raw: &str) -> Schematic {
    let mut symbols = vec![];
    let mut numbers_lookup: HashMap<Point, usize> = HashMap::new();

    for (y, line) in raw.lines().enumerate() {
        let mut line = line.trim().chars();
        let mut num = vec![];
        let mut x = 0;

        while let Some(next) = line.next() {
            if next.is_ascii_digit() {
                num.push(next)
            } else {
                if !num.is_empty() {
                    let parsed_num = num.iter().collect::<String>().parse::<usize>().unwrap();
                    for offset in 1..=num.len() {
                        numbers_lookup.insert(Point::new(x - offset as i32, y as i32), parsed_num);
                    }
                    num.clear()
                }

                if !next.is_ascii_digit() && next != '.' {
                    symbols.push((next, Point::new(x as i32, y as i32)))
                }
            }
            x += 1;
        }

        // at the end of the line we need to save the num as well
        if !num.is_empty() {
            let parsed_num = num.iter().collect::<String>().parse::<usize>().unwrap();
            for offset in 1..=num.len() {
                numbers_lookup.insert(Point::new(x - offset as i32, y as i32), parsed_num);
            }
        }
    }

    Schematic {
        symbols,
        numbers_lookup,
    }
}

pub fn part_one(input: &Schematic) -> SolutionType {
    let mut seen = HashSet::new();

    input.symbols.iter().for_each(|(_, symbol)| {
        symbol.neighbours().iter().for_each(|&gear_neighbour| {
            if let Some(&num) = input.numbers_lookup.get(&gear_neighbour) {
                // by storing the number along with the adjacent symbol we can ensure
                // we count numbers adjacent to n-symbols the correct amount of times
                seen.insert((symbol, num));
            }
        })
    });
    seen.iter().map(|(_, num)| num).sum()
}

pub fn part_two(input: &Schematic) -> SolutionType {
    input
        .symbols
        .iter()
        .filter(|(symbol, _)| symbol == &'*')
        .filter_map(|(_, gear)| {
            let mut seen = HashSet::new();

            gear.neighbours().iter().for_each(|gear_neighbour| {
                if let Some(&num) = input.numbers_lookup.get(&gear_neighbour) {
                    seen.insert(num);
                }
            });

            // if the gear has two neighbours, return their product
            (seen.len() == 2).then(|| seen.iter().product::<usize>())
        })
        .sum()
}

#[cfg(test)]
mod test {
    use utils::point::Point;

    use crate::year2023::day03::{input, part_one, part_two, Schematic};

    const EXAMPLE: &str = r"467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..";

    #[test]
    fn input_symbol_test() {
        let Schematic { symbols: gears, .. } = input(EXAMPLE);
        assert_eq!(
            vec![
                ('*', Point::new(3, 1)),
                ('#', Point::new(6, 3)),
                ('*', Point::new(3, 4)),
                ('+', Point::new(5, 5)),
                ('$', Point::new(3, 8)),
                ('*', Point::new(5, 8)),
            ],
            gears
        );
    }

    #[test]
    fn input_numbers_test() {
        let Schematic { numbers_lookup, .. } = input(EXAMPLE);
        assert_eq!(467, *numbers_lookup.get(&Point::new(0, 0)).unwrap());
        assert_eq!(467, *numbers_lookup.get(&Point::new(1, 0)).unwrap());
        assert_eq!(467, *numbers_lookup.get(&Point::new(2, 0)).unwrap());
        assert_eq!(114, *numbers_lookup.get(&Point::new(5, 0)).unwrap());
        assert_eq!(114, *numbers_lookup.get(&Point::new(6, 0)).unwrap());
        assert_eq!(114, *numbers_lookup.get(&Point::new(7, 0)).unwrap());
        assert_eq!(35, *numbers_lookup.get(&Point::new(2, 2)).unwrap());
        assert_eq!(35, *numbers_lookup.get(&Point::new(3, 2)).unwrap());
        assert_eq!(None, numbers_lookup.get(&Point::new(4, 2)));
    }

    #[test]
    fn part_one_test() {
        assert_eq!(4361, part_one(&input(EXAMPLE)));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(467835, part_two(&input(EXAMPLE)));
    }
}
