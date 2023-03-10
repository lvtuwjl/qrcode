// // use actix::clock::sleep;
// use etcd_client::{Client, PutOptions};
// // use log::{error, info};
// // use sonya_meta::api::{sleep_between_reconnects, MAX_RECONNECT_ATTEMPTS};
// use sonya_meta::config::Shards;
// use std::time::Duration;
// use crate::utils;
//
// const DEFAULT_TTL: i64 = 5; // lease租约时长
// const DEFAULT_SLEEP: Duration = Duration::from_secs(2);
// // const DEFAULT_SLEEP: u64 = 2;
// const MAX_RECONNECT_ATTEMPTS:u8 = 10; // 重试次数
//
// /// 服务注册
// pub async fn register_instance(
//     uris: Shards,
//     prefix: String,
//     instance_id: String,
//     instance_addr: String,
// ) {
//     let key = format!("{}/{}", prefix, instance_id);
//     let value = instance_addr;
//
//     let mut attempts: u8 = 0;
//
//     loop {
//         // maximum attempts
//         if attempts == MAX_RECONNECT_ATTEMPTS {
//             error!("registration in etcd failed more then 10 times");
//             return;
//         }
//         let client_r = Client::connect(uris.clone(), None).await;
//
//         match client_r {
//             Ok(c) => {
//                 register_instance_impl(c, &key, &value).await;
//                 attempts = 0
//             }
//             Err(e) => {
//                 error!("connection error: {}", e);
//                 attempts += 1
//             }
//         }
//
//         // sleep_between_reconnects(attempts).await
//         tokio::time::sleep(DEFAULT_SLEEP)
//     }
// }
//
// async fn register_instance_impl(mut client: Client, key: &str, value: &str) {
//     let cli = match client.lease_grant(DEFAULT_TTL, None).await {
//         Ok(cli) => cli,
//         Err(e) => {
//             error!("lease grant error: {}", e);
//             return;
//         }
//     };
//
//     if let Err(e) = client
//         .put(
//             key.as_bytes(),
//             value.as_bytes(),
//             Some(PutOptions::new().with_lease(cli.id())),
//         )
//         .await
//     {
//         error!("putting instance addr error: {}", e);
//         return;
//     }
//
//     info!("register instance in etcd, key: {}, value: {}", key, value);
//
//     loop {
//         if let Err(e) = client.lease_keep_alive(cli.id()).await {
//             error!("lease keep alive error: {}", e);
//             return;
//         }
//
//         // utils::time::sleep(DEFAULT_SLEEP); // 休眠
//         tokio::time::sleep(DEFAULT_SLEEP)
//     }
// }
