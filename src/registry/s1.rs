// use crate::service_discovery::ServiceDiscoveryStreamFactory;
// use etcd_client::*;
// use log::error;
// use std::collections::HashMap;
//
// pub fn factory(uris: Vec<String>, prefix: String) -> ServiceDiscoveryStreamFactory {
//     Box::new(move || {
//         let uris = uris.clone();
//         let prefix = prefix.clone();
//         Box::pin(async_stream::stream! {
//             let mut client = match Client::connect(
//                 uris,
//                 None,
//             )
//             .await {
//                 Ok(c) => c,
//                 Err(e) => {
//                     error!("connection error {}", e);
//                     return
//                 }
//             };
//
//             let resp = match client
//                 .get(prefix.clone(), Some(GetOptions::new().with_prefix()))
//                 .await {
//                 Ok(c) => c,
//                 Err(e) => {
//                     error!("getting list error {}", e);
//                     return
//                 }
//             };
//
//             let mut registry_list: HashMap<String, String> = resp
//                 .kvs()
//                 .iter()
//                 .map(|kv| {
//                     (
//                         kv.key_str().expect("key is invalid utf-8").into(),
//                         kv.value_str().expect("value is invalid utf-8").into(),
//                     )
//                 })
//                 .collect();
//
//             let (_, mut stream) = match client
//                 .watch(prefix, Some(WatchOptions::new().with_prefix()))
//                 .await {
//                 Ok(c) => c,
//                 Err(e) => {
//                     error!("watching error {}", e);
//                     return
//                 }
//             };
//
//             yield registry_list.values().cloned().collect();
//
//             while let Ok(Some(resp)) = stream.message().await {
//                 if resp.canceled() {
//                     break;
//                 }
//
//                 for event in resp.events() {
//                     if let Some(kv) = event.kv() {
//                         match event.event_type() {
//                             EventType::Put => registry_list.insert(
//                                 kv.key_str().expect("key is invalid utf-8").into(),
//                                 kv.value_str().expect("value is invalid utf-8").into(),
//                             ),
//                             EventType::Delete => {
//                                 registry_list.remove(kv.key_str().expect("key is invalid utf-8"))
//                             }
//                         };
//
//                         yield registry_list.values().cloned().collect();
//                     }
//                 }
//             }
//         })
//     })
// }
