use std::path::PathBuf;

pub struct Config {
    pub solution_dir: PathBuf,
    pub examples_dir: PathBuf,
    pub input_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            solution_dir: PathBuf::from("../aoc-solutions/src"),
            examples_dir: PathBuf::from("../resources/examples"),
            input_dir: PathBuf::from("../resources/input"),
        }
    }
}

pub struct PathConfig {
    pub solution_file: PathBuf,
    pub example_file: PathBuf,
    pub input_file: PathBuf,
}

impl PathConfig {
    pub fn from(year: u16, day: u8) -> Self {
        let config = Config::default(); // todo: allow configuration of this

        PathConfig {
            solution_file: config.solution_dir.join(format!("{year}/day{day:02}.rs")),
            example_file: config.examples_dir.join(format!("{year}-{day:02}.txt")),
            input_file: config.input_dir.join(format!("{year}-{day:02}.txt")),
        }
    }
}
