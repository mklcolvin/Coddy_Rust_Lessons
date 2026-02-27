// TODO: Define a public trait called Printable
// It should have a method print_info that takes &self and returns a String

// TODO: Define a public generic function called announce
// It should accept any type T that implements Printable
// The function should print "Announcement: {info}" where {info} is the result of print_info()
pub trait Printable {
   fn print_info(&self) -> String;
}

pub fn announce<T: Printable>(item: T) {
    println!("Announcement: {}", item.print_info());
}