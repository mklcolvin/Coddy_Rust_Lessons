use crate::draw::Draw;

// TODO: Define a Button struct with two public fields:
// - label: String
// - width: u32

pub struct Button {
    pub label: String,
    pub width: u32,
}


// TODO: Implement the Draw trait for Button.
// The draw method should print: Button: {label} (width: {width})

impl Draw for Button {
    fn draw(&self) {
        println! ("Button: {} (width: {})", self.label, self.width)
    }
}
