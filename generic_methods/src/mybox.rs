// TODO: Define a public generic struct called MyBox<T>
// with a private field 'contents' of type T

pub struct MyBox<T> {
    pub contents: T,
}


// TODO: Implement methods for MyBox<T>
// Remember: use impl<T> MyBox<T> { ... }

impl<T> MyBox<T> {

// TODO: Implement the following methods:
// - new(value: T) -> MyBox<T>: Creates a new MyBox with the given value
// - peek(&self) -> &T: Returns a reference to the contents
// - replace(&mut self, value: T): Replaces the current contents with a new value

    pub fn new(contents: T) -> MyBox<T> {
        MyBox{ contents }
    }

    pub fn peek(&self) -> &T {
        &self.contents
    }

    pub fn replace(&mut self, contents: T) {
        self.contents = contents
    }

}


