//! # Day 2: Cube Conundrum

type SolutionType = usize;

#[derive(Debug, PartialEq, Eq)]
pub struct Game(usize, usize, usize);

pub fn input(raw: &str) -> Vec<Game> {
    raw.lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .collect::<Vec<_>>()
                .chunks(2)
                .skip(1)
                .fold(Game(0, 0, 0), |Game(r, g, b), color| {
                    let amount = color[0].trim().parse::<usize>().unwrap();
                    let color = color[1];

                    match color.chars().nth(0) {
                        Some('r') => Game(r.max(amount), g, b),
                        Some('g') => Game(r, g.max(amount), b),
                        Some('b') => Game(r, g, b.max(amount)),
                        _ => unreachable!(),
                    }
                })
        })
        .collect()
}

pub fn part_one(input: &[Game]) -> SolutionType {
    input
        .iter()
        .enumerate()
        .filter_map(|(id, &Game(r, g, b))| (r <= 12 && g <= 13 && b <= 14).then_some(id + 1))
        .sum()
}

pub fn part_two(input: &[Game]) -> SolutionType {
    input.iter().map(|&Game(r, g, b)| r * g * b).sum()
}

#[cfg(test)]
mod test {
    use crate::year2023::day02::{input, part_one, part_two, Game};

    const EXAMPLE: &str = r"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn input_test() {
        assert_eq!(
            vec![
                Game(4, 2, 6),
                Game(1, 3, 4),
                Game(20, 13, 6),
                Game(14, 3, 15),
                Game(6, 3, 2),
            ],
            input(EXAMPLE)
        );
    }

    #[test]
    fn part_one_test() {
        assert_eq!(8, part_one(&input(EXAMPLE)));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(2286, part_two(&input(EXAMPLE)));
    }
}
