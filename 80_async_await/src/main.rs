use std::{
    os::linux::raw::stat,
    sync::{Arc, Mutex},
};

use futures::executor::block_on;

async fn task1() -> i64 {
    println!("Task1");
    32
}

async fn task2() -> i64 {
    println!("Task2");
    12
}

async fn async_main() {
    let (result1, result2) = futures::join!(task1(), task2()); // run concurrently
    println!("Result 1 {} Result 2 {}", result1, result2)
}

struct State {
    count: u64,
}

//////////////////////////////////////////////////

async fn task3(state: &Arc<Mutex<State>>) -> i64 {
    if let Ok(mut state) = state.lock() {
        state.count += 1;
    }
    println!("Add 1");
    10
}

async fn task4(state: &Arc<Mutex<State>>) -> i64 {
    if let Ok(mut state) = state.lock() {
        state.count += 2;
    }
    println!("Add 2");
    25
}

async fn share_memory() {
    let state = State { count: 0 };
    let data = Arc::new(Mutex::new(state));

    let (r3, r4) = futures::join!(task3(&data), task4(&data));
    println!("Result 1 {} Result 2 {}", r3, r4);

    if let Ok(state) = data.lock() {
        println!("State {}", state.count)
    };
}

///////////////////////////////////////////////////

fn run() {
    block_on(async_main());

    println!("");
    block_on(share_memory());
}

fn main() {
    run();
}
