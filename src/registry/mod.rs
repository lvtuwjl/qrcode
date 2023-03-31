//! service register and discover
//!
//!
mod discovery;
mod etcd;
mod etcd1;
pub mod pb;
mod registry;
mod s1;
mod s2;
mod s3;

pub async fn watch() -> Result<(), etcd_client::Error> {
    let mut dc = discovery::DiscoveryClient::new();

    // watch key
    // let mut wc = dc.cli();
    let (mut watcher, mut stream) = dc.watch("watch02", None).await?;

    // watcher.cancel().await?;

    loop {
        // println!("1111:{:#?}", watcher);
        let resp = stream.message().await?.unwrap();
        println!("22:{:#?}", resp);
    }

    Ok(())
}

pub async fn put() -> Result<(), etcd_client::Error> {
    let mut dc = discovery::DiscoveryClient::new();
    dc.put("watch01", "01", None).await?;

    Ok(())
}

pub async fn get() -> Result<(), etcd_client::Error> {
    let mut dc = discovery::DiscoveryClient::new();
    // dc.get("watch01", "01", None).await?;

    // {
    let resp = dc.get("watch01", None).await?;
    assert_eq!(resp.count(), 1);
    // assert!(!resp.more());
    // assert_eq!(resp.kvs().len(), 1);
    // assert_eq!(resp.kvs()[0].key(), b"get11");
    // assert_eq!(resp.kvs()[0].value(), b"11");
    // }

    println!("resp:{:?}", resp);

    Ok(())
}

pub async fn lease_keep_alive() -> Result<(), etcd_client::Error> {
    let mut dc = discovery::DiscoveryClient::new();

    // 7587869215666571603,
    // 7587869215666571603
    let (mut keeper, mut stream) = dc.keep_alive(0x694d87314abcfa33, None).await?;
    println!("1111:{:?}", keeper);

    tokio::spawn(async move {
        for i in 0..3 {
            tokio::time::sleep(std::time::Duration::from_secs(20)).await;
            let hh = keeper.keep_alive().await;
            println!("hh:{:?}", hh);
        }
    });

    loop {
        // std::thread::sleep(std::time::Duration::from_secs(10));
        println!("1111:{:?}", "keeper");
        let resp = stream.message().await?;

        println!("22:{:?}", resp);
    }

    Ok(())
}
