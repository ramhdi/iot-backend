/**
 * iot-backend
 * IoT backend for home IoT device
 * ramhdi, 26/01/2023
 */
// Import local modules
mod api;
mod connector;
mod model;

// Import crates
use api::api_wrapper::*;
use axum::{routing::get, routing::post, Router};
use mongodb::Client;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    // MongoDB init
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();

    // CORS layer
    let cors = CorsLayer::new().allow_origin(Any);

    // API routing
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/device_data", post(post_data_wrapped))
        .route("/device_data/batch", post(post_batch_data_wrapped))
        .route("/device_data/:id", get(get_data_by_id_wrapped))
        .route("/device_data/dummy", get(get_dummy_data_wrapped))
        .route("/device_data/dummy", post(post_dummy_data_wrapped))
        .route("/device_data/latest", get(get_latest_data_wrapped))
        .route("/device_data/all", get(get_all_data_wrapped))
        .with_state(client)
        .layer(cors);

    // Run the app on localhost
    let address = SocketAddr::from(([127, 0, 0, 1], 1287));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
