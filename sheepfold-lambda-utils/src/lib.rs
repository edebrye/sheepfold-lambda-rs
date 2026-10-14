use std::error::Error;

use lambda_http::{
    Body, Response, aws_lambda_events::query_map::QueryMap, http::response::Builder,
};
use serde_json::json;
use sheepfold_model::Sheep;
use thiserror::Error;
pub use toasty::Db;
use toasty::stmt::Uuid;

#[derive(Error, Debug)]
pub enum FetchIDError {
    #[error("Missing parameter 'id'")]
    Missing,

    #[error("Parameter 'id' is not a proper uuid.")]
    NotUuid,
}

pub fn get_id_from_parameters(parameters: QueryMap) -> Result<Uuid, FetchIDError> {
    let id = parameters.first("id");

    if id.is_none() {
        return Err(FetchIDError::Missing);
    }

    let id = Uuid::parse_str(id.unwrap());

    if id.is_err() {
        return Err(FetchIDError::NotUuid);
    }

    Ok(id.unwrap())
}

pub fn base_response() -> Builder {
    Response::builder().header("content-type", "application/json")
}

pub fn err_to_response(err: impl Error, status: u16) -> Response<Body> {
    base_response()
        .status(status)
        .body(
            json!({"status": "error", "message": err.to_string()})
                .to_string()
                .into(),
        )
        .unwrap()
}

//https://refactoring.guru/design-patterns/singleton/rust/example#example-1
pub async fn get_db() -> Db {
    Db::builder()
        .models(toasty::models!(Sheep))
        .connect("dynamodb://")
        .await
        .expect("Could not create database connection.")
}
