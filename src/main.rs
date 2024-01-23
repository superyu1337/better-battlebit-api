use std::{sync::Arc, time::UNIX_EPOCH};

use api::{leaderboards, serverlist};
use battlebit_api::{BBApi, Leaderboard, ServerData};
use rocket::{response::content::RawHtml, tokio::{sync::RwLock, time::sleep}};


use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_swagger_ui::SwaggerUi;

#[macro_use] extern crate rocket;

mod api;
//mod error;

#[derive(Clone)]
struct BBData {
    api_client: BBApi,
    leaderboard: Option<Leaderboard>,
    server_list: Option<Vec<ServerData>>,
    leaderboard_stamp: u64,
    server_list_stamp: u64,
}

type BBDataPointer = Arc<RwLock<BBData>>;

impl BBData {
    pub fn leaderboard(&self) -> Option<&Leaderboard> {
        self.leaderboard.as_ref()
    }

    pub fn server_list(&self) -> Option<Vec<ServerData>> {
        self.server_list.as_ref().cloned()
    }

    fn get_stamp(&self) -> u64 {
        let current_time = std::time::SystemTime::now();
        let duration_since_epoch = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        duration_since_epoch.as_secs()
    }

    pub async fn update(&mut self) -> Result<(), battlebit_api::Error> {
        if let Ok(new_lb) = self.api_client.leaderboard().await {
            self.leaderboard = Some(new_lb);
            self.leaderboard_stamp = self.get_stamp();
        } else {
            // Todo: Logging the error
        }

        if let Ok(new_sl) = self.api_client.server_list().await {
            self.server_list = Some(new_sl);
            self.leaderboard_stamp = self.get_stamp();
        } else {
            // Todo: Logging the error
        }

        Ok(())
    }
}

async fn fetch_api_data(bbdata: BBDataPointer) {
    loop {
        let mut data = bbdata.write().await;

        if let Err(e) = data.update().await {
            println!("Error fetching new data: {}", e);
        }

        drop(data);
        sleep(rocket::tokio::time::Duration::from_secs(60)).await;
    }
}

#[derive(OpenApi)]
#[openapi(
    info(license(
        name = "DBAD",
        url = "https://dbad-license.org/"
    )),
    paths(
        leaderboards::kills,
        leaderboards::heal,
        leaderboards::longest_kills,
        leaderboards::objectives_complete,
        leaderboards::revives,
        leaderboards::roadkills,
        leaderboards::vehicle_repairs,
        leaderboards::vehicles_destroyed,
        leaderboards::xp,
        leaderboards::clans,

        serverlist::serverlist
    ),
    components(
        schemas(
            //error::ErrorResponse,
            leaderboards::responses::PlayerLeaderboardResponse,
            leaderboards::responses::ClanLeaderboardResponse,
            serverlist::response::ServerListResponse,

            battlebit_api::Player,
            battlebit_api::Clan,
            battlebit_api::ServerData,
            battlebit_api::AntiCheat,
            battlebit_api::DayNight,
            battlebit_api::Gamemode,
            battlebit_api::MapSize,
            battlebit_api::Region,
        )
    ),
)]
struct ApiDoc;

#[get("/")]
pub async fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("static/index.html"))
}

#[get("/favicon.ico")]
pub async fn favicon() -> Vec<u8> {
    include_bytes!("static/favicon.ico").to_vec()
}

#[rocket::main]
async fn main() {
    let bbdata = Arc::new(RwLock::new(BBData {
        api_client: BBApi::new(),
        leaderboard: None,
        server_list: None,
        leaderboard_stamp: 0,
        server_list_stamp: 0,
    }));

    let rocket = rocket::build()
        .manage(bbdata.clone())
        .mount(
            "/",
            SwaggerUi::new("/swagger/<_..>").url("/docs/openapi.json", ApiDoc::openapi()),
        )
        .mount("/", RapiDoc::new("/docs/openapi.json").path("/rapidoc"))
        .mount("/", Redoc::with_url("/redoc", ApiDoc::openapi()))
        .manage(ApiDoc::openapi())
        .mount("/", routes![index, favicon])
        .mount("/api", routes![
            serverlist::serverlist
        ])
        .mount("/api/leaderboards", routes![
            leaderboards::kills,
            leaderboards::heal,
            leaderboards::longest_kills,
            leaderboards::objectives_complete,
            leaderboards::revives,
            leaderboards::roadkills,
            leaderboards::vehicle_repairs,
            leaderboards::vehicles_destroyed,
            leaderboards::xp,
            leaderboards::clans
        ]);

    rocket::tokio::spawn(async move { fetch_api_data(bbdata).await });
    rocket.launch().await.expect("Rocket encountered an error!");
}