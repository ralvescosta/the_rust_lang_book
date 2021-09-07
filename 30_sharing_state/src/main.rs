use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

fn main() {
    first_example();
    second_example();
}

fn first_example() {
    println!("FIRST EXAMPLE!");

    let m = Mutex::new(5);

    {
        let mut num: MutexGuard<i32> = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
    println!("");
}

fn second_example() {
    println!("SECOND EXAMPLE!");

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
