mod draw;
mod text_field;
mod button;
mod screen;

use crate::text_field::TextField;
use crate::draw::Draw;
use crate::button::Button;
use crate::screen::Screen;

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
//  my_text_field.draw();  From the instructions, it seems we should not call draw() on the TextField directly, but rather add it to the Screen and call draw() through the Screen's components.

    // TODO: Create a Button with `label` and `width`, then call draw()
   
   let my_button = Button { label: label, width: width };
//   my_button.draw();  Again, we will add this to the Screen and call draw() through the Screen's components.

    // TODO: Create a Screen using Screen::new()
    // TODO: Add a TextField wrapped in Box::new() to the screen
    // TODO: Add a Button wrapped in Box::new() to the screen
    // TODO: Iterate over screen.components and call draw() on each component

    let mut my_screen = Screen::new();
    my_screen.add(Box::new(my_text_field));
    my_screen.add(Box::new(my_button));

    // TODO: Replace the manual iteration below with a single call to screen.run()

    my_screen.run();
    
}