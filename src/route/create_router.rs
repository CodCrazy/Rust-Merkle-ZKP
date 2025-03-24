use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use crate::{
    AppState,
    handlers::ddid_handlers::*
};


pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/prove_ddid", post(prove_ddid_handler))
        // .route("/api/is_ddid_member", post(is_ddid_member_handler))
        // .route("/api/add_merchant", post(add_merchant_handler))
        // .route("/api/write_merchant_record", post(write_merchant_record_handler))
        // .route("/api/read_merchant_record", post(read_merchant_record_handler))
        .with_state(app_state)
}