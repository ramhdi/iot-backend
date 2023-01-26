use crate::model::device::Device;
use axum::Json;
use std::time::{SystemTime, UNIX_EPOCH};

// sample function, return dummy data
pub async fn get_latest_data() -> Json<Device> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    let latest_data = Device {
        id: None,
        timestamp: ts,
        device_id: "488188e2-1c9a-4c65-a83f-ef4b8cb640f1".to_owned(),
        temperature: 23.88,
        humidity: 32,
        accel_x: 0.002,
        accel_y: -0.001,
        accel_z: -9.81,
    };

    Json(latest_data)
}
