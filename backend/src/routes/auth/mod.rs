mod login;

use login::*;

use axum::routing::post;
use axum::Router;

pub fn make_auth_routes() -> Router {
    Router::new().route("/login", post(login))
}
