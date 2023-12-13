use std::path::PathBuf;

pub struct Config {
    pub solution_dir: PathBuf,
    pub input_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            solution_dir: PathBuf::from("aoc-solutions/src"),
            input_dir: PathBuf::from("resources/input"),
        }
    }
}

pub struct PathConfig {
    pub solution_file: PathBuf,
    pub input_file: PathBuf,
}

impl PathConfig {
    pub fn from(year: u16, day: u8) -> Self {
        let config = Config::default(); // todo: allow configuration of this

        PathConfig {
            solution_file: config
                .solution_dir
                .join(format!("year{year}/day{day:02}.rs")),
            input_file: config.input_dir.join(format!("year{year}/day{day:02}.txt")),
        }
    }
}
