use crate::{api::*, model::device::DeviceData};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongodb::Client;
use std::{error::Error, time::SystemTimeError};

// get_dummy_data
pub async fn get_dummy_data_wrapped() -> impl IntoResponse {
    println!("get_dummy_data");
    let res: Result<DeviceData, SystemTimeError> = retriever::get_dummy_data().await;
    match res {
        Ok(dummy) => return (StatusCode::OK, Ok(Json(dummy))),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string())),
    }
}

// get_data_by_id
pub async fn get_data_by_id_wrapped(
    State(client): State<Client>,
    oid: Path<String>,
) -> impl IntoResponse {
    println!("get_data_by_id");
    let res: Result<Option<DeviceData>, Box<dyn Error>> =
        retriever::get_data_by_id(client, oid).await;
    match res {
        Ok(Some(data)) => return (StatusCode::OK, Ok(Json(data))),
        Ok(None) => return (StatusCode::NOT_FOUND, Err(String::from("Not found"))),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string())),
    }
}

// get_latest_data
pub async fn get_latest_data_wrapped(State(client): State<Client>) -> impl IntoResponse {
    println!("get_latest_data");
    let res: Result<Option<DeviceData>, mongodb::error::Error> =
        retriever::get_latest_data(client).await;
    match res {
        Ok(Some(data)) => return (StatusCode::OK, Ok(Json(data))),
        Ok(None) => return (StatusCode::NOT_FOUND, Err(String::from("Database empty"))),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string())),
    }
}

// post_data
pub async fn post_data_wrapped(
    State(client): State<Client>,
    Json(device_data): Json<DeviceData>,
) -> impl IntoResponse {
    println!("post_data");
    let res: Result<String, mongodb::error::Error> =
        publisher::post_data(client, device_data).await;
    match res {
        Ok(str) => return (StatusCode::OK, Ok(str)),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string())),
    }
}
