use axum::{
    Router,
    routing::{get, post},
};
pub mod error;
pub mod handlers;
pub mod store;
use handlers::{redirect_url, shorten_url};
use store::new_db;

pub fn create_app() -> Router {
    let db = new_db();

    Router::new()
        .route("/shorten", post(shorten_url))
        .route("/{code}", get(redirect_url))
        .with_state(db)
}
