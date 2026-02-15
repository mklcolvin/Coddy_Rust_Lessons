use crate::processor::TextProcessor;

// TODO: Create a public unit struct SimpleProcessor
// Implement TextProcessor for SimpleProcessor:
// - repeat: joins text with spaces (e.g., "Hi Hi Hi")
// - truncate: simply cuts at max_len

pub struct SimpleProcessor;

impl TextProcessor for SimpleProcessor {


    // TODO: Implement repeat method
    fn repeat(&self, text: &str, times: u32) -> String {
        let mut result = String::new();
        for i in 1..=times {
            if i > 1 {
                result.push(' ');
            }
            result.push_str(text);
        }
        result
    }

    // TODO: Implement truncate method
    fn truncate(&self, text: &str, max_len: usize) -> String {
        if text.len() > max_len {
            text[..max_len].to_string()
        } else {
            text.to_string()
        }
    }
}


// TODO: Create a public unit struct FancyProcessor
// Implement TextProcessor for FancyProcessor:
// - repeat: joins text with " * " (e.g., "Hi * Hi * Hi")
// - truncate: cuts at max_len and adds "..." if text was shortened

pub struct FancyProcessor;

impl TextProcessor for FancyProcessor {
    // TODO: Implement repeat method
    fn repeat(&self, text: &str, times: u32) -> String {
        let mut result = String::new();
        for i in 1..=times {
            if i > 1 {
                result.push_str(" * ");
            }
            result.push_str(text);
        }
        result
    }

    // TODO: Implement truncate method
    fn truncate(&self, text: &str, max_len: usize) -> String {
        if text.len() > max_len {
            let mut truncated = text[..max_len].to_string();
            truncated.push_str("...");
            truncated
        } else {
            text.to_string()
        }
    }
}
