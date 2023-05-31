mod handlers;

use std::net::SocketAddr;

use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handlers::Root::get).post(handlers::Root::post))
        .route(
            "/:id",
            get(handlers::GraphDetail::get)
                .patch(handlers::GraphDetail::patch)
                .delete(handlers::GraphDetail::delete),
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
