// Device data model

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize, Serializer};

// Local function to serialize BSON OID as string
fn serialize_object_id<S>(object_id: &Option<ObjectId>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match object_id {
        Some(ref object_id) => serializer.serialize_some(object_id.to_string().as_str()),
        None => serializer.serialize_none(),
    }
}

// Raw device data
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "serialize_object_id")]
    pub id: Option<ObjectId>,
    pub timestamp: u64,
    pub device_id: String,
    pub temperature: f64,
    pub humidity: u8,
    pub accel_x: f64,
    pub accel_y: f64,
    pub accel_z: f64,
}

// Time-series device data request format
#[derive(Debug, Serialize, Deserialize)]
pub struct TSRequest {
    pub device_id: String,
    pub start: u64,
    pub end: u64,
}
