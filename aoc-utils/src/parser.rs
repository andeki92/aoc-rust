use std::{fs::File, io::Read, str::FromStr};

pub trait ParseExt {
    fn iter_unsigned<'a, T>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Result<T, <T as FromStr>::Err>> + 'a>
    where
        T: FromStr;
}

impl ParseExt for &str {
    fn iter_unsigned<'a, T>(
        &'a self,
    ) -> Box<dyn Iterator<Item = Result<T, <T as FromStr>::Err>> + 'a>
    where
        T: FromStr,
    {
        Box::new(self.split(&[' ', ',']).map(|c| c.parse::<T>()))
    }
}

pub fn read(file_name: &str) -> String {
    let mut f = File::open(file_name).expect(&format!(
        "File not found: {}. Files are read relative to the Cargo.toml directory",
        file_name
    ));

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect(&format!("cannot read file {}", file_name));

    contents.to_string()
}

pub fn read_to_vec(file_name: &str) -> Vec<String> {
    let mut f = File::open(file_name).expect(&format!("file not found: {}", file_name));

    let mut contents = String::new();

    f.read_to_string(&mut contents)
        .expect(&format!("cannot read file {}", file_name));

    contents
        .trim_end()
        .split("\n")
        .map(|s| s.to_owned())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_test() {
        let relative_file = format!("../{}", file!());
        assert!(read(&relative_file).contains(&"pub fn read(file_name: &str) -> String".to_owned()))
    }

    #[test]
    fn read_to_vec_test() {
        let relative_file = format!("../{}", file!());
        assert!(read_to_vec(&relative_file)
            .iter()
            .any(|l| l.contains(&"pub fn read(file_name: &str) -> String".to_owned())))
    }
}
