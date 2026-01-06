// TODO: Define a public Locker struct with private fields:
// - code: u32
// - contents: String

pub struct Locker {
    code: u32,
    contents: String,
}


impl Locker {
    // TODO: Implement a public 'new' constructor
    // Takes initial code (u32) and contents (String)
    // Returns a new Locker instance
    pub fn new(initial_code: u32, contents: String) -> Self {
        Locker{ code: initial_code, contents: contents }
    }



    // TODO: Implement a public 'set_code' method
    // Takes a new code (u32) and updates the locker's code
    pub fn set_code (&mut self, code: u32) {

        self.code = code;
    }

    // TODO: Implement a public 'get_contents' method
    // Takes an attempted code (u32)
    // Returns String: the contents if code matches, or "Access denied" if not

    pub fn get_contents (&self, attempted_code: u32) -> String {

        if attempted_code == self.code {
            return self.contents.clone();
        } else {
            return "Access denied".to_string();
        }

    }
}
