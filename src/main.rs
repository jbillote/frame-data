use axum::{
    extract:: {Path},
    routing::{get},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/character/:character_name", get(character_info));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn character_info(Path(character_name): Path<String>) -> String {
    return character_name;
}
