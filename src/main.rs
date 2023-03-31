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

    // let (tx, mut rx) = tokio::sync::mpsc::channel(100);
    // tokio::spawn(async move {
    //     for i in 0..10 {
    //         // std::thread::sleep(std::time::Duration::from_secs(2));
    //         let sd = tx.send(i).await;
    //         println!("receiver dropped:{i} {:?}", sd);
    //         // return;
    //     }
    // });
    //
    // while let Some(i) = rx.recv().await {
    //     // std::thread::sleep(std::time::Duration::from_secs(2));
    //     println!("got = {}", i);
    // }

    // server register
    // registry::put();

    // exec watch loop block
    // registry::watch().await?;
    // registry::lease_keep_alive().await?;
    //
    // std::thread::sleep(std::time::Duration::from_secs(20));

    // use tokio::sync::mpsc::channel;
    //
    // let (request_sender, mut request_receiver) = channel::<i32>(2);
    // let request_stream = ReceiverStream::new(request_receiver);

    // let rs = request_sender.send(12).await;
    // println!("rs:{:?}", rs);
    //
    // let rs1 = request_sender.send(13).await;
    // println!("rs1:{:?}", rs1);
    //
    // let ss = request_receiver.recv().await;
    //
    // println!("ss:{:?}", ss);

    // let response_stream = self.inner.watch(request_stream).await?.into_inner();
    // let mut watch_stream = WatchStream::new(response_stream);

    // let watch_id = match watch_stream.message().await? {
    //     Some(resp) => {
    //         assert!(resp.created(), "not a create watch response");
    //         resp.watch_id()
    //     }
    //     None => {
    //         return Err(Error::WatchError("failed to create watch".to_string()));
    //     }
    // };

    // Ok((Watcher::new(watch_id, request_sender), watch_stream))

    // use utils::time;
    // println!("{} {} {} {}",time::secs(),time::millis(),time::micros(),time::nanos());
    // time::sleep(3);

    Ok(())
}
