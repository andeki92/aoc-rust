use std::path::PathBuf;

use clap::ValueEnum;
use handlebars::Handlebars;
use serde::Serialize;

#[derive(ValueEnum, Clone, Debug)]
pub enum Template {
    Solution,
}

impl Template {
    fn path(&self) -> PathBuf {
        match self {
            Self::Solution => std::path::PathBuf::from(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../resources/templates/solution.hbs"
            )),
        }
    }

    pub fn render(&self, data: TemplateData) -> String {
        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(true);
        handlebars
            .register_template_file("template", self.path())
            .expect(&format!(
                "Failed to load {self:?} as a handlebar template. Does it exist?"
            ));

        let values = serde_json::to_value(&data)
            .expect(&format!("Failed to deserialize template-data {:?}", data));

        handlebars
            .render("template", &values)
            .expect(&format!("Failed to render {self:?} template"))
    }
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum TemplateData {
    Solution {
        year: u16,
        day: u8,
        day_formatted: String,
    },
}

impl std::fmt::Display for Template {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
