use axum::{http::StatusCode, routing::get, Router};
use std::net::SocketAddr;

pub async fn entrypoint() {
    let router = Router::new()
        .route("/", get(async || (StatusCode::OK, "Hello, world!")))
        .merge(crate::routes::make_auth_routes())
        .merge(crate::routes::make_user_routes())
        .merge(crate::routes::make_quotes_routes())
        .merge(crate::routes::make_ws_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 8081));
    let server = axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .with_graceful_shutdown(async {
            tokio::signal::ctrl_c()
                .await
                .expect("Failed to listen for Ctrl+C.");
        });

    server.await.expect("Failed to start HTTP server.");
}
