use std::sync::Arc;
use tokio::sync::Mutex;
use env_logger;
use log::info;
use warp::Filter;

mod api;
mod chain;
mod miner;
mod p2p;
mod storage;
mod types;
mod wallet;

use types::Blockchain;
use storage::Storage;

#[tokio::main]
async fn main() {
    env_logger::init();

    // Initialize storage
    let db = Arc::new(Storage::new("blockchain_db").expect("Failed to init storage"));

    // Initialize blockchain
    let bc = Blockchain::new();
    let shared_chain = Arc::new(Mutex::new(bc));

    info!("Blockchain initialized");

    // Start API server
    let api_routes = api::routes(shared_chain.clone(), db.clone());
    info!("Starting HTTP server at 127.0.0.1:3030");

    warp::serve(api_routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
