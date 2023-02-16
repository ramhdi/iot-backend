use crate::{connector::connector::MongoDB, model::device::DeviceData};
use mongodb::Client;

// post data
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
