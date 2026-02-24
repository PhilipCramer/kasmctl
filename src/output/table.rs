use anyhow::Result;
use comfy_table::{Table, presets::UTF8_FULL_CONDENSED};

use crate::resource::Resource;

pub fn render_list<R: Resource>(items: &[R]) -> Result<String> {
    if items.is_empty() {
        return Ok(format!("No {}s found.", R::resource_name().to_lowercase()));
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_header(R::table_headers());
    for item in items {
        table.add_row(item.table_row());
    }

    let plural = if items.len() == 1 { "" } else { "s" };
    let footer = format!(
        "\n{} {}{}",
        items.len(),
        R::resource_name().to_lowercase(),
        plural
    );
    Ok(format!("{}{}", table, footer))
}

pub fn render_one<R: Resource>(item: &R) -> Result<String> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_header(vec!["FIELD", "VALUE"]);
    for (key, value) in item.table_detail() {
        table.add_row(vec![key, &value]);
    }
    Ok(table.to_string())
}
