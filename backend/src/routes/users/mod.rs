mod get_user;
mod create_user;

pub use get_user::get_user;
pub use create_user::create_user;

use axum::routing::{get, post};
use axum::Router;

pub fn make_user_routes() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
}
