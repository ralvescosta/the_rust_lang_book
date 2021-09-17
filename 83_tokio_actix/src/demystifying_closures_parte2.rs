use futures::future;
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
    debug!("in rt.enter()");
    tokio::spawn(future::lazy(|_| debug!("in tokio::spawn()")));
    rt.spawn(future::lazy(|_| debug!("in tokio::spawn()")));
    rt.block_on(future::lazy(|_| debug!("in tokio::spawn()")));
    println!("");
}
