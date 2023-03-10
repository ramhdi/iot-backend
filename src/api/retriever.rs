// Data retriever

use crate::{api_error::error::APIError, connector::connector::MongoDB, model::device::*};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    Client,
};
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

// Get dummy data
pub async fn get_dummy_data() -> Result<Json<DeviceData>, APIError> {
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

    return Ok(Json(dummy_data));
}

// Get entry by object id
pub async fn get_data_by_oid(
    State(client): State<Client>,
    oid: Path<String>,
) -> Result<Json<Option<DeviceData>>, APIError> {
    let id = ObjectId::parse_str(oid.0)?;
    let filter = doc! {"_id": id};
    return Ok(Json(
        MongoDB::init_collection(client)
            .await?
            .get_collection()
            .find_one(filter, None)
            .await?,
    ));
}

// Get latest entry
pub async fn get_latest_data(
    State(client): State<Client>,
) -> Result<Json<Option<DeviceData>>, APIError> {
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
        return Ok(Json(None));
    }

    return Ok(Json(Some(cursor.deserialize_current()?)));
}

// Get all entries
pub async fn get_all_data(State(client): State<Client>) -> Result<Json<Vec<DeviceData>>, APIError> {
    let mut cursor = MongoDB::init_collection(client)
        .await?
        .get_collection()
        .find(None, None)
        .await?;

    let mut result: Vec<DeviceData> = Vec::new();
    while cursor.advance().await? {
        result.push(cursor.deserialize_current()?);
    }

    return Ok(Json(result));
}

// Get historical data by device_id, start and end time
pub async fn get_hist_data(
    State(client): State<Client>,
    request: Query<TSRequest>,
) -> Result<Json<Vec<DeviceData>>, APIError> {
    let filter = doc! {
        "device_id": request.0.device_id,
        "timestamp": {
            "$gt" : request.0.start as i64,
            "$lt" : request.0.end as i64
        }
    };

    let mut cursor = MongoDB::init_collection(client)
        .await?
        .get_collection()
        .find(filter, None)
        .await?;

    let mut ts_data: Vec<DeviceData> = Vec::new();
    while cursor.advance().await? {
        ts_data.push(cursor.deserialize_current()?);
    }

    return Ok(Json(ts_data));
}
