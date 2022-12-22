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
}
