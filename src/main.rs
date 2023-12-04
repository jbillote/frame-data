use axum::{
    routing::{get},
    Router,
};
use crate::controllers::character::get_characters;
use crate::controllers::attack::{ get_attack, get_attacks };

mod controllers;
mod models;

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new()
        .route("/character/:character_name", get(get_attacks))
        .route("/character/:character_name/:input", get(get_attack))
        .route("/characters", get(get_characters));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
