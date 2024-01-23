
use rocket::State;

use crate::BBDataPointer;

use self::responses::{ClanLeaderboardResponse, PlayerLeaderboardResponse};

pub mod responses;

/// Top clans
/// 
/// Gets a leaderboard with the clans that have the most experience.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Returns a ClanLeaderboardResponse", body = ClanLeaderboardResponse),
    )
)]
#[get("/clans")]
pub async fn clans(state: &State<BBDataPointer>) -> ClanLeaderboardResponse {
    let bbdata = state.read().await;

    if let Some(lb) = bbdata.leaderboard() {
        ClanLeaderboardResponse::new(lb.top_clans().clone(), bbdata.leaderboard_stamp)
    } else {
        ClanLeaderboardResponse::new(Vec::new(), bbdata.leaderboard_stamp)
    }
}

/// Most kills
/// 
/// Gets a leaderboard with the players that have the most kills.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Returns a PlayerLeaderboardResponse", body = PlayerLeaderboardResponse),
    )
)]
#[get("/kills")]
pub async fn kills(state: &State<BBDataPointer>) -> PlayerLeaderboardResponse {
    let bbdata = state.read().await;

    if let Some(lb) = bbdata.leaderboard() {
        PlayerLeaderboardResponse::new(lb.most_kills().clone(), bbdata.leaderboard_stamp)
    } else {
        PlayerLeaderboardResponse::new(Vec::new(),  bbdata.leaderboard_stamp)
    }
}

/// Most heal
/// 
/// Gets a leaderboard with the players that have healed the most.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Returns a PlayerLeaderboardResponse", body = PlayerLeaderboardResponse),
    )
)]
#[get("/heal")]
pub async fn heal(state: &State<BBDataPointer>) -> PlayerLeaderboardResponse {
    let bbdata = state.read().await;

    if let Some(lb) = bbdata.leaderboard() {
        PlayerLeaderboardResponse::new(lb.most_heals().clone(),  bbdata.leaderboard_stamp)
    } else {
        PlayerLeaderboardResponse::new(Vec::new(),  bbdata.leaderboard_stamp)
    }
}

/// Longest kill
/// 
/// Gets a leaderboard with the players that have longest kill.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Returns a PlayerLeaderboardResponse", body = PlayerLeaderboardResponse),
    )
)]
#[get("/longest_kills")]
pub async fn longest_kills(state: &State<BBDataPointer>) -> PlayerLeaderboardResponse {
    let bbdata = state.read().await;

    if let Some(lb) = bbdata.leaderboard() {
        PlayerLeaderboardResponse::new(lb.longest_kills().clone(),  bbdata.leaderboard_stamp)
    } else {
        PlayerLeaderboardResponse::new(Vec::new(),  bbdata.leaderboard_stamp)
    }
}

/// Most objectives completed
/// 
/// Gets a leaderboard with the players that have the most revives.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Returns a PlayerLeaderboardResponse", body = PlayerLeaderboardResponse),
    )
)]
#[get("/objectives_complete")]
pub async fn objectives_complete(state: &State<BBDataPointer>) -> PlayerLeaderboardResponse {
    let bbdata = state.read().await;

    if let Some(lb) = bbdata.leaderboard() {
        PlayerLeaderboardResponse::new(lb.most_objectives_complete().clone(),  bbdata.leaderboard_stamp)
    } else {
        PlayerLeaderboardResponse::new(Vec::new(),  bbdata.leaderboard_stamp)
    }
}

/// Most revives
/// 
/// Gets a leaderboard with the players that have the most revives.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Returns a PlayerLeaderboardResponse", body = PlayerLeaderboardResponse),
    )
)]
#[get("/revives")]
pub async fn revives(state: &State<BBDataPointer>) -> PlayerLeaderboardResponse{
    let bbdata = state.read().await;

    if let Some(lb) = bbdata.leaderboard() {
        PlayerLeaderboardResponse::new(lb.most_revives().clone(),  bbdata.leaderboard_stamp)
    } else {
        PlayerLeaderboardResponse::new(Vec::new(),  bbdata.leaderboard_stamp)
    }
}

/// Most roadkills
/// 
/// Gets a leaderboard with the players that have the most roadkills.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Returns a PlayerLeaderboardResponse", body = PlayerLeaderboardResponse),
    )
)]
#[get("/roadkills")]
pub async fn roadkills(state: &State<BBDataPointer>) -> PlayerLeaderboardResponse {
    let bbdata = state.read().await;

    if let Some(lb) = bbdata.leaderboard() {
        PlayerLeaderboardResponse::new(lb.most_roadkills().clone(),  bbdata.leaderboard_stamp)
    } else {
        PlayerLeaderboardResponse::new(Vec::new(),  bbdata.leaderboard_stamp)
    }
}

/// Most vehicle repairs
/// 
/// Gets a leaderboard with the players that have repaired the most vehicles.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Returns a PlayerLeaderboardResponse", body = PlayerLeaderboardResponse),
    )
)]
#[get("/vehicle_repairs")]
pub async fn vehicle_repairs(state: &State<BBDataPointer>) -> PlayerLeaderboardResponse {
    let bbdata = state.read().await;

    if let Some(lb) = bbdata.leaderboard() {
        PlayerLeaderboardResponse::new(lb.most_vehicle_repairs().clone(),  bbdata.leaderboard_stamp)
    } else {
        PlayerLeaderboardResponse::new(Vec::new(),  bbdata.leaderboard_stamp)
    }
}

/// Most vehicles destroyed
/// 
/// Gets a leaderboard with the players that have the most vehicles destroyed.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Returns a PlayerLeaderboardResponse", body = PlayerLeaderboardResponse),
    )
)]
#[get("/vehicles_destroyed")]
pub async fn vehicles_destroyed(state: &State<BBDataPointer>) -> PlayerLeaderboardResponse {
    let bbdata = state.read().await;

    if let Some(lb) = bbdata.leaderboard() {
        PlayerLeaderboardResponse::new(lb.most_vehicles_destroyed().clone(),  bbdata.leaderboard_stamp)
    } else {
        PlayerLeaderboardResponse::new(Vec::new(),  bbdata.leaderboard_stamp)
    }
}

/// Most experience
/// 
/// Gets a leaderboard with the players that have the most experience.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Returns a PlayerLeaderboardResponse", body = PlayerLeaderboardResponse),
    )
)]
#[get("/xp")]
pub async fn xp(state: &State<BBDataPointer>) -> PlayerLeaderboardResponse {
    let bbdata = state.read().await;

    if let Some(lb) = bbdata.leaderboard() {
        PlayerLeaderboardResponse::new(lb.most_xp().clone(),  bbdata.leaderboard_stamp)
    } else {
        PlayerLeaderboardResponse::new(Vec::new(),  bbdata.leaderboard_stamp)
    }
}