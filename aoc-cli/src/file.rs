use std::{
    error::Error,
    fs::{create_dir_all, File},
    path::PathBuf,
};

use crate::cli::{user_validation, Validation};

pub trait PathBufExt {
    fn create(&self) -> Result<File, Box<dyn Error>>;
}

impl PathBufExt for PathBuf {
    fn create(&self) -> Result<File, Box<dyn Error>> {
        if self.exists() {
            panic!("File {self:?} already exists. The CLI is not designed to overwrite existing files!")
        }

        let parent_dir = &self.parent().expect("Failed to extract parent dir");

        if !parent_dir.exists() {
            if let Validation::APPROVED = user_validation(&format!(
                "Directory {:?} does not exist. Create it?",
                parent_dir
            )) {
                // todo; promt
                create_dir_all(parent_dir)
                    .expect(&format!("Failed to create directories {:?}", parent_dir));
            } else {
                panic!("User did not approve directory creation. Cannot continue.")
            }
        }

        let file = File::create(&self).expect(&format!("Failed to create file {:?}", &self));
        Ok(file)
    }
}
