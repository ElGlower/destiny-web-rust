fn get_json_long(obj: &serde_json::Value, key: &str) -> i64 {
    obj.get(key)
       .and_then(|v| v.get("integerValue"))
       .and_then(|v| v.as_str())
       .and_then(|s| s.parse::<i64>().ok())
       .unwrap_or(0)
}

fn get_mode(stats_value: &serde_json::Value, mode: &str) -> (i64, i64, i64, i64, i64) {
    let f = stats_value.get("fields")
        .and_then(|f| f.get("stats"))
        .and_then(|s| s.get("mapValue"))
        .and_then(|m| m.get("fields"))
        .and_then(|f| f.get(mode))
        .and_then(|t| t.get("mapValue"))
        .and_then(|m| m.get("fields"));
    match f {
        Some(f) => (
            get_json_long(f, "kills"),
            get_json_long(f, "assists"),
            get_json_long(f, "wins"),
            get_json_long(f, "losses"),
            get_json_long(f, "matches_played"),
        ),
        None => (0, 0, 0, 0, 0),
    }
}

fn get_radar_point(angle_idx: usize, percentage: f64) -> (f64, f64) {
    let r = (percentage / 100.0 * 100.0).clamp(10.0, 100.0);
    let a = match angle_idx {
        0 => 0.0,
        1 => -std::f64::consts::PI / 3.0,
        2 => -2.0 * std::f64::consts::PI / 3.0,
        3 => std::f64::consts::PI,
        4 => 2.0 * std::f64::consts::PI / 3.0,
        5 => std::f64::consts::PI / 3.0,
        _ => 0.0,
    };
    (150.0 + r * a.cos(), 150.0 + r * a.sin())
}

fn mode_card(label: &str, icon: &str, color: &str, kills: i64, assists: i64, wins: i64, losses: i64, played: i64, description: &str) -> String {
    format!(
        r##"<div class="event-item glass-surface">
  <div class="event-meta">
    <span class="event-badge" style="background:rgba({color_bg}); color:{color}; border-color:rgba({color_bg});">{label}</span>
  </div>
  <div class="event-details">
    <h3><span class="material-symbols-rounded" style="vertical-align:middle;color:{color};margin-right:6px;">{icon}</span>{label}</h3>
    <p>{description}</p>
    <div class="stats-details-grid">
      <div class="stat-detail-card glass-surface">
        <span class="material-symbols-rounded stat-icon color-kills">swords</span>
        <div class="stat-detail-info"><span class="stat-value">{kills}</span><span class="stat-label">Asesinatos</span><span class="stat-desc">Enemigos eliminados</span></div>
      </div>
      <div class="stat-detail-card glass-surface">
        <span class="material-symbols-rounded stat-icon color-assists">handshake</span>
        <div class="stat-detail-info"><span class="stat-value">{assists}</span><span class="stat-label">Asistencias</span><span class="stat-desc">Apoyo a compañeros</span></div>
      </div>
      <div class="stat-detail-card glass-surface">
        <span class="material-symbols-rounded stat-icon color-wins">emoji_events</span>
        <div class="stat-detail-info"><span class="stat-value">{wins}</span><span class="stat-label">Victorias</span><span class="stat-desc">Juegos ganados</span></div>
      </div>
      <div class="stat-detail-card glass-surface">
        <span class="material-symbols-rounded stat-icon color-losses">skull</span>
        <div class="stat-detail-info"><span class="stat-value">{losses}</span><span class="stat-label">Derrotas</span><span class="stat-desc">Juegos perdidos</span></div>
      </div>
      <div class="stat-detail-card glass-surface">
        <span class="material-symbols-rounded stat-icon color-matches">sports_esports</span>
        <div class="stat-detail-info"><span class="stat-value">{played}</span><span class="stat-label">Partidas</span><span class="stat-desc">Encuentros totales</span></div>
      </div>
    </div>
  </div>
</div>"##,
        label = label,
        icon = icon,
        color = color,
        color_bg = "155, 89, 182, 0.15",
        description = description,
        kills = kills,
        assists = assists,
        wins = wins,
        losses = losses,
        played = played,
    )
}

pub fn render(username: &str, stats_value: &serde_json::Value) -> String {
    let (ttr_kills, ttr_assists, ttr_wins, ttr_losses, ttr_matches) = get_mode(stats_value, "ttr");
    let (sw_kills, sw_assists, sw_wins, sw_losses, sw_matches) = get_mode(stats_value, "skywars");

    let total_kills = ttr_kills + sw_kills;
    let total_wins = ttr_wins + sw_wins;
    let total_losses = ttr_losses + sw_losses;
    let total_matches = ttr_matches + sw_matches;

    let kd_ratio = if total_losses > 0 { total_kills as f64 / total_losses as f64 } else { total_kills as f64 };
    let winrate = if total_matches > 0 { total_wins as f64 * 100.0 / total_matches as f64 } else { 0.0 };

    let p_combat = (total_kills as f64 * 3.0).clamp(15.0, 100.0);
    let p_survival = winrate.clamp(15.0, 100.0);
    let p_tactics = (total_wins as f64 * 8.0).clamp(15.0, 100.0);
    let p_velocity = (total_matches as f64 * 5.0).clamp(15.0, 100.0);
    let p_aim = (kd_ratio * 30.0).clamp(15.0, 100.0);
    let p_consistency = (100.0 - (total_losses as f64 * 100.0 / total_matches.max(1) as f64)).clamp(15.0, 100.0);

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

    let ttr_card = mode_card(
        "The Towers Remastered",
        "castle",
        "#9b59b6",
        ttr_kills, ttr_assists, ttr_wins, ttr_losses, ttr_matches,
        "Controla los puentes y anota en las jaulas enemigas. Estadísticas acumulativas de combate e incursión."
    );

    let sw_card = mode_card(
        "SkyWars",
        "cloudy",
        "#38bdf8",
        sw_kills, sw_assists, sw_wins, sw_losses, sw_matches,
        "Sobrevive en las islas flotantes. Recoge recursos y elimina oponentes para ser el último en pie."
    );

    format!(
        r##"
<div class="container mt-5 fade-in">
<div class="profile-container">

  <div class="profile-card glass-surface slide-up">
    <div class="profile-header-layout">
      <div class="profile-avatar-wrapper">
        <img src="https://mc-heads.net/body/{username}/150" class="profile-skin" alt="{username}">
      </div>
      <div class="profile-info">
        <div class="badge-vip">Perfil de Red</div>
        <h1 class="profile-name">{username}</h1>
        <p class="profile-status"><span class="status-dot"></span> Registrado en Destiny Network</p>
        <div class="quick-stats-row">
          <div class="quick-stat-box">
            <span class="num">{total_wins}</span>
            <span class="lbl">Victorias Totales</span>
          </div>
          <div class="quick-stat-box">
            <span class="num">{kd_ratio:.2}</span>
            <span class="lbl">K/D Ratio</span>
          </div>
          <div class="quick-stat-box">
            <span class="num">{total_matches}</span>
            <span class="lbl">Partidas</span>
          </div>
        </div>
      </div>

      <div class="profile-chart-wrapper">
        <svg viewBox="0 0 300 300" class="radar-chart">
          <polygon points="250,150 200,63.4 100,63.4 50,150 100,236.6 200,236.6" fill="none" stroke="rgba(255,255,255,0.08)" stroke-width="2"/>
          <polygon points="216,150 183,92.8 117,92.8 84,150 117,207.2 183,207.2" fill="none" stroke="rgba(255,255,255,0.08)" stroke-width="1.5"/>
          <polygon points="183,150 166.5,121.4 133.5,121.4 117,150 133.5,178.6 166.5,178.6" fill="none" stroke="rgba(255,255,255,0.08)" stroke-width="1"/>
          <line x1="150" y1="150" x2="250" y2="150" stroke="rgba(255,255,255,0.08)" stroke-dasharray="2,2"/>
          <line x1="150" y1="150" x2="200" y2="63.4" stroke="rgba(255,255,255,0.08)" stroke-dasharray="2,2"/>
          <line x1="150" y1="150" x2="100" y2="63.4" stroke="rgba(255,255,255,0.08)" stroke-dasharray="2,2"/>
          <line x1="150" y1="150" x2="50" y2="150" stroke="rgba(255,255,255,0.08)" stroke-dasharray="2,2"/>
          <line x1="150" y1="150" x2="100" y2="236.6" stroke="rgba(255,255,255,0.08)" stroke-dasharray="2,2"/>
          <line x1="150" y1="150" x2="200" y2="236.6" stroke="rgba(255,255,255,0.08)" stroke-dasharray="2,2"/>
          <polygon points="{points_str}" fill="rgba(208,188,255,0.25)" stroke="var(--md-sys-color-primary)" stroke-width="3"/>
          <circle cx="{pt0x:.1}" cy="{pt0y:.1}" r="4" fill="var(--md-sys-color-primary)"/>
          <circle cx="{pt1x:.1}" cy="{pt1y:.1}" r="4" fill="var(--md-sys-color-primary)"/>
          <circle cx="{pt2x:.1}" cy="{pt2y:.1}" r="4" fill="var(--md-sys-color-primary)"/>
          <circle cx="{pt3x:.1}" cy="{pt3y:.1}" r="4" fill="var(--md-sys-color-primary)"/>
          <circle cx="{pt4x:.1}" cy="{pt4y:.1}" r="4" fill="var(--md-sys-color-primary)"/>
          <circle cx="{pt5x:.1}" cy="{pt5y:.1}" r="4" fill="var(--md-sys-color-primary)"/>
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

  <div class="events-card glass-surface mt-5 slide-up" style="animation-delay:0.1s;">
    <h2 class="section-title"><span class="material-symbols-rounded">sports_esports</span> Estadísticas por Modalidad</h2>
    <div class="events-timeline">
      {ttr_card}
      {sw_card}
    </div>
  </div>

</div>
</div>
"##,
        username = username,
        total_wins = total_wins,
        kd_ratio = kd_ratio,
        total_matches = total_matches,
        points_str = points_str,
        pt0x = pt0.0, pt0y = pt0.1,
        pt1x = pt1.0, pt1y = pt1.1,
        pt2x = pt2.0, pt2y = pt2.1,
        pt3x = pt3.0, pt3y = pt3.1,
        pt4x = pt4.0, pt4y = pt4.1,
        pt5x = pt5.0, pt5y = pt5.1,
        ttr_card = ttr_card,
        sw_card = sw_card,
    )
}
