use std::{error::Error, io::Write};

use clap::Parser;
use config::PathConfig;
use file::PathBufExt;

use crate::{cli::AocCli, template::TemplateData};

mod cli;
mod config;
mod file;
mod template;

fn run_setup(paths: PathConfig, solution_template: Option<&str>) -> Result<(), Box<dyn Error>> {
    if let Some(content) = solution_template {
        if let Ok(mut solution_file) = paths.solution_file.create() {
            solution_file
                .write_all(content.as_bytes())
                .expect("Unable to write data to solution file");
        }
    }
    paths.example_file.create()?;
    paths.input_file.create()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = AocCli::parse();

    match args {
        AocCli::New {
            year,
            day,
            template,
        } => {
            let template_output = template.render(TemplateData::Solution { year, day });
            run_setup(PathConfig::from(year, day), Some(&template_output))?;
        }
    }
    Ok(())
}
