fn get_json_long(obj: &serde_json::Value, key: &str) -> i64 {
    obj.get(key)
       .and_then(|v| v.get("integerValue"))
       .and_then(|v| v.as_str())
       .and_then(|s| s.parse::<i64>().ok())
       .unwrap_or(0)
}

fn get_radar_point(angle_idx: usize, percentage: f64) -> (f64, f64) {
    let r = (percentage / 100.0) * 100.0; // Max radius is 100
    let r = r.clamp(10.0, 100.0); // Keep within reasonable bounds
    
    // Angles to match hexagon directions:
    // idx 0: Combat (angle 0 deg -> right)
    // idx 1: Velocity (angle -60 deg -> top-right)
    // idx 2: Tactics (angle -120 deg -> top-left)
    // idx 3: Aim (angle 180 deg -> left)
    // idx 4: Consistency (angle 120 deg -> bottom-left)
    // idx 5: Survival (angle 60 deg -> bottom-right)
    let a = match angle_idx {
        0 => 0.0,
        1 => -std::f64::consts::PI / 3.0,
        2 => -2.0 * std::f64::consts::PI / 3.0,
        3 => std::f64::consts::PI,
        4 => 2.0 * std::f64::consts::PI / 3.0,
        5 => std::f64::consts::PI / 3.0,
        _ => 0.0,
    };
    
    let x = 150.0 + r * a.cos();
    let y = 150.0 + r * a.sin();
    (x, y)
}

pub fn render(username: &str, stats_value: &serde_json::Value) -> String {
    // Parse stats
    let mut ttr_kills = 0;
    let mut ttr_wins = 0;
    let mut ttr_losses = 0;
    let mut ttr_matches = 0;
    let mut ttr_assists = 0;
    
    let mut sw_kills = 0;
    let mut sw_wins = 0;
    let mut sw_losses = 0;
    let mut sw_matches = 0;
    
    let mut bw_kills = 0;
    let mut bw_wins = 0;
    let mut bw_losses = 0;
    let mut bw_matches = 0;
    
    if let Some(fields) = stats_value.get("fields") {
        if let Some(stats) = fields.get("stats").and_then(|s| s.get("mapValue")).and_then(|m| m.get("fields")) {
            // Parse TTR
            if let Some(ttr) = stats.get("ttr").and_then(|v| v.get("mapValue")).and_then(|m| m.get("fields")) {
                ttr_kills = get_json_long(ttr, "kills");
                ttr_wins = get_json_long(ttr, "wins");
                ttr_losses = get_json_long(ttr, "losses");
                ttr_matches = get_json_long(ttr, "matches_played");
                ttr_assists = get_json_long(ttr, "assists");
            }
            // Parse SkyWars
            if let Some(sw) = stats.get("skywars").and_then(|v| v.get("mapValue")).and_then(|m| m.get("fields")) {
                sw_kills = get_json_long(sw, "kills");
                sw_wins = get_json_long(sw, "wins");
                sw_losses = get_json_long(sw, "losses");
                sw_matches = get_json_long(sw, "matches_played");
            }
            // Parse BedWars
            if let Some(bw) = stats.get("bedwars").and_then(|v| v.get("mapValue")).and_then(|m| m.get("fields")) {
                bw_kills = get_json_long(bw, "kills");
                bw_wins = get_json_long(bw, "wins");
                bw_losses = get_json_long(bw, "losses");
                bw_matches = get_json_long(bw, "matches_played");
            }
        }
    }
    
    let total_kills = ttr_kills + sw_kills + bw_kills;
    let total_wins = ttr_wins + sw_wins + bw_wins;
    let total_losses = ttr_losses + sw_losses + bw_losses;
    let total_matches = ttr_matches + sw_matches + bw_matches;
    
    let kd_ratio = if total_losses > 0 {
        (total_kills as f64) / (total_losses as f64)
    } else {
        total_kills as f64
    };
    
    let winrate = if total_matches > 0 {
        (total_wins as f64) * 100.0 / (total_matches as f64)
    } else {
        0.0
    };
    
    // Performance metrics percentages (0 - 100)
    let p_combat = (total_kills as f64 * 3.0).clamp(15.0, 100.0);
    let p_survival = winrate.clamp(15.0, 100.0);
    let p_tactics = (total_wins as f64 * 8.0).clamp(15.0, 100.0);
    let p_velocity = (total_matches as f64 * 5.0).clamp(15.0, 100.0);
    let p_aim = (kd_ratio * 30.0).clamp(15.0, 100.0);
    let p_consistency = (100.0 - (total_losses as f64 * 100.0 / total_matches.max(1) as f64)).clamp(15.0, 100.0);
    
    // Coordinates
    let pt0 = get_radar_point(0, p_combat);
    let pt1 = get_radar_point(1, p_velocity);
    let pt2 = get_radar_point(2, p_tactics);
    let pt3 = get_radar_point(3, p_aim);
    let pt4 = get_radar_point(4, p_consistency);
    let pt5 = get_radar_point(5, p_survival);
    
    let points_str = format!(
        "{:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1} {:.1},{:.1}",
        pt0.0, pt0.1, pt1.0, pt1.1, pt2.0, pt2.1, pt3.0, pt3.1, pt4.0, pt4.1, pt5.0, pt5.1
    );

    format!(
        r##"
<div class="container mt-5 fade-in">
<div class="profile-container">
  
  <!-- Ficha de Perfil Principal (Carátula) -->
  <div class="profile-card glass-surface slide-up">
    <div class="profile-header-layout">
      <div class="profile-avatar-wrapper">
        <img src="https://mc-heads.net/body/{}/150" class="profile-skin" alt="{}">
      </div>
      <div class="profile-info">
        <div class="badge-vip">Perfil de Red</div>
        <h1 class="profile-name">{}</h1>
        <p class="profile-status"><span class="status-dot"></span> Registrado en Destiny Network</p>
        
        <div class="quick-stats-row">
          <div class="quick-stat-box">
            <span class="num">{}</span>
            <span class="lbl">Victorias</span>
          </div>
          <div class="quick-stat-box">
            <span class="num">{:.2}</span>
            <span class="lbl">K/D Ratio</span>
          </div>
          <div class="quick-stat-box">
            <span class="num">{}</span>
            <span class="lbl">Partidas</span>
          </div>
        </div>
      </div>

      <!-- Gráfico de Hexágono (Radar Chart SVG) -->
      <div class="profile-chart-wrapper">
        <svg viewBox="0 0 300 300" class="radar-chart">
          <!-- Concentric hexagons (grid) -->
          <polygon points="250,150 200,63.4 100,63.4 50,150 100,236.6 200,236.6" fill="none" stroke="rgba(255,255,255,0.08)" stroke-width="2"/>
          <polygon points="216,150 183,92.8 117,92.8 84,150 117,207.2 183,207.2" fill="none" stroke="rgba(255,255,255,0.08)" stroke-width="1.5"/>
          <polygon points="183,150 166.5,121.4 133.5,121.4 117,150 133.5,178.6 166.5,178.6" fill="none" stroke="rgba(255,255,255,0.08)" stroke-width="1"/>
          
          <!-- Axis lines -->
          <line x1="150" y1="150" x2="250" y2="150" stroke="rgba(255,255,255,0.08)" stroke-dasharray="2,2"/>
          <line x1="150" y1="150" x2="200" y2="63.4" stroke="rgba(255,255,255,0.08)" stroke-dasharray="2,2"/>
          <line x1="150" y1="150" x2="100" y2="63.4" stroke="rgba(255,255,255,0.08)" stroke-dasharray="2,2"/>
          <line x1="150" y1="150" x2="50" y2="150" stroke="rgba(255,255,255,0.08)" stroke-dasharray="2,2"/>
          <line x1="150" y1="150" x2="100" y2="236.6" stroke="rgba(255,255,255,0.08)" stroke-dasharray="2,2"/>
          <line x1="150" y1="150" x2="200" y2="236.6" stroke="rgba(255,255,255,0.08)" stroke-dasharray="2,2"/>
          
          <!-- Player Stat Shape -->
          <polygon points="{}" fill="rgba(208, 188, 255, 0.25)" stroke="var(--md-sys-color-primary)" stroke-width="3"/>
          
          <!-- Dot markers on vertices -->
          <circle cx="{:.1}" cy="{:.1}" r="4" fill="var(--md-sys-color-primary)"/>
          <circle cx="{:.1}" cy="{:.1}" r="4" fill="var(--md-sys-color-primary)"/>
          <circle cx="{:.1}" cy="{:.1}" r="4" fill="var(--md-sys-color-primary)"/>
          <circle cx="{:.1}" cy="{:.1}" r="4" fill="var(--md-sys-color-primary)"/>
          <circle cx="{:.1}" cy="{:.1}" r="4" fill="var(--md-sys-color-primary)"/>
          <circle cx="{:.1}" cy="{:.1}" r="4" fill="var(--md-sys-color-primary)"/>

          <!-- Label Texts -->
          <text x="255" y="154" fill="#a0a0a8" font-size="11" font-family="Outfit" font-weight="700">COMBATE</text>
          <text x="205" y="52" fill="#a0a0a8" font-size="11" font-family="Outfit" font-weight="700">VELOCIDAD</text>
          <text x="50" y="52" fill="#a0a0a8" font-size="11" font-family="Outfit" font-weight="700">TÁCTICA</text>
          <text x="5" y="154" fill="#a0a0a8" font-size="11" font-family="Outfit" font-weight="700">PUNTERÍA</text>
          <text x="40" y="255" fill="#a0a0a8" font-size="11" font-family="Outfit" font-weight="700">CONSISTENCIA</text>
          <text x="205" y="255" fill="#a0a0a8" font-size="11" font-family="Outfit" font-weight="700">SUPERVIVENCIA</text>
        </svg>
      </div>

    </div>
  </div>

  <!-- Estadísticas por Modalidad -->
  <div class="events-card glass-surface mt-5 slide-up" style="animation-delay: 0.1s;">
    <h2 class="section-title"><span class="material-symbols-rounded">sports_esports</span> Estadísticas por Modalidad</h2>
    
    <div class="events-timeline">
      
      <!-- The Towers Card -->
      <div class="event-item glass-surface">
        <div class="event-meta">
          <span class="event-badge gold" style="background: rgba(155, 89, 182, 0.15); color: #9b59b6; border-color: rgba(155, 89, 182, 0.2);">The Towers</span>
        </div>
        <div class="event-details">
          <h3>The Towers Remastered</h3>
          <p>Controla los puentes y anota en las jaulas enemigas. Estadísticas acumulativas de combate e incursión.</p>
          <div class="event-stats-chips">
            <span class="chip"><span class="material-symbols-rounded">swords</span> {} Kills</span>
            <span class="chip"><span class="material-symbols-rounded">handshake</span> {} Assists</span>
            <span class="chip"><span class="material-symbols-rounded">emoji_events</span> {} Victorias</span>
            <span class="chip"><span class="material-symbols-rounded">skull</span> {} Derrotas</span>
            <span class="chip"><span class="material-symbols-rounded">videogame_asset</span> {} Partidas</span>
          </div>
        </div>
      </div>

      <!-- SkyWars Card -->
      <div class="event-item glass-surface">
        <div class="event-meta">
          <span class="event-badge gold" style="background: rgba(52, 73, 94, 0.15); color: #34495e; border-color: rgba(52, 73, 94, 0.2);">SkyWars</span>
        </div>
        <div class="event-details">
          <h3>Destiny SkyWars</h3>
          <p>Combate en islas flotantes individuales. El último jugador en pie gana.</p>
          <div class="event-stats-chips">
            <span class="chip"><span class="material-symbols-rounded">swords</span> {} Kills</span>
            <span class="chip"><span class="material-symbols-rounded">emoji_events</span> {} Victorias</span>
            <span class="chip"><span class="material-symbols-rounded">skull</span> {} Derrotas</span>
            <span class="chip"><span class="material-symbols-rounded">videogame_asset</span> {} Partidas</span>
          </div>
        </div>
      </div>

      <!-- BedWars Card -->
      <div class="event-item glass-surface">
        <div class="event-meta">
          <span class="event-badge gold" style="background: rgba(230, 126, 34, 0.15); color: #e67e22; border-color: rgba(230, 126, 34, 0.2);">BedWars</span>
        </div>
        <div class="event-details">
          <h3>Destiny BedWars</h3>
          <p>Protege tu cama y destruye la de los oponentes para eliminar a sus equipos.</p>
          <div class="event-stats-chips">
            <span class="chip"><span class="material-symbols-rounded">swords</span> {} Kills</span>
            <span class="chip"><span class="material-symbols-rounded">emoji_events</span> {} Victorias</span>
            <span class="chip"><span class="material-symbols-rounded">skull</span> {} Derrotas</span>
            <span class="chip"><span class="material-symbols-rounded">videogame_asset</span> {} Partidas</span>
          </div>
        </div>
      </div>
      
    </div>
  </div>

</div>
</div>
"##,
        username, username, username, total_wins, kd_ratio, total_matches,
        points_str, 
        pt0.0, pt0.1, pt1.0, pt1.1, pt2.0, pt2.1, pt3.0, pt3.1, pt4.0, pt4.1, pt5.0, pt5.1,
        ttr_kills, ttr_assists, ttr_wins, ttr_losses, ttr_matches,
        sw_kills, sw_wins, sw_losses, sw_matches,
        bw_kills, bw_wins, bw_losses, bw_matches
    )
}
