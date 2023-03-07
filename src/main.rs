use crate::cli::Args;
use crate::kafka::push_blocks;
use crate::primitives::Block;
use crate::substrate::{connect_to_node, stream_finalized_blocks};
use clap::Parser;
use futures::SinkExt;

mod cli;
mod kafka;
mod primitives;
mod substrate;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    // "wss://astar.api.onfinality.io:443/public-ws"

    let client = connect_to_node(&args.substrate_node_url).await;
    let (tx, rx) = futures::channel::mpsc::channel::<Block>(1000);

    tokio::spawn(async move {
        stream_finalized_blocks(&client, tx.sink_map_err(|_| ())).await;
    });

    push_blocks(rx).await;
    Ok(())
}
