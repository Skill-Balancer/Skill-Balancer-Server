use axum::{Router, response::Html, routing::get};
use ormlite::sqlite::SqliteConnection;
use ormlite::{Connection, model::*};
use dotenv::dotenv;
use std::env;

#[derive(Model, Debug)]
#[ormlite(table = "Game")]
pub struct Game {
    #[ormlite(primary_key)]
    pub player_one_id: u32,
    pub player_two_id: u32,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db = env::var("DATABASE_URL").unwrap();
    let mut conn = SqliteConnection::connect(&db).await.unwrap();

    {
        let _game = (Game {
            player_one_id: 1,
            player_two_id: 2,
        })
        .insert(&mut conn)
        .await;
    }

    let used = Game::select().fetch_all(&mut conn).await.unwrap();

    let app = Router::new().route(
        "/",
        get(Html(
            "
            <h1>Hello!</h1>
            <p>Hi from Rust</p>
            ",
        )),
    );
    println!("Player one: {:?}", used);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
