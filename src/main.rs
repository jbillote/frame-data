use crate::controllers::attack::{get_attack, get_attacks};
use crate::controllers::character::get_characters;
use axum::{routing::get, Router};

mod controllers;
mod models;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let app = Router::new()
        .route("/character/:character_name", get(get_attacks))
        .route("/character/:character_name/:input", get(get_attack))
        .route("/characters", get(get_characters));

    Ok(app.into())
}
