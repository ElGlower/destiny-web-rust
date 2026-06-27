use axum::http::StatusCode;
use serde_json::{json, Value};
use crate::state::AppState;

const EXCLUDED_USERS: &[&str] = &[
    "cestart", "espiral_", "prismangames", "pilahd14",
    "pilah14", "elbalam15", "sombradr", "elglower",
];

async fn get_auth_token(state: &AppState) -> Result<String, (StatusCode, String)> {
    let scopes = &["https://www.googleapis.com/auth/datastore"];
    match state.auth.token(scopes).await {
        Ok(t) => Ok(t.token().unwrap_or("").to_string()),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error de autenticacion: {}", e))),
    }
}

fn extract_int(fields: &Value, key: &str) -> i64 {
    fields.get(key)
        .and_then(|v| v.get("integerValue"))
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0)
}

fn get_mode_stats(fields: &Value, mode: &str) -> (i64, i64, i64, i64, i64) {
    let mode_fields = fields.get("stats")
        .and_then(|s| s.get("mapValue"))
        .and_then(|m| m.get("fields"))
        .and_then(|f| f.get(mode))
        .and_then(|t| t.get("mapValue"))
        .and_then(|m| m.get("fields"));

    match mode_fields {
        Some(f) => (
            extract_int(f, "kills"),
            extract_int(f, "assists"),
            extract_int(f, "wins"),
            extract_int(f, "losses"),
            extract_int(f, "matches_played"),
        ),
        None => (0, 0, 0, 0, 0),
    }
}

async fn run_firestore_query(state: &AppState, query: Value) -> Result<Value, (StatusCode, String)> {
    let token = get_auth_token(state).await?;
    let url = format!(
        "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents:runQuery",
        state.project_id
    );
    let client = reqwest::Client::new();
    let response = client
        .post(&url)
        .bearer_auth(&token)
        .json(&query)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Error de red: {}", e)))?;

    if !response.status().is_success() {
        let status_code = response.status();
        let err_body = response.text().await.unwrap_or_default();
        eprintln!("Error Firestore query ({}): {}", status_code, err_body);
        return Err((
            StatusCode::from_u16(status_code.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            "Error en Firestore".to_string(),
        ));
    }

    response.json().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn fetch_leaderboard(state: &AppState) -> Result<Value, (StatusCode, String)> {
    let ttr_query = json!({
        "structuredQuery": {
            "from": [{"collectionId": "players"}],
            "orderBy": [{"field": {"fieldPath": "stats.ttr.kills"}, "direction": "DESCENDING"}],
            "limit": 50
        }
    });

    let skywars_query = json!({
        "structuredQuery": {
            "from": [{"collectionId": "players"}],
            "orderBy": [{"field": {"fieldPath": "stats.skywars.wins"}, "direction": "DESCENDING"}],
            "limit": 50
        }
    });

    let (ttr_raw, skywars_raw) = tokio::join!(
        run_firestore_query(state, ttr_query),
        run_firestore_query(state, skywars_query)
    );

    let parse_results = |raw: Result<Value, _>, mode: &str| -> Vec<Value> {
        let mut out = Vec::new();
        if let Ok(arr_val) = raw {
            if let Some(arr) = arr_val.as_array() {
                for item in arr {
                    if let Some(doc) = item.get("document") {
                        if let (Some(fields), Some(name)) = (doc.get("fields"), doc.get("name")) {
                            let username = fields.get("username")
                                .and_then(|u| u.get("stringValue"))
                                .and_then(|u| u.as_str())
                                .unwrap_or("Desconocido")
                                .to_string();

                            if EXCLUDED_USERS.contains(&username.to_lowercase().as_str()) {
                                continue;
                            }

                            let (kills, assists, wins, losses, played) = get_mode_stats(fields, mode);

                            out.push(json!({
                                "uuid": name.as_str().unwrap_or("").split('/').last().unwrap_or(""),
                                "username": username,
                                "kills": kills,
                                "assists": assists,
                                "wins": wins,
                                "losses": losses,
                                "played": played,
                                "mode": mode
                            }));
                        }
                    }
                }
            }
        }
        out
    };

    let ttr_list = parse_results(ttr_raw, "ttr");
    let skywars_list = parse_results(skywars_raw, "skywars");

    Ok(json!({
        "ttr": ttr_list,
        "skywars": skywars_list
    }))
}

pub async fn fetch_player_stats(state: &AppState, uuid: &str) -> Result<Value, (StatusCode, String)> {
    let token = get_auth_token(state).await?;
    let url = format!(
        "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents/players/{}",
        state.project_id, uuid
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .bearer_auth(&token)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Error al consultar Firestore: {}", e)))?;

    if !response.status().is_success() {
        let status_code = response.status();
        eprintln!("Error fetch_player_stats ({})", status_code);
        return Err((
            StatusCode::from_u16(status_code.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            "Jugador no encontrado".to_string(),
        ));
    }

    response.json().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn fetch_live_status(state: &AppState) -> Result<Value, (StatusCode, String)> {
    let token = get_auth_token(state).await?;
    let url = format!(
        "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents/servers/ttr",
        state.project_id
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .bearer_auth(&token)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Error Firestore: {}", e)))?;

    if !response.status().is_success() {
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(json!({"status": "DESCONECTADO", "active_event": "Ninguno", "online_players": 0}));
        }
        return Err((StatusCode::BAD_GATEWAY, "Error Firestore status".to_string()));
    }

    let doc: Value = response.json().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    let fields = doc.get("fields");
    let status = fields.and_then(|f| f.get("status")).and_then(|v| v.get("stringValue")).and_then(|s| s.as_str()).unwrap_or("DESCONECTADO").to_string();
    let active_event = fields.and_then(|f| f.get("active_event")).and_then(|v| v.get("stringValue")).and_then(|s| s.as_str()).unwrap_or("Ninguno").to_string();
    let online_players = fields.and_then(|f| f.get("online_players")).and_then(|v| v.get("integerValue")).and_then(|v| v.as_str()).and_then(|v| v.parse::<i32>().ok()).unwrap_or(0);

    Ok(json!({"status": status, "active_event": active_event, "online_players": online_players}))
}
