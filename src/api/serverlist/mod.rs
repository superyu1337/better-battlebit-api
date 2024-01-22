pub mod response;

use response::ServerList;
use rocket::State;

use crate::{error::{ApiError, ErrorResponse}, BBDataPointer};

/// Serverlist
/// 
/// Gets the serverlist.
#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Everything went well. Returns a ServerList", body = ServerList),
        (status = 500, description = "Internal Server Error.", body = ErrorResponse)
    )
)]
#[get("/serverlist")]
pub async fn serverlist(state: &State<BBDataPointer>) -> Result<ServerList, ErrorResponse> {
    let bbdata = state.read().await;
    let serverlist = bbdata.server_list().ok_or(
        ApiError::Unknown(String::from("The serverlist was nonexistent"))
    )?;

    if serverlist.is_empty() {
        Err(ErrorResponse::from(ApiError::Unknown(String::from("The serverlist was empty"))))
    } else {
        Ok(ServerList::new(serverlist.clone()))
    }
}