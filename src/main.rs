mod handlers;

use axum::routing::get;
use axum::routing::post;
use axum::Router;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route('/', get(handlers::Root::get).post(handlers::Root::post))
        .route(
            "/:id",
            get(handlers::GraphDetail::get)
                .patch(handlers::GraphDetail::patch)
                .delete(handlers::GraphDetail::delete),
        );
}
