// Custom error type

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum APIError {
    MongoDBError(mongodb::error::Error),
    SystemTimeError(std::time::SystemTimeError),
    OIDError(mongodb::bson::oid::Error),
}

impl From<mongodb::error::Error> for APIError {
    fn from(e: mongodb::error::Error) -> Self {
        return APIError::MongoDBError(e);
    }
}

impl From<std::time::SystemTimeError> for APIError {
    fn from(e: std::time::SystemTimeError) -> Self {
        return APIError::SystemTimeError(e);
    }
}

impl From<mongodb::bson::oid::Error> for APIError {
    fn from(e: mongodb::bson::oid::Error) -> Self {
        return APIError::OIDError(e);
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            APIError::MongoDBError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            APIError::SystemTimeError(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            APIError::OIDError(e) => (StatusCode::BAD_REQUEST, e.to_string()),
        };

        return (status, error_message).into_response();
    }
}
