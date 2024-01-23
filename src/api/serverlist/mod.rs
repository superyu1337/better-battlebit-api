pub mod response;

use response::ServerListResponse;
use rocket::State;

use crate::BBDataPointer;

/// Serverlist
/// 
/// Gets the serverlist.
#[utoipa::path(
    context_path = "/api",
    responses(
        (status = 200, description = "Returns a ServerListResponse", body = ServerListResponse),
    )
)]
#[get("/serverlist")]
pub async fn serverlist(state: &State<BBDataPointer>) -> ServerListResponse {
    let bbdata = state.read().await;

    if let Some(serverlist) = bbdata.server_list() {
        ServerListResponse::new(serverlist.clone(), bbdata.server_list_stamp)
    } else {
        ServerListResponse::new(Vec::new(), bbdata.server_list_stamp)
    }
}