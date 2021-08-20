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
    let (result1, result2) = futures::join!(task1(), task2());
    println!("Result 1 {} Result 2 {}", result1, result2)
}

fn main() {
    block_on(async_main());
}
