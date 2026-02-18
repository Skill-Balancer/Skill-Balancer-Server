use ormlite::sqlite::SqliteConnection;
use ormlite::{Connection, Model};

use axum::Router;
use dotenv::dotenv;
use std::env;

//importing routes and files.
mod config;
mod routes;

// importing models
mod models;
use crate::models::game::Game;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = env::var("DATABASE_URL").unwrap();
    let mut conn = SqliteConnection::connect(&db).await.unwrap();

    {
        let _game = Game {
            player_one_id: 1,
            player_two_id: 2,
        }
        .insert(&mut conn)
        .await;
    }

    let game = Game::select().fetch_all(&mut conn).await.unwrap();

    let app = Router::new().merge(routes::root::get_root());

    println!("Player one: {:?}", game);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
