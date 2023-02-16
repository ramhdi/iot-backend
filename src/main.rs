/**
 * iot-backend
 * IoT backend for home IoT device
 * ramhdi, 26/01/2023
 */
// import local modules
mod api;
mod connector;
mod model;

use std::net::SocketAddr;

// import crates
use api::api_wrapper::*;
use axum::{routing::get, routing::post, Router};
use mongodb::Client;

#[tokio::main]
async fn main() {
    // MongoDB init
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();

    // API routing
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/device_data/dummy", get(get_dummy_data_wrapped))
        .route("/device_data/:id", get(get_data_by_id_wrapped))
        .route("/device_data/latest", get(get_latest_data_wrapped))
        .route("/device_data", post(post_data_wrapped))
        .with_state(client);

    // Run the app on localhost
    let address = SocketAddr::from(([127, 0, 0, 1], 1287));
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
