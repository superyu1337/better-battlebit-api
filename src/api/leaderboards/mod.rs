use rocket::State;

use crate::{error::{ApiError, ErrorResponse}, BBDataPointer};

use self::responses::{ClanLeaderboard, PlayerLeaderboard};

pub mod responses;

/// Top clans
/// 
/// Gets a leaderboard with the clans that have the most experience.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Everything went well. Returns a ClanLeaderboard", body = ClanLeaderboard),
        (status = 500, description = "Internal Server Error.", body = ErrorResponse)
    )
)]
#[get("/clans")]
pub async fn clans(state: &State<BBDataPointer>) -> Result<ClanLeaderboard, ErrorResponse> {
    let bbdata = state.read().await;
    let leaderboard = bbdata.leaderboard().ok_or(
        ApiError::Unknown(String::from("The leaderboard was nonexistent"))
    )?;

    let clans = leaderboard.top_clans();

    if clans.is_empty() {
        Err(ErrorResponse::from(ApiError::Unknown(String::from("The leaderboard was empty"))))
    } else {
        Ok(ClanLeaderboard::new(clans.clone()))
    }
}

/// Most kills
/// 
/// Gets a leaderboard with the players that have the most kills.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Everything went well. Returns a PlayerLeaderboard", body = PlayerLeaderboard),
        (status = 500, description = "Internal Server Error.", body = ErrorResponse)
    )
)]
#[get("/kills")]
pub async fn kills(state: &State<BBDataPointer>) -> Result<PlayerLeaderboard, ErrorResponse> {
    let bbdata = state.read().await;
    let leaderboard = bbdata.leaderboard().ok_or(
        ApiError::Unknown(String::from("The leaderboard was nonexistent"))
    )?;

    let players = leaderboard.most_kills();

    if players.is_empty() {
        Err(ErrorResponse::from(ApiError::Unknown(String::from("The leaderboard was empty"))))
    } else {
        Ok(PlayerLeaderboard::new(players.clone()))
    }
}

/// Most heal
/// 
/// Gets a leaderboard with the players that have healed the most.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Everything went well. Returns a PlayerLeaderboard", body = PlayerLeaderboard),
        (status = 500, description = "Internal Server Error.", body = ErrorResponse)
    )
)]
#[get("/heal")]
pub async fn heal(state: &State<BBDataPointer>) -> Result<PlayerLeaderboard, ErrorResponse> {
    let bbdata = state.read().await;
    let leaderboard = bbdata.leaderboard().ok_or(
        ApiError::Unknown(String::from("The leaderboard was nonexistent"))
    )?;

    let players = leaderboard.most_heals();

    if players.is_empty() {
        Err(ErrorResponse::from(ApiError::Unknown(String::from("The leaderboard was empty"))))
    } else {
        Ok(PlayerLeaderboard::new(players.clone()))
    }
}

/// Longest kill
/// 
/// Gets a leaderboard with the players that have longest kill.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Everything went well. Returns a PlayerLeaderboard", body = PlayerLeaderboard),
        (status = 500, description = "Internal Server Error.", body = ErrorResponse)
    )
)]
#[get("/longest_kills")]
pub async fn longest_kills(state: &State<BBDataPointer>) -> Result<PlayerLeaderboard, ErrorResponse> {
    let bbdata = state.read().await;
    let leaderboard = bbdata.leaderboard().ok_or(
        ApiError::Unknown(String::from("The leaderboard was nonexistent"))
    )?;

    let players = leaderboard.longest_kills();

    if players.is_empty() {
        Err(ErrorResponse::from(ApiError::Unknown(String::from("The leaderboard was empty"))))
    } else {
        Ok(PlayerLeaderboard::new(players.clone()))
    }
}

/// Most objectives completed
/// 
/// Gets a leaderboard with the players that have the most revives.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Everything went well. Returns a PlayerLeaderboard", body = PlayerLeaderboard),
        (status = 500, description = "Internal Server Error.", body = ErrorResponse)
    )
)]
#[get("/objectives_complete")]
pub async fn objectives_complete(state: &State<BBDataPointer>) -> Result<PlayerLeaderboard, ErrorResponse> {
    let bbdata = state.read().await;
    let leaderboard = bbdata.leaderboard().ok_or(
        ApiError::Unknown(String::from("The leaderboard was nonexistent"))
    )?;

    let players = leaderboard.most_objectives_complete();

    if players.is_empty() {
        Err(ErrorResponse::from(ApiError::Unknown(String::from("The leaderboard was empty"))))
    } else {
        Ok(PlayerLeaderboard::new(players.clone()))
    }
}

/// Most revives
/// 
/// Gets a leaderboard with the players that have the most revives.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Everything went well. Returns a PlayerLeaderboard", body = PlayerLeaderboard),
        (status = 500, description = "Internal Server Error.", body = ErrorResponse)
    )
)]
#[get("/revives")]
pub async fn revives(state: &State<BBDataPointer>) -> Result<PlayerLeaderboard, ErrorResponse> {
    let bbdata = state.read().await;
    let leaderboard = bbdata.leaderboard().ok_or(
        ApiError::Unknown(String::from("The leaderboard was nonexistent"))
    )?;

    let players = leaderboard.most_revives();

    if players.is_empty() {
        Err(ErrorResponse::from(ApiError::Unknown(String::from("The leaderboard was empty"))))
    } else {
        Ok(PlayerLeaderboard::new(players.clone()))
    }
}

/// Most roadkills
/// 
/// Gets a leaderboard with the players that have the most roadkills.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Everything went well. Returns a PlayerLeaderboard", body = PlayerLeaderboard),
        (status = 500, description = "Internal Server Error.", body = ErrorResponse)
    )
)]
#[get("/roadkills")]
pub async fn roadkills(state: &State<BBDataPointer>) -> Result<PlayerLeaderboard, ErrorResponse> {
    let bbdata = state.read().await;
    let leaderboard = bbdata.leaderboard().ok_or(
        ApiError::Unknown(String::from("The leaderboard was nonexistent"))
    )?;

    let players = leaderboard.most_roadkills();

    if players.is_empty() {
        Err(ErrorResponse::from(ApiError::Unknown(String::from("The leaderboard was empty"))))
    } else {
        Ok(PlayerLeaderboard::new(players.clone()))
    }
}

/// Most vehicle repairs
/// 
/// Gets a leaderboard with the players that have repaired the most vehicles.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Everything went well. Returns a PlayerLeaderboard", body = PlayerLeaderboard),
        (status = 500, description = "Internal Server Error.", body = ErrorResponse)
    )
)]
#[get("/vehicle_repairs")]
pub async fn vehicle_repairs(state: &State<BBDataPointer>) -> Result<PlayerLeaderboard, ErrorResponse> {
    let bbdata = state.read().await;
    let leaderboard = bbdata.leaderboard().ok_or(
        ApiError::Unknown(String::from("The leaderboard was nonexistent"))
    )?;

    let players = leaderboard.most_vehicle_repairs();

    if players.is_empty() {
        Err(ErrorResponse::from(ApiError::Unknown(String::from("The leaderboard was empty"))))
    } else {
        Ok(PlayerLeaderboard::new(players.clone()))
    }
}

/// Most vehicles destroyed
/// 
/// Gets a leaderboard with the players that have the most vehicles destroyed.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Everything went well. Returns a PlayerLeaderboard", body = PlayerLeaderboard),
        (status = 500, description = "Internal Server Error.", body = ErrorResponse)
    )
)]
#[get("/vehicles_destroyed")]
pub async fn vehicles_destroyed(state: &State<BBDataPointer>) -> Result<PlayerLeaderboard, ErrorResponse> {
    let bbdata = state.read().await;
    let leaderboard = bbdata.leaderboard().ok_or(
        ApiError::Unknown(String::from("The leaderboard was nonexistent"))
    )?;

    let players = leaderboard.most_vehicles_destroyed();

    if players.is_empty() {
        Err(ErrorResponse::from(ApiError::Unknown(String::from("The leaderboard was empty"))))
    } else {
        Ok(PlayerLeaderboard::new(players.clone()))
    }
}

/// Most experience
/// 
/// Gets a leaderboard with the players that have the most experience.
#[utoipa::path(
    context_path = "/api/leaderboards",
    responses(
        (status = 200, description = "Everything went well. Returns a PlayerLeaderboard", body = PlayerLeaderboard),
        (status = 500, description = "Internal Server Error.", body = ErrorResponse)
    )
)]
#[get("/xp")]
pub async fn xp(state: &State<BBDataPointer>) -> Result<PlayerLeaderboard, ErrorResponse> {
    let bbdata = state.read().await;
    let leaderboard = bbdata.leaderboard().ok_or(
        ApiError::Unknown(String::from("The leaderboard was nonexistent"))
    )?;

    let players = leaderboard.most_xp();

    if players.is_empty() {
        Err(ErrorResponse::from(ApiError::Unknown(String::from("The leaderboard was empty"))))
    } else {
        Ok(PlayerLeaderboard::new(players.clone()))
    }
}