use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| { // placing this spawned thread into a variable allows us to check if its finished later
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    // if we place a .join() call here, then the main thread will be blocked until the spawned thread finishes
    for i in 1..5 {
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // Call thread::spawn .join to make sure the spawned thread finishes first before the main function exits.
    handle.join().unwrap(); // <== this will block the main thread until the spawned thread is finished!

    // MOVE closures

    let v = vec!(1,2,3);

    // Without `move`, we will get an error here
    let handle_2 = thread::spawn(move || { // Adding move keyword here will force the closure tot ake ownership of the `v` values, instead of inferring a borrow
        println!("Here's a vecotr: {v:?}"); // <== will borrow `v`, so it captures it and make it part of the closure's env. 
    });

    // drop(v); // oh no! This will cause an error as well. However, using `move` in the above spawned thread will also cause an issue, since `move` transfers ownership
    // to the new thread instead of the `main` thread

    handle_2.join().unwrap();
}
