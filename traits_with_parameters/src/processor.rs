// TODO: Define a public TextProcessor trait with two methods:
// - repeat(&self, text: &str, times: u32) -> String
// - truncate(&self, text: &str, max_len: usize) -> String

pub trait TextProcessor {
    fn repeat(&self, text: &str, times: u32) -> String;
    fn truncate(&self, text: &str, max_len: usize) -> String;
}
