use anyhow::Result;
use serde::Serialize;

pub fn render_list<T: Serialize>(items: &[T]) -> Result<String> {
    Ok(serde_yaml::to_string(items)?)
}

pub fn render_one<T: Serialize>(item: &T) -> Result<String> {
    Ok(serde_yaml::to_string(item)?)
}
