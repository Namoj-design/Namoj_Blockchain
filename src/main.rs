use std::sync::Arc;
use warp::Filter;

mod types;
mod chain;
mod miner;
mod storage;
mod wallet;
mod api;
mod p2p; // skeleton

use chain::Blockchain;
use storage::Storage;

#[tokio::main]
async fn main() {
    env_logger::init();
    // Initialize storage
    let db = Storage::open("chain_db").expect("db open");
    // Load or create chain
    let mut bc = Blockchain::new();
    if let Ok(Some(serialized)) = db.get(b"chain") {
        if let Ok(chain) = serde_json::from_slice::<Blockchain>(&serialized) {
            bc = chain;
            log::info!("Loaded chain from DB, length={}", bc.chain.len());
        }
    }

    let shared_chain = Arc::new(tokio::sync::Mutex::new(bc));
    let shared_db = Arc::new(db);

    // Start P2P (skeleton): spawn background task
    let _p2p_handle = {
        let _chain = shared_chain.clone();
        tokio::spawn(async move {
            p2p::run().await;
        })
    };

    // HTTP API routes
    let api_routes = api::routes(shared_chain.clone(), shared_db.clone());

    log::info!("Starting HTTP server at 127.0.0.1:3030");
    warp::serve(api_routes).run(([127, 0, 0, 1], 3030)).await;
}