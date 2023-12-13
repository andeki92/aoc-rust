use std::{fmt::Debug, str::FromStr};

pub struct Solution {
    pub year: u16,
    pub day: u8,
    pub input: &'static str,
    pub callback: fn(&str) -> (String, String),
}

#[macro_export]
macro_rules! solution {
    ($year:tt, $day:tt) => {
        Solution {
            year: stringify!($year).safe_parse(),
            day: stringify!($day).safe_parse(),
            input: include_str!(concat![
                env!("CARGO_MANIFEST_DIR"),
                "/../resources/input/",
                stringify!($year),
                "/",
                stringify!($day),
                ".txt"
            ]),
            callback: |raw_input: &str| {
                use $year::$day::*;
                let input = input(raw_input);
                let part_one = part_one(&input).to_string();
                let part_two = part_two(&input).to_string();
                (part_one, part_two)
            },
        }
    };
}

pub trait FilterExt {
    fn safe_parse<T>(&self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: Debug;
}

impl FilterExt for &str {
    fn safe_parse<T>(&self) -> T
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        self.chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse::<T>()
            .unwrap()
    }
}
