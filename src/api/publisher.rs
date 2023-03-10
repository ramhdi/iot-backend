// Data publisher

use crate::{api_error::error::APIError, connector::connector::MongoDB, model::device::DeviceData};
use axum::{extract::State, Json};
use mongodb::{results::InsertManyResult, Client};

// Post new data
pub async fn post_data(
    State(client): State<Client>,
    Json(device_data): Json<DeviceData>,
) -> Result<String, APIError> {
    return Ok(MongoDB::init_collection(client)
        .await?
        .get_collection()
        .insert_one(device_data, None)
        .await?
        .inserted_id
        .as_object_id()
        .unwrap_or_default()
        .to_string());
}

pub async fn post_batch_data(
    State(client): State<Client>,
    Json(device_data_vec): Json<Vec<DeviceData>>,
) -> Result<Json<InsertManyResult>, APIError> {
    return Ok(Json(
        MongoDB::init_collection(client)
            .await?
            .get_collection()
            .insert_many(device_data_vec, None)
            .await?,
    ));
}
