use axum::http::StatusCode;
use serde_json::{json, Value};
use crate::state::AppState;

const EXCLUDED_USERS: &[&str] = &[
    "cestart", "espiral_", "prismangames", "pilahd14",
    "pilah14", "elbalam15", "sombradr", "elglower",
];

fn get_env_url(var_name: &str, fallback_port: &str) -> String {
    std::env::var(var_name).unwrap_or_else(|_| format!("http://localhost:{}", fallback_port))
}

async fn fetch_from_minecraft(api_url: &str, path: &str) -> Result<Value, (StatusCode, String)> {
    let url = format!("{}{}", api_url, path);
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, format!("Error de red con Minecraft Server: {}", e)))?;

    if !response.status().is_success() {
        return Err((
            StatusCode::from_u16(response.status().as_u16()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            "Error en la API de Minecraft".to_string(),
        ));
    }

    response.json().await.map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}

pub async fn fetch_leaderboard(_state: &AppState) -> Result<Value, (StatusCode, String)> {
    let ttr_api = get_env_url("TTR_API_URL", "8080");
    let sw_api = get_env_url("SKYWARS_API_URL", "8080");

    let (ttr_res, sw_res) = tokio::join!(
        fetch_from_minecraft(&ttr_api, "/api/leaderboard"),
        fetch_from_minecraft(&sw_api, "/api/leaderboard")
    );

    let filter_list = |res: Result<Value, _>| -> Vec<Value> {
        let mut out = Vec::new();
        if let Ok(Value::Array(arr)) = res {
            for item in arr {
                if let Some(username) = item.get("username").and_then(|u| u.as_str()) {
                    if EXCLUDED_USERS.contains(&username.to_lowercase().as_str()) {
                        continue;
                    }
                    out.push(item);
                }
            }
        }
        out
    };

    Ok(json!({
        "ttr": filter_list(ttr_res),
        "skywars": filter_list(sw_res)
    }))
}

pub async fn fetch_player_stats(_state: &AppState, uuid: &str) -> Result<Value, (StatusCode, String)> {
    let ttr_api = get_env_url("TTR_API_URL", "8080");
    let sw_api = get_env_url("SKYWARS_API_URL", "8080");

    let path = format!("/api/stats?uuid={}", uuid);

    let (ttr_res, sw_res) = tokio::join!(
        fetch_from_minecraft(&ttr_api, &path),
        fetch_from_minecraft(&sw_api, &path)
    );

    let get_fields = |res: Result<Value, _>| -> Value {
        match res {
            Ok(Value::Object(map)) => {
                let wins = map.get("wins").and_then(|v| v.as_i64()).unwrap_or(0);
                let kills = map.get("kills").and_then(|v| v.as_i64()).unwrap_or(0);
                let deaths = map.get("deaths").and_then(|v| v.as_i64()).unwrap_or(0);
                let games = map.get("gamesPlayed").and_then(|v| v.as_i64()).unwrap_or(0);
                let elo = map.get("elo").and_then(|v| v.as_i64()).unwrap_or(1000);

                json!({
                    "mapValue": {
                        "fields": {
                            "wins": { "integerValue": wins.to_string() },
                            "kills": { "integerValue": kills.to_string() },
                            "losses": { "integerValue": deaths.to_string() },
                            "matches_played": { "integerValue": games.to_string() },
                            "elo": { "integerValue": elo.to_string() }
                        }
                    }
                })
            }
            _ => json!({
                "mapValue": {
                    "fields": {
                        "wins": { "integerValue": "0" },
                        "kills": { "integerValue": "0" },
                        "losses": { "integerValue": "0" },
                        "matches_played": { "integerValue": "0" },
                        "elo": { "integerValue": "1000" }
                    }
                }
            })
        }
    };

    let ttr_fields = get_fields(ttr_res);
    let sw_fields = get_fields(sw_res);

    let username = if let Ok(Value::Array(arr)) = fetch_from_minecraft(&sw_api, "/api/leaderboard").await {
        arr.iter()
            .find(|item| item.get("uuid").and_then(|u| u.as_str()) == Some(uuid))
            .and_then(|item| item.get("username").and_then(|u| u.as_str()))
            .unwrap_or("Jugador")
            .to_string()
    } else {
        "Jugador".to_string()
    };

    Ok(json!({
        "fields": {
            "username": { "stringValue": username },
            "stats": {
                "mapValue": {
                    "fields": {
                        "ttr": ttr_fields,
                        "skywars": sw_fields
                    }
                }
            }
        }
    }))
}

pub async fn fetch_live_status(_state: &AppState) -> Result<Value, (StatusCode, String)> {
    let sw_api = get_env_url("SKYWARS_API_URL", "8080");

    match fetch_from_minecraft(&sw_api, "/api/arenas").await {
        Ok(Value::Array(arr)) => {
            let online_players: i64 = arr.iter()
                .map(|arena| arena.get("players").and_then(|p| p.as_i64()).unwrap_or(0))
                .sum();

            let active_event = arr.iter()
                .filter(|arena| arena.get("state").and_then(|s| s.as_str()) == Some("INGAME"))
                .map(|arena| arena.get("name").and_then(|n| n.as_str()).unwrap_or(""))
                .collect::<Vec<&str>>()
                .join(", ");

            let status = if arr.is_empty() { "MANTENIMIENTO" } else { "EN LINEA" };

            Ok(json!({
                "status": status,
                "active_event": if active_event.is_empty() { "Ninguno".to_string() } else { active_event },
                "online_players": online_players
            }))
        }
        _ => Ok(json!({
            "status": "DESCONECTADO",
            "active_event": "Ninguno",
            "online_players": 0
        }))
    }
}
