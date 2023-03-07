use crate::primitives::Block;
use futures::{Stream, StreamExt};

/// Consumes blocks from the stream and pushes them to kafka
pub async fn push_blocks<T>(mut stream: T)
where
    T: Stream<Item = Block> + Unpin,
{
    // this dummy implementation will be replaced with real one sending blocks to kafka
    while let Some(next) = stream.next().await {
        println!("Got block with id {}", next.id);
    }
}
