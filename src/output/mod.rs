pub mod display;
pub mod json;
pub mod table;
pub mod yaml;

use anyhow::Result;
use clap::ValueEnum;

use crate::resource::Resource;

#[derive(Clone, Default, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Table,
    Json,
    Yaml,
}

pub fn render_list<R: Resource>(items: &[R], format: &OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Table => table::render_list(items),
        OutputFormat::Json => json::render_list(items),
        OutputFormat::Yaml => yaml::render_list(items),
    }
}

pub fn render_one<R: Resource>(item: &R, format: &OutputFormat) -> Result<String> {
    match format {
        OutputFormat::Table => table::render_one(item),
        OutputFormat::Json => json::render_one(item),
        OutputFormat::Yaml => yaml::render_one(item),
    }
}
