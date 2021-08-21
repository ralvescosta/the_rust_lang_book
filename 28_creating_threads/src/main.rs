use std::{thread, time::Duration};

fn create_a_thread() {
    let handler = thread::spawn(|| {
        for i in 1..10 {
            println!("2 his number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("1 his number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handler.join().unwrap();
}

fn move_keyword() {
    let v = vec![1, 2, 3];

    let handler = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handler.join().unwrap();
}

fn main() {
    create_a_thread();
    move_keyword();
}
