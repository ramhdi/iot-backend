use crate::{connector::connector::MongoDB, model::device::DeviceData};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::FindOptions,
    Client,
};
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

pub async fn get_data_by_id(State(client): State<Client>, oid: Path<String>) -> impl IntoResponse {
    println!("get_data_by_id");
    let id_str = oid.0;

    let col_result = MongoDB::init_collection(client).await;
    match col_result {
        Ok(c) => {
            // Success connecting to DB
            let col = c.get_collection();
            // Find document with corresponding id
            let id = ObjectId::parse_str(id_str);
            match id {
                Ok(id) => {
                    let filter = doc! {"_id": id};
                    let find_result = col.find_one(filter, None).await;
                    match find_result {
                        Ok(None) => return (StatusCode::NOT_FOUND, Err("Not found")),
                        Ok(Some(data)) => return (StatusCode::OK, Ok(Json(data))),
                        Err(e) => {
                            println!("{}", e);
                            return (StatusCode::INTERNAL_SERVER_ERROR, Err("Error finding data"));
                        }
                    }
                }
                Err(e) => {
                    println!("{}", e);
                    return (StatusCode::BAD_REQUEST, Err("Invalid ID"));
                }
            }
        }
        Err(e) => {
            // Failed connecting to DB
            println!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Err("Failed connecting to DB"),
            );
        }
    }
}

pub async fn get_latest_data(State(client): State<Client>) -> impl IntoResponse {
    println!("get_latest_data");
    let col_result = MongoDB::init_collection(client).await;
    match col_result {
        Ok(c) => {
            // Success connecting to DB
            let col = c.get_collection();
            // Find document with latest timestamp
            let find_options = FindOptions::builder()
                .sort(doc! {"timestamp": -1})
                .limit(1)
                .build();

            let find_result = col
                .find(None, find_options)
                .await
                .unwrap()
                .deserialize_current()
                .unwrap();
            return (StatusCode::OK, Ok(Json(find_result)));
        }
        Err(e) => {
            // Failed connecting to DB
            println!("{}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Err("Failed connecting to DB"),
            );
        }
    }
}
