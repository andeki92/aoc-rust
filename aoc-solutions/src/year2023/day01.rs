//! # Day 1: Trebuchet?!
//!
//! For part two the sections can overlap, eg. twone or eighthree

type SolutionType = usize;

const DIGITS: [&str; 9] = [
    r"one", r"two", r"three", r"four", r"five", r"six", r"seven", r"eight", r"nine",
];

pub fn input(raw: &str) -> Vec<&str> {
    raw.lines().collect()
}

pub fn part_one(input: &[&str]) -> SolutionType {
    input.iter().map(|&line| digit_calibration(line)).sum()
}

pub fn part_two(input: &[&str]) -> SolutionType {
    input.iter().map(|&line| letter_calibration(line)).sum()
}

fn letter_calibration(mut input: &str) -> usize {
    let first = 'first: loop {
        if let Some(digit) = input.chars().nth(0).unwrap().to_digit(10) {
            break digit as usize;
        }

        for (value, &digit) in DIGITS.iter().enumerate() {
            if input.starts_with(digit) {
                break 'first value + 1;
            }
        }

        // remove the first char in the input
        input = &input[1..];
    };

    let last = 'last: loop {
        if let Some(digit) = input.chars().last().unwrap().to_digit(10) {
            break digit as usize;
        }

        for (value, &digit) in DIGITS.iter().enumerate() {
            if input.ends_with(digit) {
                break 'last value + 1;
            }
        }

        // remove the last char in the input
        input = &input[..input.len() - 1];
    };

    10 * first + last
}

fn digit_calibration(input: &str) -> usize {
    let first = input
        .chars()
        .find(char::is_ascii_digit)
        .and_then(|c| c.to_digit(10))
        .unwrap();

    let last = input
        .chars()
        .rfind(char::is_ascii_digit)
        .and_then(|c| c.to_digit(10))
        .unwrap();

    (10 * first + last) as usize
}

#[cfg(test)]
mod test {
    use crate::year2023::day01::{
        digit_calibration, input, letter_calibration, part_one, part_two,
    };

    const EXAMPLE: &str = r"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE_TWO: &str = r"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn input_test() {
        assert_eq!(
            vec!["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"],
            input(EXAMPLE)
        );

        assert_eq!(
            vec![
                "two1nine",
                "eightwothree",
                "abcone2threexyz",
                "xtwone3four",
                "4nineeightseven2",
                "zoneight234",
                "7pqrstsixteen"
            ],
            input(EXAMPLE_TWO)
        );
    }

    #[test]
    fn part_one_test() {
        assert_eq!(142, part_one(&input(EXAMPLE)));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(281, part_two(&input(EXAMPLE_TWO)));
    }

    #[test]
    fn digit_calibration_test() {
        assert_eq!(12, digit_calibration(r"1abc2"));
        assert_eq!(38, digit_calibration(r"pqr3stu8vwx"));
        assert_eq!(15, digit_calibration(r"a1b2c3d4e5f"));
        assert_eq!(77, digit_calibration(r"treb7uchet"));
    }

    #[test]
    fn letter_calibration_test() {
        assert_eq!(29, letter_calibration(r"two1nine"));
        assert_eq!(83, letter_calibration(r"eightwothree"));
        assert_eq!(13, letter_calibration(r"abcone2threexyz"));
        assert_eq!(24, letter_calibration(r"xtwone3four"));
        assert_eq!(42, letter_calibration(r"4nineeightseven2"));
        assert_eq!(14, letter_calibration(r"zoneight234"));
        assert_eq!(76, letter_calibration(r"7pqrstsixteen"));
    }
}
