use crate::storage::db::DB;
use axum::Router;
use dotenv::dotenv;
use network::profile::Profile;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::services::ServeDir;

//importing routes and files.
mod config;
mod routes;
// importing models
mod entities;
mod env;
mod models;
mod network;
mod storage;
#[cfg(test)]
mod tests;

#[derive(Clone)]
struct AppState {
    profile: Arc<Mutex<Option<Profile>>>,
    db: DB,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = DB::new().await.expect("Failed to connect to database");
    db.sync_schema()
        .await
        .expect("Failed to synchronize database schema");

    let state = AppState {
        profile: Arc::new(Mutex::new(None)),
        db: db.clone(),
    };

    let app = Router::new()
        .merge(routes::root::get_root())
        .nest_service("/models", ServeDir::new("data"))
        .merge(routes::config_route::config_route())
        .merge(routes::step_route::step_route())
        .merge(routes::all_config_route::all_config_route())
        .merge(routes::model::checkpoint::checkpoint_route())
        .merge(routes::model::load::load_model_route())
        .merge(routes::model::export::export_model_route())
        .merge(routes::model::list_checkpoints::list_checkpoints_route())
        .merge(routes::model::list_exports::list_exports_route())
        .with_state(state);

    println!("Server running on http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    db.close().await.expect("Failed to close database");
}
