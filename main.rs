mod chain_a;
mod chain_b;
use tokio::sync::mpsc;
use tokio::task;
#[tokio::main]
async fn main() {let (tx,rx) = mpsc::channel(100);
let chain_a_handle = task::spawn(chain_a::run_chain_a(tx));
let chain_b_handle = task::spawn(chain_b::run_chain_b(rx))
let _ = tokio::join!(chain_a_handle,chain_b_handle);}
