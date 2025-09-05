use std::sync::Arc;
use warp::Filter;
use crate::types::Transaction;
use crate::chain::Blockchain;
use crate::storage::Storage;
use serde_json::json;

pub fn routes(shared_chain: Arc<tokio::sync::Mutex<Blockchain>>, db: Arc<Storage>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let get_chain = warp::path!("chain")
        .and(warp::get())
        .and_then(move || {
            let chain = shared_chain.clone();
            async move {
                let locked = chain.lock().await;
                let body = serde_json::to_string(&*locked).unwrap();
                Ok::<_, warp::Rejection>(warp::reply::with_status(body, warp::http::StatusCode::OK))
            }
        });

    let post_tx = warp::path!("tx" / "new")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |tx: Transaction| {
            let chain = shared_chain.clone();
            async move {
                // verify signature before adding
                let valid = crate::wallet::verify_signature(&tx.from, serde_json::to_string(&(&tx.from, &tx.to, &tx.amount)).unwrap().as_bytes(), &tx.signature);
                if !valid { return Ok::<_, warp::Rejection>(warp::reply::with_status("invalid signature", warp::http::StatusCode::BAD_REQUEST)); }
                let mut c = chain.lock().await;
                c.add_transaction(tx);
                Ok(warp::reply::with_status("tx added", warp::http::StatusCode::CREATED))
            }
        });

    let post_mine = warp::path!("mine")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(move |params: serde_json::Value| {
            let chain = shared_chain.clone();
            let db = db.clone();
            async move {
                let miner = params.get("miner").and_then(|v| v.as_str()).unwrap_or("miner").to_string();
                let mut c = chain.lock().await;
                if c.mempool.is_empty() { return Ok::<_, warp::Rejection>(warp::reply::with_status("no txs", warp::http::StatusCode::BAD_REQUEST)); }
                if let Some(block) = crate::miner::mine_block(&mut c, miner.clone()) {
                    // persist chain snapshot
                    let serialized = serde_json::to_vec(&*c).unwrap();
                    let _ = db.set(b"chain", &serialized);
                    let body = json!({"mined": block.index});
                    Ok(warp::reply::with_status(serde_json::to_string(&body).unwrap(), warp::http::StatusCode::OK))
                } else {
                    Ok(warp::reply::with_status("mine failed", warp::http::StatusCode::INTERNAL_SERVER_ERROR))
                }
            }
        });

    get_chain.or(post_tx).or(post_mine)
}
