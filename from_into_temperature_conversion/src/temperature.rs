// TODO: Define a public newtype struct Celsius wrapping an f64


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Celsius(pub f64);

// TODO: Define a public newtype struct Fahrenheit wrapping an f64

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fahrenheit(pub f64);

// TODO: Implement a value() method for Celsius that returns the inner f64


impl Celsius {
    pub fn value(&self) -> f64 {
        self.0  
    }
}

// TODO: Implement a value() method for Fahrenheit that returns the inner f64

impl Fahrenheit {
    pub fn value(&self) -> f64 {
        self.0
    }
}

// TODO: Implement From<Celsius> for Fahrenheit
// Use the formula: fahrenheit = celsius × 1.8 + 32.0
// Remember: implementing From automatically gives you Into for free!

impl From<Celsius> for Fahrenheit {
    fn from(celsius: Celsius) -> Self {
        Fahrenheit(celsius.value() * 1.8 + 32.0)
    }
}