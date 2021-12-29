mod practice_quote;

use practice_quote::practice_quote;

use axum::routing::get;
use axum::Router;

pub fn make_ws_routes() -> Router {
    Router::new().route("/ws/practice_quote", get(practice_quote))
}
