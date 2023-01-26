// device model

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub timestamp: u64,
    pub device_id: String,
    pub temperature: f64,
    pub humidity: u8,
    pub accel_x: f64,
    pub accel_y: f64,
    pub accel_z: f64,
}
