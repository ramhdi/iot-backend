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
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

// sample function, return dummy data
pub async fn get_dummy_data() -> impl IntoResponse {
    println!("get_dummy_data");
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

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

    return (StatusCode::OK, Json(dummy_data));
}

// get data by object id
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
                    // Valid ID
                    let filter = doc! {"_id": id};
                    let find_result = col.find_one(filter, None).await;
                    match find_result {
                        Ok(None) => return (StatusCode::NOT_FOUND, Err(String::from("Not found"))),
                        Ok(Some(data)) => return (StatusCode::OK, Ok(Json(data))),
                        Err(e) => {
                            println!("{}", e.to_string());
                            return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string()));
                        }
                    }
                }
                Err(e) => {
                    // Caught invalid ID
                    println!("{}", e.to_string());
                    return (StatusCode::BAD_REQUEST, Err(e.to_string()));
                }
            }
        }
        Err(e) => {
            // Failed connecting to DB
            println!("{}", e.to_string());
            return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string()));
        }
    }
}

// get latest data
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
            println!("{}", e.to_string());
            return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string()));
        }
    }
}
