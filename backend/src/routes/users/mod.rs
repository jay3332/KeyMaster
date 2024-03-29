mod create_user;
mod get_self;
mod get_user;

pub use create_user::create_user;
pub use get_self::get_me;
pub use get_user::get_user;

use axum::routing::{get, post};
use axum::Router;

pub fn make_user_routes() -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .route("/users/me", get(get_me))
}
