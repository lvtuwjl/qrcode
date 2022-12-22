#![allow(unused)]

#[macro_use]
extern crate log;

mod client;
mod registry;
mod server;
mod utils;

// use futures::executor;
use pb::HelloRequest;

mod pb {
    include!("../proto/helloworld/helloworld.rs");
}

#[tokio::main]
async fn main() -> Result<(), etcd_client::Error> {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .write_style(env_logger::WriteStyle::Always)
        .init();
    // use server::main;
    // server::main();

    // exec
    registry::watch().await

    // use utils::time;
    // println!("{} {} {} {}",time::secs(),time::millis(),time::micros(),time::nanos());
    // time::sleep(3);
}
