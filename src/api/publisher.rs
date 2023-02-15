use crate::{connector::connector::MongoDB, model::device::DeviceData};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use mongodb::Client;

// post data
pub async fn post_data(
    State(client): State<Client>,
    Json(device_data): Json<DeviceData>,
) -> impl IntoResponse {
    println!("post_data");
    let db_result = MongoDB::init_collection(client).await;
    match db_result {
        Ok(db) => {
            // Success connecting to DB
            let col = db.get_collection();
            let insert_one_result = col.insert_one(device_data, None).await;
            match insert_one_result {
                Ok(res) => {
                    return (
                        StatusCode::OK,
                        Ok(res.inserted_id.as_object_id().unwrap().to_string()),
                    )
                }
                Err(e) => {
                    println!("{}", e.to_string());
                    return (StatusCode::INTERNAL_SERVER_ERROR, Err(e.to_string()));
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
