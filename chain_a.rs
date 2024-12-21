use serde::{Deserialise,Serialize};
use tokio::sync::mpsc::Sender;
use rand::Rng;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index:u64,
    pub previous_hash: String,
    pub timestamp: u64,
    pub data: String,
    pub hash: String,
}
pub struct Validator {
    pub id: String,
    pub stake: u64,
}
pub async fn run_chain_a(tx: Sender<Block>) {
    println!("Chain A (Proof of Stake) is running...");
    let block = Block {
        index:0,
        previous_hash: "0".to_string(),
        timestamp:0,
        data:"Genesis Block".to_string(),
        hash: "abc123".to_string(),
        };
        if let Err(e) = tx.send(block).await {
            eprintln!("Failed to send blocck to Chain B: {}", e);
        }
}