use crate::List::{Cons, Nil};
use std::ops::Deref;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Implementing our own smart pointer similar to Box<T>
struct MyBox<T>(T); // T is the generic parameter

impl<T> Deref for MyBox<T> {
    type Target = T;

    // deref function is required when implementing the Deref trait
    fn deref(&self) -> &Self::Target {
        &self.0 // .0 accesses first value of a tuple struct.
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

struct CustomSmartPointer {
    data: String
}

// Drop is implemented for freeing resources automagically
impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}


fn main() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    // DEREF and DROP are important traits for a smart pointer


    let x = 5;
    let y = &x; // equals a reference to x

    let y = Box::new(x); // Biggest difference is that this is an instance of Box<T>, pointing to a copied value of x

    let y = MyBox::new(x); // Won't compile by itself since Rust doesn't know how to dereference this custom smart pointer
    // With the Deref trait, `y` will compile for MyBox

    assert_eq!(5, x);
    assert_eq!(5, *y); // deref operator to follow reference to the value it's pointing to (dereferencing)
    // *y basically runs *(y.deref())


    let m = MyBox::new(String::from("Rust"));
    hello(&m); // <== works due to deref coercion

    // If Rust didn't have deref coercion, we'd need to write the following:

    let n = MyBox::new(String::from("Rust"));
    hello(&(*n)[..]); // & and [..] take a string slice of the String that is equal to the whole string to match the sig of hello


    // Drop trait stuff
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    let d = CustomSmartPointer {
        data: String::from("Other stuff")
    };
    println!("Customer Smart Pointers created.");
}

fn hello(name: &str) {
    println!("Hello, {name}");
}
