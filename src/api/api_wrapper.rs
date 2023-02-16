// REST API wrapper

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

// get_all_data
pub async fn get_all_data_wrapped(State(client): State<Client>) -> impl IntoResponse {
    println!("get_all_data");
    let res: Result<Vec<DeviceData>, mongodb::error::Error> = retriever::get_all_data(client).await;
    match res {
        Ok(data_vec) => return (StatusCode::OK, Ok(Json(data_vec))),
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
        Ok(oid) => return (StatusCode::OK, Ok(oid)),
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string())),
    }
}

// post_dummy_data
pub async fn post_dummy_data_wrapped(State(client): State<Client>) -> impl IntoResponse {
    println!("post_dummy_data");
    let dummy_data_result: Result<DeviceData, SystemTimeError> = retriever::get_dummy_data().await;
    match dummy_data_result {
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string())),
        Ok(dummy_data) => {
            let post_result: Result<String, mongodb::error::Error> =
                publisher::post_data(client, dummy_data).await;
            match post_result {
                Ok(oid) => return (StatusCode::OK, Ok(oid)),
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string())),
            }
        }
    }
}

// post_batch_data
pub async fn post_batch_data_wrapped(
    State(client): State<Client>,
    Json(device_data_vec): Json<Vec<DeviceData>>,
) -> impl IntoResponse {
    println!("post_batch_data");
    let inserted = publisher::post_batch_data(client, device_data_vec).await;
    match inserted {
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string())),
        Ok(inserted) => {
            let mut inserted_str: Vec<String> = Vec::new();
            inserted
                .inserted_ids
                .values()
                .for_each(|oid| inserted_str.push(oid.as_object_id().unwrap().to_string()));
            return (StatusCode::CREATED, Ok(Json(inserted_str)));
        }
    }
}
