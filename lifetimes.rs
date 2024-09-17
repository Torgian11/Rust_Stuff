use std::fmt::Display;

struct ImportantExerpt<'a> {
    part: &'a str, //string slice, but is a lifetime annotation
}

// Unsafe
// fn main() {
//     let r;
//     {
//         let x = 5;
        // r = &x; <== this ends up being assigned a reference that no longe rexists outside the block
//     }

//     println!("r: {}", r);
// }
// error: could not compile `lifetimes` (bin "lifetimes") due to 1 previous error


// Fixes above code so that there aren't any dangling references
fn main() {
    let x = 5;
    let r = &x; // <== no longer a dangling refernce
    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}"); // This is all valid

    // Below is invalid:
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    //     In this case, resultis assigned "longest". Whiel valid in Javascript, in Rust, this is invalid since
    //     `result` goes out of scope at the end of this block.
    //      
    // }
    // println!("The longest string is {result}"); Error is thrown


    // Using struct with lifetime annotation
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExerpt {
        part: first_sentence, // Valid and will compile, since `novel` won't go out of scope until after `i` is used
    };

    let s: &'static str = "I have a static lifetime."; // stored directly in program's binary, and is always available.
}

// type parameters, trait bounds, lifetimesa ll in one function!
fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}"); // since ann is not returned, this code compiles
    if x.len() > y.len() {
        x // x and y have the lifetime reference of 'a, so code compiles
    } else {
        y
    }
}



// Lifetiem Elision
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

impl<'a> ImportantExerpt<'a> {
    fn level(&self) -> i32 { // i32 is not a reference to anything
        3
    }

    // Third lifetime elision rule applies here. Since &self and announcement get their own lifetimes, and one of the parameters
    // is &self, the return type gets the lifetime of &self. Therefore, all lifetimes are accounted for.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}
// fn longest(x: &str, y: &str) -> &str { // &str return will return expected named lifetime parameter error. So, the returntype expects a generic lifetime parameter on it
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// The 'a is a lifetime name, which is common, and is added to each reference.
// This means 'a is a generic lifetime parameter. All references in the signature have the same lifetime of 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}


// The following would fail to compile, since two lifetime references are created, but only 'a is returned.
// So, if this function returned 'b, it would fail, therefore, compiling will fail
// fn shortest<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {

//     if x.len() < y.len() {
  
//       x
  
//     } else {
  
//       y
  
//     }
  
//   }

// This "longest" function returns a DANGLING reference, so it will fail to compile since the returned type is not the declared lifetime ref
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str() < This returns a value that is refernecing data owned by this current function, 
// this is because "result" is cleaned up and goes out of scope. Otherwise, this would return a dangling reference
// }
