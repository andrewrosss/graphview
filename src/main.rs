mod handlers;

use std::net::SocketAddr;

use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Setup tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let public = ServeDir::new("public");
    let app = Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/", get(handlers::Root::get).post(handlers::Root::post))
        .route(
            "/g/:id",
            get(handlers::GraphDetail::get)
                .patch(handlers::GraphDetail::patch)
                .delete(handlers::GraphDetail::delete),
        )
        .fallback_service(public);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
