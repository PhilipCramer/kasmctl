use anyhow::Result;
use comfy_table::{presets::UTF8_FULL_CONDENSED, Table};

use crate::resource::Resource;

pub fn render_list<R: Resource>(items: &[R]) -> Result<String> {
    let mut table = Table::new();
    table.load_preset(UTF8_FULL_CONDENSED);
    table.set_header(R::table_headers());
    for item in items {
        table.add_row(item.table_row());
    }
    Ok(table.to_string())
}

pub fn render_one<R: Resource>(item: &R) -> Result<String> {
    render_list(std::slice::from_ref(item))
}
