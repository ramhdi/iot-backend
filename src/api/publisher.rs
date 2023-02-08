use crate::{connector::connector::MongoDB, model::device::DeviceData};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use mongodb::{
    Client,
};

// post data
pub async fn post_data(State(client): State<Client>, Json(device_data): Json<DeviceData>) -> impl IntoResponse {
    println!("post_data");
    let col_result = MongoDB::init_collection(client).await;
    match col_result {
        Ok(c) => {
            // Success connecting to DB
            let col = c.get_collection();
            let insert_one_result = col.insert_one(device_data, None).await;
            match insert_one_result {
                Ok(_) => return (StatusCode::OK, Ok("Success")),
                Err(e) => {
                    println!("{}", e);
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Err("Failed inserting data to DB"),
                    );
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
