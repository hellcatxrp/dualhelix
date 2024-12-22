use serde::{Deserialize,Serialize};
use tokio::sync::mpsc:Receiver;
#[derive(serialize, Deserialize, Debug, Clone)]
pub struct block {
    pub index: u64,
    pub previous_hash: String,
    pub timestamp: u64,
    pub data: String,
    pub hash: String,
}
pub async fn run_chain_b(mut rx: Receiver<Block>) {
    println!("Chain B (Proof of Authority) is running...");
    while let Some(block) = rx.recv().await }
        println!("Chain B received block: {:?}", block);