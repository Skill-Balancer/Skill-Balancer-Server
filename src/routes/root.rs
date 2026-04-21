use crate::AppState;
use askama::Template;
use axum::{
    Router,
    extract::State,
    response::{Html, IntoResponse},
    routing::get,
};

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct Dashboard {
    pub config: String,
}
pub fn get_root() -> Router<AppState> {
    Router::new().route("/", get(render_dashboard))
}

async fn render_dashboard(State(state): State<AppState>) -> impl IntoResponse {
    match state.profile.lock().await.as_ref() {
        Some(profile) => Html(
            Dashboard {
                config: profile.name.clone(),
            }
            .render()
            .unwrap(),
        ),
        None => Html(
            Dashboard {
                config: "No Profile".to_string(),
            }
            .render()
            .unwrap(),
        ),
    }
}
