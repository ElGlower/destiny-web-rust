pub fn render(data: &serde_json::Value) -> String {
    let mut rows_html = String::new();
    
    if let Some(arr) = data.as_array() {
        for (index, player) in arr.iter().enumerate() {
            let rank = index + 1;
            let uuid = player.get("uuid").and_then(|v| v.as_str()).unwrap_or("");
            let username = player.get("username").and_then(|v| v.as_str()).unwrap_or("Desconocido");
            let kills = player.get("kills").and_then(|v| v.as_i64()).unwrap_or(0);
            let wins = player.get("wins").and_then(|v| v.as_i64()).unwrap_or(0);
            
            rows_html.push_str(&format!(
                r##"<tr class="player-row clickable-row" onclick="window.location.href='/profile/{}'">
<td><span class="rank-badge">{}</span></td>
<td>
  <div class="player-profile">
    <img src="https://mc-heads.net/avatar/{}/32" class="player-head" alt="{}">
    <span style="font-weight: 700; color: var(--md-sys-color-primary);">{} <span style="font-size:0.8rem; font-weight:400; color:#888;">(Ver Perfil)</span></span>
  </div>
</td>
<td>Wins: {}</td>
<td class="rating-val">Kills: {}</td>
</tr>"##,
                uuid, rank, username, username, username, wins, kills
            ));
        }
    } else {
        rows_html.push_str("<tr><td colspan=\"4\" style=\"text-align: center; color: #888;\">No hay datos disponibles en la clasificación</td></tr>");
    }
    
    format!(
        r##"
<div class="container mt-5 fade-in">
<div class="d-flex flex-column align-center">
<h1 class="slide-up">Clasificaciones de Jugadores</h1>
<p class="subtitle slide-up">Tiers de PvP Clasificatorio • Actualizado en vivo</p>
</div>

<div class="tabs-scroll mt-5 slide-up">
<button class="btn btn-glass active"><span class="material-symbols-rounded">public</span> Global</button>
</div>

<div class="leaderboard-card glass-surface mt-5 slide-up">
<div class="d-flex justify-between align-center flex-wrap gap-3">
<h2 style="margin:0; font-size:1.5rem; font-weight:700;">Top Jugadores</h2>
<div class="search-bar">
<span class="material-symbols-rounded">search</span>
<input type="text" id="searchInput" onkeyup="filterTable()" placeholder="Buscar jugador...">
</div>
</div>

<div class="table-wrapper">
<table id="playersTable">
<thead>
<tr>
<th>#</th>
<th>Jugador</th>
<th>Estadísticas</th>
<th>Kills</th>
</tr>
</thead>
<tbody>
{}
</tbody>
</table>
</div>
</div>
</div>

<script>
function filterTable() {{
  var input, filter, table, tr, td, i, txtValue;
  input = document.getElementById("searchInput");
  filter = input.value.toUpperCase();
  table = document.getElementById("playersTable");
  tr = table.getElementsByTagName("tr");
  for (i = 1; i < tr.length; i++) {{
    td = tr[i].getElementsByTagName("td")[1];
    if (td) {{
      txtValue = td.textContent || td.innerText;
      if (txtValue.toUpperCase().indexOf(filter) > -1) {{
        tr[i].style.display = "";
      }} else {{
        tr[i].style.display = "none";
      }}
    }}       
  }}
}}
</script>
"##,
        rows_html
    )
}
