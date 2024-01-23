use std::io::Cursor;

use rocket::{http::{ContentType, Status}, response::Responder, Response};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

#[derive(Error, Debug, Responder)]
pub enum ApiError {
    #[error("unknown error")]
    #[response(status = 500, content_type = "json")]
    Unknown(Error),
}

#[derive(Serialize, ToSchema, Debug)]
pub struct Error {
    #[schema(example = "The leaderboard was none")]
    message: String,
    code: u16,
}

impl<'a> Responder<'a, 'a> for Error {
    fn respond_to(self, _: &'a rocket::Request) -> rocket::response::Result<'a> {
        let body = serde_json::to_string(&self)
            .expect("failed to serialize response to json");

        Ok(Response::build()
            .header(ContentType::JSON)
            .status(Status::new(self.code))
            .sized_body(body.len(), Cursor::new(body))
            .finalize())
    }
}

#[derive(Serialize, ToSchema)]
// A response that gets sent when an Error occurs.
pub struct ErrorResponse {
    #[schema(example = 500)]
    status: u16,
    error: Error,
}

impl<'a> Responder<'a, 'a> for ErrorResponse {
    fn respond_to(self, _: &'a rocket::Request) -> rocket::response::Result<'a> {

        let body = serde_json::to_string(&self)
            .expect("failed to serialize response to json");

        Ok(Response::build()
            .header(ContentType::JSON)
            .status(Status::new(self.status))
            .sized_body(body.len(), Cursor::new(body))
            .finalize())
    }
}

impl From<ApiError> for ErrorResponse {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::Unknown(e) => Self {
                status: Status::InternalServerError.code,
                error: e
            },
        }
    }
}