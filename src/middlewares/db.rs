use crate::utils::context::{TxManager, TX_MANAGER};
use axum::extract::{Request, State};
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use sqlx::{Pool, Postgres};

pub async fn inject(State(db): State<Pool<Postgres>>, req: Request, next: Next) -> Result<Response, (StatusCode, String)> {
    let tx_manager = TxManager {
        db,
        tx: Default::default(),
        is_tx: false,
    };
    Ok(TX_MANAGER.scope(tx_manager, next.run(req)).await)
}