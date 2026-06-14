pub fn render() -> &'static str {
    r##"
<div class="container mt-5 fade-in">
<div class="hero-section text-center">
<h1 class="slide-up">Bienvenido a DestinyOwner</h1>
<p class="subtitle slide-up" style="margin: 16px auto 0;">Estadísticas en tiempo real, clasificaciones globales y perfiles del staff del servidor.</p>

<div class="ip-card-wrapper mt-5 slide-up">
<div class="ip-card glass-surface">
<div class="d-flex justify-between align-center flex-wrap gap-3">
<div class="d-flex align-center gap-3">
<span class="material-symbols-rounded" style="color:var(--md-sys-color-primary); font-size:32px;">dns</span>
<div>
</div>
</div>
<button class="btn btn-primary" onclick="copyToClipboard()">
<span class="material-symbols-rounded">content_copy</span>
<span id="copyBtnText">Copiar IP</span>
</button>
</div>
</div>
</div>

<!-- Live status widget -->
<div class="ip-card-wrapper mt-5 slide-up" style="margin-top: 32px;">
<div class="ip-card glass-surface" style="max-width: 600px; padding: 24px;">
<div class="d-flex justify-between align-center" style="border-bottom: 1px solid var(--md-sys-color-outline); padding-bottom: 12px; margin-bottom: 16px;">
<h3 style="margin:0; font-weight:700; font-size:1.2rem; display:flex; align-items:center; gap:8px;">
  <span class="material-symbols-rounded" style="color:var(--md-sys-color-primary);">sensors</span> Servidor en Vivo - The Towers
</h3>
<span id="liveStatus" class="badge-vip" style="background: rgba(100,100,100,0.1); color:#aaa; border:1px solid rgba(100,100,100,0.2);">Conectando...</span>
</div>

<div class="d-flex justify-between flex-wrap gap-3" style="text-align: left;">
<div>
  <div style="font-size:0.8rem; color:#888890; text-transform:uppercase; font-weight:600;">Jugadores</div>
  <div id="livePlayers" style="font-size:1.8rem; font-weight:800; color:var(--md-sys-color-on-surface);">0</div>
</div>
<div>
  <div style="font-size:0.8rem; color:#888890; text-transform:uppercase; font-weight:600;">Evento Activo</div>
  <div id="liveEvent" style="font-size:1.4rem; font-weight:700; color:var(--md-sys-color-primary); margin-top: 4px;">Ninguno</div>
</div>
</div>
</div>
</div>

</div>



<script>
function copyToClipboard() {
  const ipText = document.getElementById("serverIP").innerText;
  navigator.clipboard.writeText(ipText).then(() => {
    const btnText = document.getElementById("copyBtnText");
    btnText.innerText = "¡Copiado!";
    setTimeout(() => {
      btnText.innerText = "Copiar IP";
    }, 2000);
  });
}

async function loadLiveStatus() {
  try {
    const res = await fetch('/api/status');
    if (!res.ok) return;
    const data = await res.json();

    const statusEl = document.getElementById('liveStatus');
    const playersEl = document.getElementById('livePlayers');
    const eventEl = document.getElementById('liveEvent');

    if (!statusEl || !playersEl || !eventEl) return;

    playersEl.innerText = data.online_players;
    eventEl.innerText = data.active_event;

    if (data.status === 'INGAME') {
      statusEl.style.background = 'rgba(239, 83, 80, 0.15)';
      statusEl.style.color = '#ef5350';
      statusEl.style.borderColor = 'rgba(239, 83, 80, 0.2)';
      statusEl.innerText = 'En Partida';
    } else if (data.status === 'LOBBY') {
      statusEl.style.background = 'rgba(102, 187, 106, 0.15)';
      statusEl.style.color = '#66bb6a';
      statusEl.style.borderColor = 'rgba(102, 187, 106, 0.2)';
      statusEl.innerText = 'En Lobby';
    } else {
      statusEl.style.background = 'rgba(100, 100, 100, 0.1)';
      statusEl.style.color = '#aaa';
      statusEl.style.borderColor = 'rgba(100, 100, 100, 0.2)';
      statusEl.innerText = 'Desconectado';
    }
  } catch (e) {
    console.error("Error al cargar estado en vivo:", e);
  }
}

loadLiveStatus();
setInterval(loadLiveStatus, 5000);
</script>
"##
}
