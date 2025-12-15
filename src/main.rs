mod config;
mod dto;
mod handlers;
mod state;
mod validated_query;

use std::env::var;

use axum::{Router, routing::get};
use config::*;
use handlers::*;

use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config_path = var("CONFIG_PATH").expect("you should pass the CONFIG_PATH variable");
    let config = load_config(&config_path).expect("cannot load the config");

    env_logger::init();

    let state = AppState {
        config: config.clone(),
    };

    let app = Router::new()
        .route("/fetch", get(download_route))
        .with_state(state);

    log::info!("running server at {}", config.server_url);

    let listener = tokio::net::TcpListener::bind(&config.server_url)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .expect("axum::serve failed");

    Ok(())
}
