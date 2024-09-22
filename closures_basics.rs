use core::time::Duration;
use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked()) // <== this is a closure
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// This fails. See https://rust-book.cs.brown.edu/ch13-01-closures.html#closures-must-name-captured-lifetimes
// fn make_a_cloner(s_ref: &str) -> impl Fn() -> String {
//     move || s_ref.to_string()
// }

// Here, `s_ref` is a string reference that lives for 'a. The + 'a to the return type's trait bounds indicates the closure must live no longer than 'a.
fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
    move || s_ref.to_string()
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };
    
    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveawa2 = store.giveaway(user_pref2);
    println!("The user with preference {:?} gets {:?}", user_pref2, giveawa2);

    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    

    fn  add_one_v1   (x: u32) -> u32 { x + 1 }  //< == function
    let add_one_v2 = |x: u32| -> u32 { x + 1 }; // <== closure syntax. Fully annotated closure
    let add_one_v3 = |x: u32|             { x + 1 }; // <== no annotations closure
    let add_one_v4 = |x: u32|               x + 1  ;  // <== Brackets removed closure; optional since the closure body has only one expression.
    
    let list = vec![1,2,3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    let mut new_list = vec![1,2,3];
    let mut borrows_mutably = || new_list.push(7);
    borrows_mutably();
    println!("After calling mutable closure: {new_list:?}");

    // using a thread to print the vector in a new thread rahter than the main thread
    let list = vec![1,2,3];
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // println!("Before defining closure: {list:?}"); <== if used, this will throw an error that describes list being moved to a different thread


    // sort_by_key method using FnMut instead of FnOnce
    let mut listing = [
        Rectangle {width: 10, height: 1},
        Rectangle {width: 3, height: 5},
        Rectangle {width: 7, height: 12},
    ];

    // listing.sort_by_key(|r| r.width);
    // println!("{listing:#?}");

    // closure implementing the FnOnce trait
    // let mut sort_operations = vec![];
    let value = String::from("closure called");

    // The following sort breaks because the closure captures `value` and moves value out of the closure by transferring ownership
    // of `value` to the `sort_operations` vecotr. This works exactly once. Then, it crashes.
    // listing.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });

    let mut num_sort_operations = 0;
    listing.sort_by_key(|r| { // this is a much better way to count the number of sort operations
        num_sort_operations += 1;
        r.width
    });
    println!("{listing:#?}, sorted in {num_sort_operations} operations");
}
