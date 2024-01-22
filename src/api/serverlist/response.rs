use std::io::Cursor;

use battlebit_api::ServerData;
use rocket::{http::{ContentType, Status}, response::Responder, Response};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ServerList {
    #[schema(example = 200)]
    status: u16,
    data: Vec<ServerData>,
}

impl ServerList {
    pub fn new(data: Vec<ServerData>) -> Self {
        Self {
            status: Status::Ok.code,
            data: data.clone(),
        }
    }
}

impl<'a> Responder<'a, 'a> for ServerList {
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