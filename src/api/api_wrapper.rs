// REST API wrapper

use crate::{
    api::*,
    model::device::{DeviceData, TSRequest},
};
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
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string())),
        Ok(dummy) => return (StatusCode::OK, Ok(Json(dummy))),
    }
}

// get_data_by_id
pub async fn get_data_by_id_wrapped(
    State(client): State<Client>,
    oid: Path<String>,
) -> impl IntoResponse {
    println!("get_data_by_id");
    let res: Result<Option<DeviceData>, Box<dyn Error>> =
        retriever::get_data_by_id(client, oid.0).await;
    match res {
        Err(err) => {
            if let Some(mongo_err) = err.downcast_ref::<mongodb::error::Error>() {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Err(mongo_err.to_string()),
                );
            } else if let Some(bson_err) = err.downcast_ref::<mongodb::bson::oid::Error>() {
                return (StatusCode::BAD_REQUEST, Err(bson_err.to_string()));
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR, Err(err.to_string()));
            }
        }
        Ok(None) => return (StatusCode::NOT_FOUND, Err(String::from("Not found"))),
        Ok(Some(data)) => return (StatusCode::OK, Ok(Json(data))),
    }
}

// get_latest_data
pub async fn get_latest_data_wrapped(State(client): State<Client>) -> impl IntoResponse {
    println!("get_latest_data");
    let res: Result<Option<DeviceData>, mongodb::error::Error> =
        retriever::get_latest_data(client).await;
    match res {
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string())),
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Err(String::from("Database is empty")),
            )
        }
        Ok(Some(data)) => return (StatusCode::OK, Ok(Json(data))),
    }
}

// get_all_data
pub async fn get_all_data_wrapped(State(client): State<Client>) -> impl IntoResponse {
    println!("get_all_data");
    let res: Result<Vec<DeviceData>, mongodb::error::Error> = retriever::get_all_data(client).await;
    match res {
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string())),
        Ok(data_vec) => return (StatusCode::OK, Ok(Json(data_vec))),
    }
}

// get_time_series_data
pub async fn get_time_series_data_wrapped(
    State(client): State<Client>,
    Json(request): Json<TSRequest>,
) -> impl IntoResponse {
    println!("get_time_series_data");
    let res = retriever::get_time_series_data(client, request).await;
    match res {
        Err(err) => {
            if let Some(mongo_err) = err.downcast_ref::<mongodb::error::Error>() {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Err(mongo_err.to_string()),
                );
            } else if err.to_string() == "Invalid param" {
                return (StatusCode::BAD_REQUEST, Err(err.to_string()));
            } else {
                return (StatusCode::INTERNAL_SERVER_ERROR, Err(err.to_string()));
            }
        }
        Ok(None) => return (StatusCode::NOT_FOUND, Err(String::from("No data"))),
        Ok(Some(time_series)) => return (StatusCode::OK, Ok(Json(time_series))),
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
        Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string())),
        Ok(oid) => return (StatusCode::OK, Ok(oid)),
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
                Err(e) => return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string())),
                Ok(oid) => return (StatusCode::OK, Ok(oid)),
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
            let object_ids: Vec<String> = inserted
                .inserted_ids
                .values()
                .filter_map(|value| {
                    if let mongodb::bson::Bson::ObjectId(object_id) = value {
                        Some(object_id.to_string())
                    } else {
                        None
                    }
                })
                .collect();
            return (StatusCode::CREATED, Ok(Json(object_ids)));
        }
    }
}
