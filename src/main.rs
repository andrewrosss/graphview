mod handlers;

use std::net::Ipv4Addr;
use std::net::SocketAddr;

use axum::routing::get;
use axum::Router;
use tower_http::services::ServeDir;
use tower_http::trace::DefaultMakeSpan;
use tower_http::trace::DefaultOnResponse;
use tower_http::trace::TraceLayer;
use tracing::Level;

#[tokio::main]
async fn main() {
    // Setup tracing
    tracing_subscriber::fmt().init();
    let tracing_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));

    // Build the application
    let app = Router::new()
        .route("/", get(handlers::Root::get).post(handlers::Root::post))
        .route(
            "/g/:id",
            get(handlers::GraphDetail::get)
                .patch(handlers::GraphDetail::patch)
                .delete(handlers::GraphDetail::delete),
        )
        .nest_service("/static", ServeDir::new("public"))
        .layer(tracing_layer);

    let host: Ipv4Addr = std::env::var("HOST")
        .unwrap_or_else(|_| "127.0.0.1".to_string())
        .parse()
        .unwrap();
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .unwrap();
    axum::Server::bind(&SocketAddr::from((host, port)))
        .serve(app.into_make_service())
        .await
        .unwrap();
}
