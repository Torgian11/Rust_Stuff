// RefCell<T>
// Used for mutable references

// At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
// References must always be valid.

// RefCell<T> invariants are enforced at runtime, unlike Box<T>, where the borrowing rules' invariants are enforced at compile time
// RefCell<T> type is useful when you are sure the program follows the borrowing rules, but the compiler can't understand that.

// Rc<T>: have multiple owners of the same data
// Box<T>: allow mutable or immutable borrows checked at compile time. 
// RefCell<T>: Allow mutable borrows checked at runtime; you can mutate the value inside RefCell<T> even when it _is_ immutable

// Mutating the value inside of an immutable value is the interior mutability pattern.

// Using Rc<T> with a RefCell<T> inside to gain multiple reference pointers to multiple lists to share ownership of another list, 
// and be able to mutate them
use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

// Memory leaks
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next imem = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}
