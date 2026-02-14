use serde::Serialize;

pub trait Resource: Sized + Serialize {
    fn resource_name() -> &'static str;
    fn table_headers() -> Vec<&'static str>;
    fn table_row(&self) -> Vec<String>;
}
