mod add_quote;
mod add_quotes;
mod get_quote;
mod get_random_quote;

pub use add_quote::*;
pub use add_quotes::*;
pub use get_quote::*;
pub use get_random_quote::*;

use axum::routing::{get, post};
use axum::Router;

pub fn make_quotes_routes() -> Router {
    Router::new()
        .route("/quotes", get(get_random_quote).post(add_quote))
        .route("/quotes/:id", get(get_quote))
        .route("/quotes/bulk", post(add_quotes))
}
