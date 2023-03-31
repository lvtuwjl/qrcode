#![allow(unused)]

#[macro_use]
extern crate log;

mod client;
mod registry;
mod server;
mod utils;

use futures::TryFutureExt;
use std::fmt::Debug;
// use futures::executor;
use registry::pb::HelloRequest;

// mod pb {
//     include!("../proto/helloworld/helloworld.rs");
// }

#[tokio::main]
async fn main() -> Result<(), etcd_client::Error> {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Info)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    use clap::builder::Arg;
    use clap::builder::Command;
    debug!("run grpc with tonic!");
    let matches = Command::new("grpc with tonic")
        .version("0.1")
        .author("wjl")
        .about("grpc-demo")
        .subcommand(Command::new("client").about("start client"))
        .subcommand(Command::new("server").about("start server"))
        .subcommand(
            Command::new("startserver")
                .about("start the grpc server")
                .arg(
                    Arg::new("port")
                        .short('p')
                        .long("port")
                        .help("the port bind to locally"),
                ),
        )
        // .subcommand(
        //     Command::new("startclient").about("start an client").arg(
        //         Arg::new("port")
        //             .short('p')
        //             .long("port")
        //             .help("the port for client"),
        //     ),
        // )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("client") {
        info!("start client success!");
        client::main().await.unwrap();
    } else if let Some(_) = matches.subcommand_matches("server") {
        info!("start server success!");
        server::main().await.unwrap();
    } else if let Some(matches) = matches.subcommand_matches("startserver") {
        info!("server:{}", "start grpc server");
        if let Some(port) = matches.get_one::<String>("port") {
            info!("listen port:{}", port);
            // start grpc server
            server::main().await.unwrap();
        }
        // } else if let Some(ref matches) = matches.subcommand_matches("startclient") {
        //     info!("client:{}", "start client");
        //     if let Some(port) = matches.get_one::<String>("port") {
        //         info!("client port:{}", port);
        //         super::client::start().await?;
        //     }
    }

    async_stream::stream! {}

    Ok(())
}
