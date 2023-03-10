/**
 * iot-backend
 * IoT backend for home IoT device
 * ramhdi, 26/01/2023
 */
// Import local modules
mod api;
mod api_error;
mod connector;
mod model;

// Import crates
// use api::api_wrapper::*;
use api::publisher::*;
use api::retriever::*;
use axum::{routing::get, routing::post, Router};
use mongodb::Client;
use std::net::SocketAddr;
use tokio::signal;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    // Logging init
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // MongoDB init
    let client = Client::with_uri_str("mongodb://localhost:27017")
        .await
        .unwrap();

    // CORS layer
    let cors = CorsLayer::new().allow_origin(Any).allow_headers(Any);

    // API routing
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/device_data", post(post_data).get(get_hist_data))
        .route("/device_data/batch", post(post_batch_data))
        .route("/device_data/:oid", get(get_data_by_oid))
        .route("/device_data/dummy", get(get_dummy_data))
        .route("/device_data/latest", get(get_latest_data))
        .route("/device_data/all", get(get_all_data))
        .with_state(client)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Run the app on localhost
    let address = SocketAddr::from(([127, 0, 0, 1], 1287));
    tracing::debug!("listening on {}", address);
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

// Graceful shutdown handler
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    println!("signal received, starting graceful shutdown");
}
