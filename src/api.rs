use std::sync::Arc;
use warp::{Filter, Rejection, Reply};
use tokio::sync::Mutex;
use serde_json::json;

use crate::chain::Blockchain;
use crate::storage::Storage;
use crate::types::Transaction;

pub fn routes(
    shared_chain: Arc<Mutex<Blockchain>>,
    db: Arc<Storage>,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    // GET /chain
    let get_chain = warp::path("chain")
        .and(warp::get())
        .and(with_chain(shared_chain.clone()))
        .map(|chain: Arc<Mutex<Blockchain>>| {
            let chain = chain.blocking_lock();
            warp::reply::json(&*chain)
        });

    // POST /tx/new
    let post_tx = warp::path!("tx" / "new")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_chain(shared_chain.clone()))
        .map(|tx: Transaction, chain: Arc<Mutex<Blockchain>>| {
            let mut bc = chain.blocking_lock();
            bc.add_transaction(tx);
            warp::reply::json(&json!({"status": "transaction added"}))
        });

    get_chain.or(post_tx)
}

fn with_chain(
    chain: Arc<Mutex<Blockchain>>,
) -> impl Filter<Extract = (Arc<Mutex<Blockchain>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || chain.clone())
}
