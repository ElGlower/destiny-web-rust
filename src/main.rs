use axum::{
    routing::get,
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use yup_oauth2::{read_service_account_key, ServiceAccountAuthenticator};

mod state;
mod firebase;
mod handlers;
mod components;

use state::AppState;

#[tokio::main]
async fn main() {
    // 0. Si existe la variable de entorno FIREBASE_KEY_JSON, escribirla en firebase-key.json
    if let Ok(key_content) = std::env::var("FIREBASE_KEY_JSON") {
        if let Err(e) = std::fs::write("firebase-key.json", key_content) {
            eprintln!("Error al escribir firebase-key.json desde variable de entorno: {}", e);
        } else {
            println!("firebase-key.json escrito correctamente desde la variable de entorno.");
        }
    }

    // 1. Cargar las credenciales de Firebase
    let secret = match read_service_account_key("firebase-key.json").await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error crítico: No se pudo leer 'firebase-key.json': {}", e);
            return;
        }
    };

    let project_id = secret.project_id.clone().unwrap_or_default();
    println!("Inicializando servidor de Destiny con el proyecto Firebase: {}", project_id);

    // 2. Autenticador de Google
    let auth = match ServiceAccountAuthenticator::builder(secret).build().await {
        Ok(a) => Arc::new(a),
        Err(e) => {
            eprintln!("Error al crear el autenticador: {}", e);
            return;
        }
    };

    let state = AppState { auth, project_id };

    // 3. Crear router de Axum
    let app = Router::new()
        // SSR Pages
        .route("/", get(handlers::home_page))
        .route("/leaderboard", get(handlers::leaderboard_page))
        .route("/profile/:uuid", get(handlers::profile_page))
        .route("/staff", get(handlers::staff_page))
        .route("/icon.png", get(handlers::serve_icon))
        
        // API REST
        .route("/api/leaderboard", get(handlers::get_leaderboard))
        .route("/api/player/:uuid", get(handlers::get_player_stats))
        .route("/api/status", get(handlers::get_live_status))
        .fallback_service(ServeDir::new("static").append_index_html_on_directories(true))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // 4. Iniciar servidor
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("Servidor Web de Destiny iniciado en http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
