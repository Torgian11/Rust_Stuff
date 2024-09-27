// Rc<T> types, only used in single-threaded scenarios
// Rc<T> has immutable references

// enum List { // Fails at compile time
//     Cons(i32, Box<List>),
//     Nil,
// }

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // The following will fail at compile time since `a` is moved from b to c, and is used in c
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil)))); // Shared with B and C
    // let b = Cons(3, Box::new(a)); // 3, 5, 10
    // let c = Cons(4, Box::new(a)); // 4, 5, 10

    // Using Rc<T>

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a)); //Rc::clone is used here to INCREMENT the reference count
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a)); // We could use `a.clone`, but this makes a DEEP COPY, which takes more time than incrementing a reference
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c is out of scope: {}", Rc::strong_count(&a));    
}
