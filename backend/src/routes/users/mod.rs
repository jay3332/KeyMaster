mod get_user;

pub use get_user::*;

use axum::routing::get;
use axum::Router;

pub fn make_user_routes() -> Router {
    Router::new()
        .route("/users/:id", get(get_user))
}