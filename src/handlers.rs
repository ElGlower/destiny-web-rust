use axum::{
    extract::{Path, State},
    http::{StatusCode, header},
    response::{Html, IntoResponse},
    Json,
};
use serde_json::Value;
use crate::state::AppState;
use crate::firebase;
use crate::components;

// --- API REST Endpoints ---

pub async fn get_leaderboard(
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let data = firebase::fetch_leaderboard(&state).await?;
    Ok(Json(data))
}

pub async fn get_player_stats(
    Path(uuid): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let data = firebase::fetch_player_stats(&state, &uuid).await?;
    Ok(Json(data))
}

pub async fn get_live_status(
    State(state): State<AppState>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let data = firebase::fetch_live_status(&state).await?;
    Ok(Json(data))
}

// --- Server-Side Rendered (SSR) Web Pages ---

pub async fn home_page() -> Html<String> {
    Html(components::layout::render_page("Inicio", "/", components::home::render()))
}

pub async fn staff_page() -> Html<String> {
    Html(components::layout::render_page("Staff", "/staff", components::staff::render()))
}

pub async fn leaderboard_page(
    State(state): State<AppState>,
) -> Result<Html<String>, (StatusCode, String)> {
    let data = firebase::fetch_leaderboard(&state).await?;
    let rendered = components::leaderboard::render(&data);
    Ok(Html(components::layout::render_page("Leaderboard", "/leaderboard", &rendered)))
}

pub async fn profile_page(
    Path(uuid): Path<String>,
    State(state): State<AppState>,
) -> Result<Html<String>, (StatusCode, String)> {
    let data = firebase::fetch_player_stats(&state, &uuid).await?;
    let username = data.get("fields")
        .and_then(|f| f.get("username"))
        .and_then(|u| u.get("stringValue"))
        .and_then(|u| u.as_str())
        .unwrap_or("Desconocido");
    let rendered = components::profile::render(username, &data);
    Ok(Html(components::layout::render_page(&format!("Perfil - {}", username), "/leaderboard", &rendered)))
}

pub async fn serve_icon() -> impl IntoResponse {
    let bytes = include_bytes!("icon.png");
    ([(header::CONTENT_TYPE, "image/png")], bytes)
}

pub async fn get_skin_proxy(
    Path(username): Path<String>,
) -> impl IntoResponse {
    let url = format!("https://mc-heads.net/skin/{}", username);
    match reqwest::get(&url).await {
        Ok(resp) => {
            if resp.status().is_success() {
                if let Ok(bytes) = resp.bytes().await {
                    let mut headers = axum::http::HeaderMap::new();
                    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("image/png"));
                    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, header::HeaderValue::from_static("*"));
                    return (StatusCode::OK, headers, bytes.to_vec()).into_response();
                }
            }
        }
        Err(e) => {
            eprintln!("Error al hacer proxy de skin para {}: {}", username, e);
        }
    }
    StatusCode::NOT_FOUND.into_response()
}

