use axum::{http::StatusCode, routing::get, Router};
use std::net::SocketAddr;

pub async fn entrypoint() {
    let router = Router::new().route("/", get(async || (StatusCode::OK, "Hello, world!")));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    let server = axum::Server::bind(&addr).serve(router.into_make_service()).with_graceful_shutdown(async {
        tokio::signal::ctrl_c().await.expect("failed to listen for Ctrl+C");
    });

    server.await.expect("failed to start HTTP server");
}