use axum::{
    Router,
    extract::State,
    response::{Sse, sse::Event},
    routing::get,
};
use futures::stream::Stream;
use std::convert::Infallible;
use tokio_stream::{StreamExt, wrappers::BroadcastStream};

use crate::AppState;

pub fn metrics_route() -> Router<AppState> {
    Router::new().route("/metrics", get(metrics_sse))
}

async fn metrics_sse(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.metrics_tx.subscribe();

    let stream = BroadcastStream::new(rx).filter_map(|msg| match msg {
        Ok(metrics) => {
            let json = serde_json::to_string(&metrics).unwrap();
            Some(Ok(Event::default().event("metrics").data(json)))
        }
        Err(_) => None,
    });

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}

pub fn config_route() -> Router<AppState> {
    Router::new().route("/config", get(config_sse))
}

async fn config_sse(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.config_tx.subscribe();

    let stream = BroadcastStream::new(rx).filter_map(|msg| match msg {
        Ok(config) => {
            let json = serde_json::to_string(&config).unwrap();
            Some(Ok(Event::default().event("config").data(json)))
        }
        Err(_) => None,
    });

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::default())
}
