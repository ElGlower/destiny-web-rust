pub fn render(data: &serde_json::Value) -> String {
    let empty = vec![];
    let ttr_arr = data.get("ttr").and_then(|v| v.as_array()).unwrap_or(&empty);
    let sw_arr = data.get("skywars").and_then(|v| v.as_array()).unwrap_or(&empty);

    let ttr_rows = build_rows(ttr_arr);
    let sw_rows = build_rows(sw_arr);

    format!(
        r##"
<div class="container mt-5 fade-in">
<div class="d-flex flex-column align-center">
  <h1 class="slide-up">Clasificaciones de Jugadores</h1>
  <p class="subtitle slide-up">Rankings por Modalidad · Actualizado en vivo</p>
</div>

<div class="tabs-scroll mt-5 slide-up" id="lb-tabs" role="tablist">
  <button class="btn btn-glass active" onclick="switchTab('ttr')" id="tab-ttr" aria-selected="true">
    <span class="material-symbols-rounded">castle</span> The Towers
  </button>
  <button class="btn btn-glass" onclick="switchTab('skywars')" id="tab-skywars" aria-selected="false">
    <span class="material-symbols-rounded">cloudy</span> SkyWars
  </button>
</div>

<!-- The Towers Leaderboard -->
<div class="leaderboard-card glass-surface mt-5 slide-up" id="panel-ttr">
  <div class="d-flex justify-between align-center flex-wrap gap-3">
    <h2 style="margin:0; font-size:1.5rem; font-weight:700;">
      <span class="material-symbols-rounded" style="color:#c084fc; vertical-align:middle; margin-right:6px;">castle</span>
      The Towers Remastered
    </h2>
    <div class="search-bar">
      <span class="material-symbols-rounded">search</span>
      <input type="text" id="searchTTR" onkeyup="filterTable('playersTableTTR','searchTTR')" placeholder="Buscar jugador...">
    </div>
  </div>
  <div class="table-wrapper">
    <table id="playersTableTTR">
      <thead><tr>
        <th>#</th><th>Jugador</th><th>Kills</th><th>Assists</th><th>Wins</th><th>Losses</th><th>Partidas</th>
      </tr></thead>
      <tbody>{}</tbody>
    </table>
  </div>
</div>

<!-- SkyWars Leaderboard -->
<div class="leaderboard-card glass-surface mt-5 slide-up" id="panel-skywars" style="display:none;">
  <div class="d-flex justify-between align-center flex-wrap gap-3">
    <h2 style="margin:0; font-size:1.5rem; font-weight:700;">
      <span class="material-symbols-rounded" style="color:#38bdf8; vertical-align:middle; margin-right:6px;">cloudy</span>
      SkyWars
    </h2>
    <div class="search-bar">
      <span class="material-symbols-rounded">search</span>
      <input type="text" id="searchSW" onkeyup="filterTable('playersTableSW','searchSW')" placeholder="Buscar jugador...">
    </div>
  </div>
  <div class="table-wrapper">
    <table id="playersTableSW">
      <thead><tr>
        <th>#</th><th>Jugador</th><th>Kills</th><th>Assists</th><th>Wins</th><th>Losses</th><th>Partidas</th>
      </tr></thead>
      <tbody>{}</tbody>
    </table>
  </div>
</div>

</div>

<script>
function switchTab(mode) {{
  document.getElementById('panel-ttr').style.display = mode === 'ttr' ? '' : 'none';
  document.getElementById('panel-skywars').style.display = mode === 'skywars' ? '' : 'none';
  document.getElementById('tab-ttr').classList.toggle('active', mode === 'ttr');
  document.getElementById('tab-skywars').classList.toggle('active', mode === 'skywars');
  document.getElementById('tab-ttr').setAttribute('aria-selected', mode === 'ttr');
  document.getElementById('tab-skywars').setAttribute('aria-selected', mode === 'skywars');
}}

function filterTable(tableId, inputId) {{
  var input = document.getElementById(inputId);
  var filter = input.value.toUpperCase();
  var table = document.getElementById(tableId);
  var rows = table.getElementsByTagName('tr');
  for (var i = 1; i < rows.length; i++) {{
    var td = rows[i].getElementsByTagName('td')[1];
    if (td) {{
      var txt = td.textContent || td.innerText;
      rows[i].style.display = txt.toUpperCase().indexOf(filter) > -1 ? '' : 'none';
    }}
  }}
}}
</script>
"##,
        ttr_rows, sw_rows
    )
}

fn build_rows(arr: &[serde_json::Value]) -> String {
    if arr.is_empty() {
        return r#"<tr><td colspan="7" style="text-align:center; color:#888;">Sin datos disponibles</td></tr>"#.to_string();
    }
    let mut html = String::new();
    for (index, player) in arr.iter().enumerate() {
        let rank = index + 1;
        let uuid = player.get("uuid").and_then(|v| v.as_str()).unwrap_or("");
        let username = player.get("username").and_then(|v| v.as_str()).unwrap_or("Desconocido");
        let kills = player.get("kills").and_then(|v| v.as_i64()).unwrap_or(0);
        let assists = player.get("assists").and_then(|v| v.as_i64()).unwrap_or(0);
        let wins = player.get("wins").and_then(|v| v.as_i64()).unwrap_or(0);
        let losses = player.get("losses").and_then(|v| v.as_i64()).unwrap_or(0);
        let played = player.get("played").and_then(|v| v.as_i64()).unwrap_or(0);

        let rank_display = match rank {
            1 => r#"<span class="rank-badge" style="background:linear-gradient(135deg,#ffd700,#f59e0b);color:#000;">🥇 1</span>"#.to_string(),
            2 => r#"<span class="rank-badge" style="background:linear-gradient(135deg,#c0c0c0,#9ca3af);color:#000;">🥈 2</span>"#.to_string(),
            3 => r#"<span class="rank-badge" style="background:linear-gradient(135deg,#cd7f32,#b45309);color:#fff;">🥉 3</span>"#.to_string(),
            _ => format!(r#"<span class="rank-badge">{}</span>"#, rank),
        };

        html.push_str(&format!(
            r##"<tr class="player-row clickable-row" onclick="window.location.href='/profile/{}'">
<td>{}</td>
<td><div class="player-profile">
  <img src="https://mc-heads.net/avatar/{}/32" class="player-head" alt="{}">
  <span style="font-weight:700;color:var(--md-sys-color-primary);">{} <span style="font-size:0.8rem;font-weight:400;color:#888;">(Ver Perfil)</span></span>
</div></td>
<td style="color:#ef4444;font-weight:600;">{}</td>
<td style="color:#f59e0b;font-weight:600;">{}</td>
<td style="color:#10b981;font-weight:600;">{}</td>
<td style="color:#9ca3af;font-weight:600;">{}</td>
<td style="color:#60a5fa;font-weight:600;">{}</td>
</tr>"##,
            uuid, rank_display, username, username, username,
            kills, assists, wins, losses, played
        ));
    }
    html
}
