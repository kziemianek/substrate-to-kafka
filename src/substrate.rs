use crate::primitives::Block;
use futures::{Sink, StreamExt, TryStreamExt};
use subxt::{OnlineClient, PolkadotConfig};

/// creates client connected to substrate node
pub async fn connect_to_node(url: &str) -> OnlineClient<PolkadotConfig> {
    OnlineClient::<PolkadotConfig>::from_url(url)
        .await
        .expect("Could not connect to node")
}

/// listens to blocks from substrate node and pushes them to provided sink
pub async fn stream_finalized_blocks<T>(client: &OnlineClient<PolkadotConfig>, sink: T)
where
    T: Sink<Block, Error = ()>,
{
    client
        .blocks()
        .subscribe_finalized()
        .await
        .unwrap()
        .map_ok(|b| {
            let block_number = b.header().number;
            Block { id: block_number }
        })
        .map_err(|_| ())
        .forward(sink)
        .await
        .unwrap();
}
