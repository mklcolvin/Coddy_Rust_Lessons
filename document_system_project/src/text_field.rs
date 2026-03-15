use crate::draw::Draw;

// TODO: Define a TextField struct with a public `content` field (String)

pub struct TextField {
    pub content: String,
}


// TODO: Implement the Draw trait for TextField
//       draw() should print: Text: {content}

impl Draw for TextField {
    fn draw(&self) {
        println!("Text: {}", self.content)
    }
}