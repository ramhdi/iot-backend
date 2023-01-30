use crate::{connector::connector::MongoDB, model::device::DeviceData};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use mongodb::{bson::doc, bson::oid::ObjectId, Client};
use std::time::{SystemTime, UNIX_EPOCH};

// sample function, return dummy data
pub async fn get_dummy_data() -> Json<DeviceData> {
    println!("get_dummy_data");
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let dummy_data = DeviceData {
        id: None,
        timestamp: ts,
        device_id: "488188e2-1c9a-4c65-a83f-ef4b8cb640f1".to_owned(),
        temperature: 23.88,
        humidity: 32,
        accel_x: 0.002,
        accel_y: -0.001,
        accel_z: -9.81,
    };

    return Json(dummy_data);
}

pub async fn get_data_by_id(
    State(client): State<Client>,
    oid: Path<String>,
) -> Json<Option<DeviceData>> {
    println!("get_data_by_id");
    let id = oid.0;

    let col = MongoDB::init_collection(client)
        .await
        .unwrap()
        .get_collection();

    let find_result = col
        .find_one(doc! {"_id": ObjectId::parse_str(id).unwrap()}, None)
        .await;
    let res = match find_result {
        Ok(None) => None,
        Ok(Some(data)) => Some(data),
        Err(_) => None,
    };
    return Json(res);
}
