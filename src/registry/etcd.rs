// //! etcd 实现
// use std::future;
// use crate::registry::registry::Registry;
// use etcd_client::Client;
//
//
//
// const DEFAULT_TEST_ENDPOINT: &str = "localhost:2379";
//
// /// Get client for testing.
// async fn get_client() ->Client{
//     Client::connect([DEFAULT_TEST_ENDPOINT], None).await.unwrap()
// }
//
// pub struct EtcdOption {
//     pub addr: String,
//     pub timeout_sec: u64,
//     pub protocol: String,
// }
//
// impl Default for EtcdOption {
//     fn default() -> Self {
//         Self {
//             addr: String::from("127.0.0.1:2379"),
//             timeout_sec: 1u64,
//             protocol: "http".to_string(),
//         }
//     }
// }
//
// pub struct EtcdRegistry {
//     // name: String,
//     // id: String,
//     // tags: Vec<String>,
//     // address: String,
//     // port: i32,
//     client:Client,
//     // option:i16,
//     // register:i16,
//     // leases:i16,
// }
//
// // name：要注册的服务的名称
// // id：要注册的服务的ID
// // tags：要注册的服务的标签
// // address：要注册的服务的地址
// // port：要注册的服务的端口
//
// impl EtcdRegistry {
//     pub fn new(name: &str, id: &str, tags: Vec<&str>, addr: &str, port: i32) -> Self {
//
//         Self {
//             name: name.to_string(),
//             id: id.to_string(),
//             tags: tags.iter().map(|t| t.to_string()).collect(),
//             address: addr.to_string(),
//             port,
//             client:futures::executor::block_on(get_client()),
//         }
//     }
//
//     pub fn simple_with_tags(name: &str, tags: Vec<&str>, addr: &str, port: i32) -> Self {
//         Self::new(name, name, tags, addr, port)
//     }
//
//     pub fn simple(name: &str, addr: &str, port: i32) -> Self {
//         Self::simple_with_tags(name, vec![], addr, port)
//     }
//
//     /// register
//     pub async fn register(&self, registration: &Registration) -> Result<(), reqwest::Error> {
//         self.client
//             .put(self.api_url("service/register"))
//             .json(registration)
//             .send()
//             .await?;
//         Ok(())
//     }
//
//     /// deregister
//     pub async fn deregister(&self, service_id: &str) -> Result<(), reqwest::Error> {
//         let deregister_api = format!("service/deregister/{}", service_id);
//         self.client
//             .put(self.api_url(&deregister_api))
//             .json(&())
//             .send()
//             .await?;
//         Ok(())
//     }
//
//     /// services list
//     pub async fn services(&self) -> Result<Services, reqwest::Error> {
//         let list: Services = self
//             .client
//             .get(self.api_url("services"))
//             .send()
//             .await?
//             .json()
//             .await?;
//         Ok(list)
//     }
//
//     /// get_service
//     pub async fn get_service(&self, filter: &Filter) -> Result<Option<Service>, reqwest::Error> {
//         let list = self.services().await?;
//         for (_, s) in list {
//             let has = match &filter {
//                 &Filter::ID(id) => id == &s.id,
//                 &Filter::Service(srv) => srv == &s.service,
//             };
//             if has {
//                 return Ok(Some(s));
//             }
//         }
//         Ok(None)
//     }
//
//     pub fn build() ->Self{
//         Self {
//             name: name.to_string(),
//             id: id.to_string(),
//             tags: tags.iter().map(|t| t.to_string()).collect(),
//             address: addr.to_string(),
//             port,
//             client:
//         }
//     }
// }
//
// impl Registry for EtcdRegistry {
//     // fn register(&self) {}
//     //
//     // fn deregister(&self){}
//     //
//     // fn get_service(&self) {}
// }
//
// /// service register
// async fn register(addr: &str) {
//     println!("正在注册服务");
//     let addrs: Vec<&str> = addr.split(":").collect();
//     let addr = addrs[0];
//     let port: i32 = addrs[1].parse().unwrap();
//     let opt = consul_api::ConsulOption::default();
//     let cs = consul_api::Consul::new(opt).unwrap();
//     let reg = consul_api::Registration::simple("echo-srv", addr, port);
//     cs.register(&reg).await.unwrap();
//     println!("服务注册成功");
// }
//
// /// service discovery
// async fn discovery(service_id: &str) -> Result<String, String> {
//     println!("正在发现服务 {}", service_id);
//     let opt = consul_api::ConsulOption::default();
//     let cs = consul_api::Consul::new(opt).unwrap();
//     let filter = consul_api::Filter::ID(service_id.into());
//     let srv = cs
//         .get_service(&filter)
//         .await
//         .map_err(|err| err.to_string())?;
//     if let Some(srv) = srv {
//         return Ok(format!("http://{}:{}", srv.address, srv.port));
//     }
//     Err("没有发现指定的服务".to_string())
// }
// //...
// async fn echo(Form(EchoInput { message }): Form<EchoInput>) -> Result<Html<String>, String> {
//     let echo_srv_addr = discovery("echo-srv").await?;
//     let mut client = pb::echo_serivce_client::EchoSerivceClient::connect(echo_srv_addr)
//         .await
//         .map_err(|err| err.to_string())?;
//     //...
// }
