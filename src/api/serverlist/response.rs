use std::io::Cursor;

use battlebit_api::ServerData;
use rocket::{http::{ContentType, Status}, response::Responder, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ServerListResponse {
    #[schema(example = 200)]
    /// HTTP Response code. This is here for bad HTTP libraries.
    status: u16,
    data: Vec<ServerData>,
    /// Unix timestamp when the data was fetched.
    #[schema(example = 1706010047)]
    stamp: u64,
}

impl ServerListResponse {
    pub fn new(data: Vec<ServerData>, stamp: u64) -> Self {
        Self {
            status: Status::Ok.code,
            data: data.clone(),
            stamp
        }
    }
}

impl<'a> Responder<'a, 'a> for ServerListResponse {
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