/**
 * iot-backend
 * IoT backend for home IoT device
 * ramhdi, 26/01/2023
 */
// import local modules
mod api;
mod connector;
mod model;

// import crates
use api::retriever::{get_data_by_id, get_dummy_data, get_latest_data};
use axum::{routing::get, Router};
use mongodb::Client;

#[tokio::main]
async fn main() {
    // MongoDB init
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();
    //let db = MongoDB::init().await.unwrap();

    // API routing
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/getDummyData", get(get_dummy_data))
        .route("/getDataById/:id", get(get_data_by_id))
        .route("/getLatestData", get(get_latest_data))
        .with_state(client);

    // Run the app on localhost
    axum::Server::bind(&"0.0.0.0:1287".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
