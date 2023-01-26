/*
iot-backend
IoT backend for home IoT device
ramhdi, 26/01/2023
*/

// import local modules
mod api;
mod connector;
mod model;

// import crates
use api::retriever::get_latest_data;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/getLatestData", get(get_latest_data));

    // run it with hyper on localhost:1287
    axum::Server::bind(&"0.0.0.0:1287".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
