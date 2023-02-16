// Data publisher

use crate::{connector::connector::MongoDB, model::device::DeviceData};
use mongodb::{results::InsertManyResult, Client};

// Post new data
pub async fn post_data(
    client: Client,
    device_data: DeviceData,
) -> Result<String, mongodb::error::Error> {
    return Ok(MongoDB::init_collection(client)
        .await?
        .get_collection()
        .insert_one(device_data, None)
        .await?
        .inserted_id
        .as_object_id()
        .unwrap()
        .to_string());
}

pub async fn post_batch_data(
    client: Client,
    device_data_vec: Vec<DeviceData>,
) -> Result<InsertManyResult, mongodb::error::Error> {
    return Ok(MongoDB::init_collection(client)
        .await?
        .get_collection()
        .insert_many(device_data_vec, None)
        .await?);
}
