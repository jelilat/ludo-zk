use axum::{
    routing::{get, post},
    Router,
};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::routes::handlers::{initialize_game, play_game, GameStore};

pub async fn run_server() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // Create games store
    let games: GameStore = Arc::new(Mutex::new(HashMap::new()));

    // Build router
    let app = Router::new()
        .route("/init", post(initialize_game))
        .route("/play", post(play_game))
        .with_state(games);

    // Run server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3003")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3003");
    axum::serve(listener, app).await.unwrap();
}
