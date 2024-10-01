use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // mpsc = multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("Hello!");
        tx.send(val).unwrap();
        // println!("val is {val}"); // this will fail, since we already sent `val` down the channel via `tx.send`. This could allow val to be modified in the main
        // thread befor eit's printed here. However, Rust will fail to compile here. Ownership is important!
    });

    let received = rx.recv().unwrap(); // recv will block the main thread's execution and wait until a value is sent donw a channel.
    // try_recv will not block the main thread, so it can be checked once in aw hile to see if we received a Result (Ok value)
    println!("Got: {received}!");

    let (tx2, rx2) = mpsc::channel();
    
    // another transmitter
    let tx3 = tx2.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vales = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vales {
            tx3.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // Two threads are passing messages to the receiver. Two transmitters, single receiver!
    for received in rx2 { //treating the receiver as an iterator, as each value will print as its received
        println!("Got: {received}");
    }
}
