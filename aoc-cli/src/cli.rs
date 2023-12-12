use std::io::{stdin, stdout, Write};

use chrono::Datelike;
use clap::Parser;

use crate::template::Template;

const FIRST_AOC_YEAR: i64 = 2015;

pub enum Validation {
    APPROVED,
    REJECTED,
}

pub fn user_validation(prompt: &str) -> Validation {
    loop {
        print!("{prompt} [y/N] ");
        let mut s = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut s)
            .expect("Did not enter a correct string");

        match s.trim() {
            "y" | "Y" => return Validation::APPROVED,
            "n" | "N" | "" => return Validation::REJECTED,
            s => println!("'{}' is not a valid input, try 'y' or 'n'", s),
        }
    }
}

#[derive(Parser)]
pub enum AocCli {
    /// Create a solution boilerplate
    New {
        /// the year (or default this year). Must to be between 2015 (first AoC) and the current year
        #[arg(short, long, value_parser = clap::value_parser!(u16).range(FIRST_AOC_YEAR..=chrono::Utc::now().year() as i64), default_value_t = chrono::Utc::now().year() as u16)]
        year: u16,
        /// the day (or default this day). Has to be between 1 and 25 (for obvious reasons). Can be 0-padded (if you feel like it...)
        #[arg(short, long, value_parser = clap::value_parser!(u8).range(1..25), default_value_t = chrono::Utc::now().day() as u8 )]
        day: u8,
        // #[arg(short, long, default_value_t = Templates::Solution {})]
        /// the template to use
        #[arg(short, long, default_value = "solution")]
        template: Template,
    },
}
