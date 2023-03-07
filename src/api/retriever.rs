// Data retriever

use crate::{connector::connector::MongoDB, model::device::*};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    Client,
};
use rand::Rng;
use std::{
    error::Error,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};

// Get dummy data
pub async fn get_dummy_data() -> Result<DeviceData, SystemTimeError> {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis() as u64;

    let mut rng = rand::thread_rng();
    let dummy_data = DeviceData {
        id: None,
        timestamp: ts,
        device_id: String::from("488188e2-1c9a-4c65-a83f-ef4b8cb640f1"),
        temperature: rng.gen_range(0.0..100.0),
        humidity: rng.gen_range(0..100),
        accel_x: rng.gen_range(-1.0..1.0),
        accel_y: rng.gen_range(-1.0..1.0),
        accel_z: rng.gen_range(-10.0..10.0),
    };

    return Ok(dummy_data);
}

// Get entry by object id
pub async fn get_data_by_id(
    client: Client,
    oid: String,
) -> Result<Option<DeviceData>, Box<dyn Error>> {
    let id = ObjectId::parse_str(oid)?;
    let filter = doc! {"_id": id};
    return Ok(MongoDB::init_collection(client)
        .await?
        .get_collection()
        .find_one(filter, None)
        .await?);
}

// Get latest entry
pub async fn get_latest_data(client: Client) -> Result<Option<DeviceData>, mongodb::error::Error> {
    let find_options = FindOptions::builder()
        .sort(doc! {"timestamp": -1})
        .limit(1)
        .build();

    let mut cursor = MongoDB::init_collection(client)
        .await?
        .get_collection()
        .find(None, find_options)
        .await?;

    let ok = cursor.advance().await?;
    if !ok {
        return Ok(None);
    }

    return Ok(Some(cursor.deserialize_current()?));
}

// Get all entries
pub async fn get_all_data(client: Client) -> Result<Vec<DeviceData>, mongodb::error::Error> {
    let mut cursor = MongoDB::init_collection(client)
        .await?
        .get_collection()
        .find(None, None)
        .await?;

    let mut result: Vec<DeviceData> = Vec::new();
    while cursor.advance().await? {
        result.push(cursor.deserialize_current()?);
    }

    return Ok(result);
}

// Get data as time-series
pub async fn get_time_series_data(
    client: Client,
    request: TSRequest,
) -> Result<Option<TSData>, Box<dyn Error>> {
    let filter = doc! {
        "device_id": request.device_id,
        "timestamp": {
            "$gt" : request.start as i64,
            "$lt" : request.end as i64
        }
    };

    let mut cursor = MongoDB::init_collection(client)
        .await?
        .get_collection()
        .find(filter, None)
        .await?;

    let mut timestamp: Vec<u64> = Vec::new();
    let mut ts_data: Vec<DeviceParam> = Vec::new();

    while cursor.advance().await? {
        timestamp.push(cursor.deserialize_current()?.timestamp);
        match request.param.as_str() {
            "temperature" => ts_data.push(DeviceParam::Temperature(
                cursor.deserialize_current()?.temperature,
            )),
            "humidity" => ts_data.push(DeviceParam::Humidity(
                cursor.deserialize_current()?.humidity,
            )),
            "accel_x" => ts_data.push(DeviceParam::Accel(cursor.deserialize_current()?.accel_x)),
            "accel_y" => ts_data.push(DeviceParam::Accel(cursor.deserialize_current()?.accel_y)),
            "accel_z" => ts_data.push(DeviceParam::Accel(cursor.deserialize_current()?.accel_z)),
            _ => return Err(String::from("Invalid param").into()),
        }
    }

    if timestamp.len() == 0 {
        return Ok(None);
    }

    return Ok(Some(TSData {
        timestamp: timestamp,
        data: ts_data,
    }));
}
