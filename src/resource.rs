use serde::Serialize;

pub trait Resource: Sized + Serialize {
    fn resource_name() -> &'static str;
    fn table_headers() -> Vec<&'static str>;
    fn table_row(&self) -> Vec<String>;

    /// Return key-value pairs for a detailed single-resource view.
    /// Default implementation builds pairs from `table_headers()` and `table_row()`.
    fn table_detail(&self) -> Vec<(&'static str, String)> {
        Self::table_headers()
            .into_iter()
            .zip(self.table_row())
            .collect()
    }
}
