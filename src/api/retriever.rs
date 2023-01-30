use crate::model::device::DeviceData;
use axum::{extract::Path, Json};
use mongodb::bson::oid::ObjectId;
use std::collections::HashMap;
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

    Json(dummy_data)
}

pub async fn get_data_by_id(Path(params): Path<HashMap<String, String>>) -> Json<DeviceData> {
    println!("get_data_by_id");
    let id = params.get("id").unwrap();
    let res = DeviceData {
        id: Some(ObjectId::parse_str(id).unwrap()),
        timestamp: 0,
        device_id: ".".to_owned(),
        temperature: 0.0,
        humidity: 0,
        accel_x: 0.0,
        accel_y: 0.0,
        accel_z: 0.0,
    };
    Json(res)
}
