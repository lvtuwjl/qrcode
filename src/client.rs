use crate::registry::pb::greeter_client::GreeterClient;
use crate::registry::pb::{EchoRequest, EchoResponse, HelloReply, HelloRequest};
use futures::stream::Stream;
use std::fmt::Debug;
use std::time::{Duration, UNIX_EPOCH};
use tokio_stream::StreamExt;
use tonic::transport::Channel;
use tonic::{Request, Response};

// pub mod hello_world {
//     // tonic::include_proto!("helloworld");
//     include!("../proto/helloworld/helloworld.rs");
// }

// #[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://127.0.0.1:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    // 查询服务
    // use crate::registry;
    // let g = registry::get().await;
    // match g {
    //     Ok(t) => {
    //         println!("success!!!:{:?}", t);
    //     }
    //     Err(e) => {
    //         println!("failed!!!:{:#?}", e);
    //     }
    // }

    // if let Ok(_) = registry::get().await {
    //     println!("success!!!")
    // } else {
    //     println!("failed!!!")
    // }

    let response = client.say_hello(request).await?;
    info!("RESPONSE:{:?}", response);

    bidirectional_streaming_echo_throttle(&mut client, Duration::from_secs(2)).await;
    // println!("RESPONSE={:#?}", response);
    // std::thread::sleep(std::time::Duration::from_secs(20));
    Ok(())
}

fn echo_requests_iter() -> impl Stream<Item = EchoRequest> {
    tokio_stream::iter(1..usize::MAX).map(|i| EchoRequest {
        message: format!("msg {:02}", i),
    })
}

async fn bidirectional_streaming_echo_throttle(client: &mut GreeterClient<Channel>, dur: Duration) {
    use tokio::sync::mpsc::{channel, Sender};
    use tokio_stream::wrappers::ReceiverStream;
    let (sender, receiver) = channel::<EchoRequest>(100);
    tokio::spawn(async move {
        for i in 0..3 {
            sender
                .send(EchoRequest {
                    message: format!(
                        "tokio demo:{:?}",
                        std::time::SystemTime::now().duration_since(UNIX_EPOCH)
                    ),
                })
                .await;
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    });

    let receiver = ReceiverStream::new(receiver);
    let mut stream = client
        .bidirectional_streaming_echo(receiver)
        .await
        .unwrap()
        .into_inner();

    while let Ok(res) = stream.message().await {
        println!("res:{:?}", res);
    }

    println!("end");
    std::thread::sleep(std::time::Duration::from_secs(20));
}
