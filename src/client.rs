use crate::pb::HelloReply;
use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use std::fmt::Debug;
use tonic::Response;

pub mod hello_world {
    // tonic::include_proto!("helloworld");
    include!("../proto/helloworld/helloworld.rs");
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;

    info!("RESPONSE:{:?}", response);
    use core::fmt::Debug;

    // println!("RESPONSE={:#?}", response);

    Ok(())
}
