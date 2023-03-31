use etcd_client::Client;
use etcd_client::WatchClient;

const DEFAULT_TEST_ENDPOINT: &'static str = "localhost:2379";

/// Get client for testing.
async fn get_client() -> Client {
    Client::connect([DEFAULT_TEST_ENDPOINT], None)
        .await
        .unwrap()
}

// #[derive(Debug)]
pub struct DiscoveryClient {
    cli: Client,
}

impl DiscoveryClient {
    pub fn new() -> Self {
        Self {
            cli: futures::executor::block_on(get_client()),
        }
    }

    pub async fn watch(
        &mut self,
        key: impl Into<Vec<u8>>,
        options: Option<etcd_client::WatchOptions>,
    ) -> Result<(etcd_client::Watcher, etcd_client::WatchStream), etcd_client::Error> {
        self.cli.watch(key, options).await
    }

    pub async fn put(
        &mut self,
        key: impl Into<Vec<u8>>,
        value: impl Into<Vec<u8>>,
        options: Option<etcd_client::PutOptions>,
    ) -> Result<etcd_client::PutResponse, etcd_client::Error> {
        self.cli.put(key, value, options).await
    }

    pub async fn get(
        &mut self,
        key: impl Into<Vec<u8>>,
        options: Option<etcd_client::GetOptions>,
    ) -> Result<etcd_client::GetResponse, etcd_client::Error> {
        self.cli.get(key, options).await
    }

    pub async fn keep_alive(
        &mut self,
        key: i64,
        options: Option<etcd_client::WatchOptions>,
    ) -> Result<(etcd_client::LeaseKeeper, etcd_client::LeaseKeepAliveStream), etcd_client::Error>
    {
        self.cli.lease_keep_alive(key).await
    }
}
