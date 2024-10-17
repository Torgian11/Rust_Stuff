// A public structure,with private fields Structures are "objects" in Rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

// Example of encapsulation, which is necessary for object oriented programming.
// The option to use `pub` or not for different parts of the code enables this.
impl AveragedCollection {
    // Public functions are the only methods to update the list and retrieve the average
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    // Meanwhile, this is the only function available within this implementation
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


// Traits are one method for what other programs use for inheritance.
// Rust doesn't have inheritance though, so we can use Traits for common behavior

// Traits are not like traditional objects since we can't add data to a trait.
// However, their purpose is to allow abstraction across common behavior
pub trait Draw {
    fn draw(&self);
}

// Structure that holds a vector
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// Now we have an implementation of Screen with a run function that will draw all the components
// Trait objects allow us to use multiple concrete types at runtime
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// A generic version of this can be defined, using a generic type and a trait bound
pub struct ScreenTwo<T: Draw> {
    pub components: Vec<T>,
}

// This version will restrict us to a screen instance where all the components in the list
// are of a single type, such as Button or TextField. You cannot use mixed types in this case.
// This is preferable if we know a structure will only ever have a single type.
impl<T> ScreenTwo<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}


// Now, let's implement the Draw trait in some structures
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code for drawing the button
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to draw a select box
    }
}
