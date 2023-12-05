use crate::controllers::attack::{get_attack, get_attacks};
use crate::controllers::character::get_characters;
use axum::{extract::OriginalUri, http::Request, routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::info_span;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod models;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "frame_data=info,tower_http=info,axum::rejection=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/character/:character_name", get(get_attacks))
        .route("/character/:character_name/:input", get(get_attack))
        .route("/characters", get(get_characters))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                let path = if let Some(path) = request.extensions().get::<OriginalUri>() {
                    path.0.path().to_owned()
                } else {
                    request.uri().path().to_owned()
                };

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    path,
                )
            }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
