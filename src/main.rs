use axum::{
    extract:: {Path},
    routing::{get},
    Router,
};
use crate::controllers::character::get_characters;

mod controllers;
mod models;

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new()
        .route("/character/:character_name", get(character_info))
        .route("/characters", get(get_characters));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn character_info(Path(character_name): Path<String>) -> String {
    return character_name;
}
