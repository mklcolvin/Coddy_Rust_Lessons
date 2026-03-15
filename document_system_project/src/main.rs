mod draw;
mod text_field;
mod button;

use crate::text_field::TextField;
use crate::draw::Draw;
use crate::button::Button;

pub struct Label {
    pub text: String,
}

impl Draw for Label {
    fn draw(&self) {
        println!("{}", self.text);
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let content = input.trim().to_string();

    let mut input2 = String::new();
    std::io::stdin().read_line(&mut input2).expect("Failed to read line");
    let label = input2.trim().to_string();

    let mut input3 = String::new();
    std::io::stdin().read_line(&mut input3).expect("Failed to read line");
    let width: u32 = input3.trim().parse().expect("Failed to parse width");


    // TODO: Create a TextField instance and call draw() on it

    let my_text_field = TextField { content: content };    
    my_text_field.draw();

        // TODO: Create a Button with `label` and `width`, then call draw()
   
   let my_button = Button { label: label, width: width };
   my_button.draw();

}