use axum::http::StatusCode;
use serde_json::{json, Value};
use crate::state::AppState;

async fn get_auth_token(state: &AppState) -> Result<String, (StatusCode, String)> {
    let scopes = &["https://www.googleapis.com/auth/datastore"];
    match state.auth.token(scopes).await {
        Ok(t) => Ok(t.token().unwrap_or("").to_string()),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, format!("Error de autenticacion: {}", e))),
    }
}

pub async fn fetch_leaderboard(state: &AppState) -> Result<Value, (StatusCode, String)> {
    let token = get_auth_token(state).await?;
    let url = format!(
        "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents:runQuery",
        state.project_id
    );

    let query = json!({
        "structuredQuery": {
            "from": [{"collectionId": "players"}],
            "orderBy": [{
                "field": {"fieldPath": "stats.ttr.kills"},
                "direction": "DESCENDING"
            }]
        }
    });

    let client = reqwest::Client::new();
    let response = match client
        .post(&url)
        .bearer_auth(&token)
        .json(&query)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err((StatusCode::BAD_GATEWAY, format!("Error de red con Firestore: {}", e))),
    };

    if !response.status().is_success() {
        let status_code = response.status();
        let err_body = response.text().await.unwrap_or_default();
        eprintln!("Error de Firestore en fetch_leaderboard ({}): {}", status_code, err_body);
        let status = StatusCode::from_u16(status_code.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        return Err((status, "Error al ejecutar consulta en Firestore".to_string()));
    }

    let raw_results: Value = response.json().await.unwrap();
    let mut leaderboard = Vec::new();

    if let Some(arr) = raw_results.as_array() {
        for item in arr {
            if let Some(doc) = item.get("document") {
                if let (Some(fields), Some(name)) = (doc.get("fields"), doc.get("name")) {
                    let username = fields.get("username").and_then(|u| u.get("stringValue")).and_then(|u| u.as_str()).unwrap_or("Desconocido").to_string();
                    
                    let kills = fields.get("stats")
                        .and_then(|s| s.get("mapValue"))
                        .and_then(|m| m.get("fields"))
                        .and_then(|f| f.get("ttr"))
                        .and_then(|t| t.get("mapValue"))
                        .and_then(|m| m.get("fields"))
                        .and_then(|f| f.get("kills"))
                        .and_then(|k| k.get("integerValue"))
                        .and_then(|k| k.as_str())
                        .and_then(|k| k.parse::<i32>().ok())
                        .unwrap_or(0);

                    let assists = fields.get("stats")
                        .and_then(|s| s.get("mapValue"))
                        .and_then(|m| m.get("fields"))
                        .and_then(|f| f.get("ttr"))
                        .and_then(|t| t.get("mapValue"))
                        .and_then(|m| m.get("fields"))
                        .and_then(|f| f.get("assists"))
                        .and_then(|a| a.get("integerValue"))
                        .and_then(|a| a.as_str())
                        .and_then(|a| a.parse::<i32>().ok())
                        .unwrap_or(0);

                    let wins = fields.get("stats")
                        .and_then(|s| s.get("mapValue"))
                        .and_then(|m| m.get("fields"))
                        .and_then(|f| f.get("ttr"))
                        .and_then(|t| t.get("mapValue"))
                        .and_then(|m| m.get("fields"))
                        .and_then(|f| f.get("wins"))
                        .and_then(|w| w.get("integerValue"))
                        .and_then(|w| w.as_str())
                        .and_then(|w| w.parse::<i32>().ok())
                        .unwrap_or(0);

                    let losses = fields.get("stats")
                        .and_then(|s| s.get("mapValue"))
                        .and_then(|m| m.get("fields"))
                        .and_then(|f| f.get("ttr"))
                        .and_then(|t| t.get("mapValue"))
                        .and_then(|m| m.get("fields"))
                        .and_then(|f| f.get("losses"))
                        .and_then(|l| l.get("integerValue"))
                        .and_then(|l| l.as_str())
                        .and_then(|l| l.parse::<i32>().ok())
                        .unwrap_or(0);

                    let played = fields.get("stats")
                        .and_then(|s| s.get("mapValue"))
                        .and_then(|m| m.get("fields"))
                        .and_then(|f| f.get("ttr"))
                        .and_then(|t| t.get("mapValue"))
                        .and_then(|m| m.get("fields"))
                        .and_then(|f| f.get("matches_played"))
                        .and_then(|p| p.get("integerValue"))
                        .and_then(|p| p.as_str())
                        .and_then(|p| p.parse::<i32>().ok())
                        .unwrap_or(0);

                    leaderboard.push(json!({
                        "uuid": name.as_str().unwrap_or("").split('/').last().unwrap_or(""),
                        "username": username,
                        "kills": kills,
                        "assists": assists,
                        "wins": wins,
                        "losses": losses,
                        "played": played
                    }));
                }
            }
        }
    }

    Ok(json!(leaderboard))
}

pub async fn fetch_player_stats(state: &AppState, uuid: &str) -> Result<Value, (StatusCode, String)> {
    let token = get_auth_token(state).await?;
    let url = format!(
        "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents/players/{}",
        state.project_id, uuid
    );

    let client = reqwest::Client::new();
    let response = match client
        .get(&url)
        .bearer_auth(&token)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err((StatusCode::BAD_GATEWAY, format!("Error al consultar Firestore: {}", e))),
    };

    if !response.status().is_success() {
        let status_code = response.status();
        let err_body = response.text().await.unwrap_or_default();
        eprintln!("Error de Firestore en fetch_player_stats ({}): {}", status_code, err_body);
        let status = StatusCode::from_u16(status_code.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        return Err((status, "Jugador no encontrado o error en Firestore".to_string()));
    }

    let firestore_doc: Value = response.json().await.unwrap();
    Ok(firestore_doc)
}

pub async fn fetch_live_status(state: &AppState) -> Result<Value, (StatusCode, String)> {
    let token = get_auth_token(state).await?;
    let url = format!(
        "https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents/servers/ttr",
        state.project_id
    );

    let client = reqwest::Client::new();
    let response = match client
        .get(&url)
        .bearer_auth(&token)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err((StatusCode::BAD_GATEWAY, format!("Error al consultar Firestore: {}", e))),
    };

    if !response.status().is_success() {
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(json!({
                "status": "DESCONECTADO",
                "active_event": "Ninguno",
                "online_players": 0
            }));
        }
        let status_code = response.status();
        let err_body = response.text().await.unwrap_or_default();
        eprintln!("Error de Firestore en fetch_live_status ({}): {}", status_code, err_body);
        let status = StatusCode::from_u16(status_code.as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        return Err((status, "Error al obtener estado desde Firestore".to_string()));
    }

    let firestore_doc: Value = response.json().await.unwrap();
    
    let fields = firestore_doc.get("fields");
    let status = fields.and_then(|f| f.get("status")).and_then(|v| v.get("stringValue")).and_then(|s| s.as_str()).unwrap_or("DESCONECTADO").to_string();
    let active_event = fields.and_then(|f| f.get("active_event")).and_then(|v| v.get("stringValue")).and_then(|s| s.as_str()).unwrap_or("Ninguno").to_string();
    let online_players = fields.and_then(|f| f.get("online_players"))
        .and_then(|v| v.get("integerValue"))
        .and_then(|v| v.as_str())
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(0);

    Ok(json!({
        "status": status,
        "active_event": active_event,
        "online_players": online_players
    }))
}
