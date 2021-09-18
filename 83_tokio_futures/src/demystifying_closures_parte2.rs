use std::{convert::Infallible, error::Error, pin::Pin};

use futures::{future, Future, FutureExt};
use log::debug;
use simplelog::{ConfigBuilder, SimpleLogger};

fn config_log() {
    let config = ConfigBuilder::new()
        .set_target_level(log::LevelFilter::Trace)
        .build();

    let _ = SimpleLogger::init(log::LevelFilter::Trace, config);
}

pub fn run_1() {
    config_log();
    let rt = tokio::runtime::Runtime::new().unwrap();

    let _guard = rt.enter();
    debug!("1 - in rt.enter()");
    tokio::spawn(future::lazy(|_| debug!("1 - in tokio::spawn()")));
    rt.spawn(future::lazy(|_| debug!("1 - in tokio::spawn()")));
    rt.block_on(future::lazy(|_| debug!("1 - in tokio::spawn()")));
    println!("");
}

pub fn run_2() {
    config_log();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .on_thread_start(|| debug!("2 - on_thread_start()"))
        .worker_threads(4)
        .build()
        .unwrap();

    let _guard = rt.enter();
    debug!("2 - in rt.enter()");
    tokio::spawn(future::lazy(|_| debug!("2 - in tokio::spawn()")));
    rt.spawn(future::lazy(|_| debug!("2 - in tokio::spawn()")));
    rt.block_on(future::lazy(|_| debug!("2 - in tokio::spawn()")));
    println!("");
}

pub fn returns_future_i32() -> impl Future<Output = i32> {
    future::ready(10)
}

pub fn returns_dyn_future_i32() -> Pin<Box<dyn Future<Output = i32> + Send>> {
    future::ready(32).boxed()
}

pub fn returns_future_result() -> impl Future<Output = Result<i32, impl Error>> {
    future::ok::<_, Infallible>(32)
}

pub fn run_3() {
    config_log();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .on_thread_start(|| debug!("2 - on_thread_start()"))
        .worker_threads(4)
        .build()
        .unwrap();

    rt.spawn(returns_future_i32());
    rt.spawn(returns_dyn_future_i32());
    rt.spawn(returns_future_result());
}
