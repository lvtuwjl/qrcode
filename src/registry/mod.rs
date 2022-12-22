//! service register and discover
//!
//!
mod discovery;
mod etcd;
mod etcd1;
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
        println!("22:{:?}", resp);
    }

    Ok(())
}

// async fn test_watch() -> Result<()> {
//     let mut client = get_client().await?;
//
//     let (mut watcher, mut stream) = client.watch("watch01", None).await?;
//
//     client.put("watch01", "01", None).await?;
//
//     let resp = stream.message().await?.unwrap();
//     assert_eq!(resp.watch_id(), watcher.watch_id());
//     assert_eq!(resp.events().len(), 1);
//
//     let kv = resp.events()[0].kv().unwrap();
//     assert_eq!(kv.key(), b"watch01");
//     assert_eq!(kv.value(), b"01");
//     assert_eq!(resp.events()[0].event_type(), EventType::Put);
//
//     watcher.cancel().await?;
//
//     let resp = stream.message().await?.unwrap();
//     assert_eq!(resp.watch_id(), watcher.watch_id());
//     assert!(resp.canceled());
//
//     Ok(())
// }
