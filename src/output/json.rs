use anyhow::Result;
use serde::Serialize;

pub fn render_list<T: Serialize>(items: &[T]) -> Result<String> {
    Ok(serde_json::to_string_pretty(items)?)
}

pub fn render_one<T: Serialize>(item: &T) -> Result<String> {
    Ok(serde_json::to_string_pretty(item)?)
}
