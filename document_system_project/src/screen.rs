use crate::draw::Draw;

// TODO: Define a Screen struct with a public field `components` of type Vec<Box<dyn Draw>>

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>
}

// TODO: Implement Screen:
//   - A `new` associated function that returns an empty Screen
//   - An `add` method that takes &mut self and a Box<dyn Draw>, and pushes it to components

impl Screen {
    pub fn new() -> Self {
        Screen { components: Vec::new() }
    }

    pub fn add(&mut self, component: Box<dyn Draw>) {
        self.components.push(component);
    }

    // TODO: Add a `run` method that takes &self and calls draw() on every component

    pub fn run(&self) {
        for component in &self.components {
            component.draw();
        }
    }
}
