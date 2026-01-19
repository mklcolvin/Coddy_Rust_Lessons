use std::io;

// TODO: Define the TextStyle struct here

struct TextStyle {
    bold: bool,
    italic: bool,
    size: u32,
}

// TODO: Add the implementation block for TextStyle with:
// - new() associated function
// - set_bold() method
// - set_italic() method
// - set_size() method

impl TextStyle { 

    fn new() -> TextStyle {
        TextStyle { bold: false, italic: false, size: 12 }
    }

    fn set_bold(&mut self, bold_value: bool) -> &mut Self {
        self.bold = bold_value;
        self
    }

    fn set_italic(&mut self, italic_value: bool) -> &mut Self {
        self.italic = italic_value;
        self
    }
    
    fn set_size(&mut self, new_size: u32) -> &mut Self {
        self.size = new_size;
        self
    }


}

fn main() {
    let mut input = String::new();
    
    // Read bold setting
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let bold: bool = input.trim().parse().expect("Invalid bool");
    input.clear();
    
    // Read italic setting
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let italic: bool = input.trim().parse().expect("Invalid bool");
    input.clear();
    
    // Read size setting
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let size: u32 = input.trim().parse().expect("Invalid u32");

    // TODO: Create a mutable TextStyle using new(), then use method chaining
    // to apply all three settings in a single expression

    let mut my_textstyle = TextStyle::new();

    my_textstyle.set_bold(bold).set_italic(italic).set_size(size);
     
    // TODO: Print the result in the format: Bold: {bold}, Italic: {italic}, Size: {size}
       
     println!("Bold: {}, Italic: {}, Size: {}", my_textstyle.bold, my_textstyle.italic, my_textstyle.size);

}