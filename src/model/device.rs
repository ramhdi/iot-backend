// Device data model

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

// Raw device data
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceData {
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

// Enum for time-series data type
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeviceParam {
    Temperature(f64),
    Humidity(u8),
    Accel(f64),
}

// Time-series device data
#[derive(Debug, Serialize, Deserialize)]
pub struct TSData {
    pub timestamp: Vec<u64>,
    pub data: Vec<DeviceParam>,
}

// Time-series device data request format
#[derive(Debug, Serialize, Deserialize)]
pub struct TSRequest {
    pub device_id: String,
    pub param: String,
    pub start: u64,
    pub end: u64,
}
