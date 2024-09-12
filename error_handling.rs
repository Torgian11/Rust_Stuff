use std::fs::{self, File};
// use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // let file_read_result = File::open("hello.txt")
    //     .expect("hello.txt should be included");

    // let file_read = match file_read_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("problem creating the file: {e:?}"),
    //         },
    //         other_error => {
    //             panic!("Other problem opening the file! {other_error:?}");
    //         }    
    //     },
    // };

    // println!("{file_read_result:?}");
    let username = read_username_from_file();
    let username_q_op = read_username_from_file_question_op();
    let username_chained = username_from_file_chained();

    println!("username: {username:?}");
    println!("username_?_op: {username_q_op:?}");
    println!("username_chained_?: {username_chained:?}");
    // let username_fs = username_from_file_fs();

    println!("Last char in text: {:?}", last_char_in_line("String"));
}



fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_question_op() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn username_from_file_chained() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// using filesystem directly to read file to string
// fn username_from_file_fs() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }


// Possibility of Optional character or not
fn last_char_in_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// Dynamic main function
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;

//     Ok(())
// }

// Implementation should be in a lib.rs file
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess {value}
    }

    // Getter
    pub fn value(&self) -> i32 {
        self.value
    }
}
