async function loadLeaderboard() {
    try {
        const res = await fetch('/api/leaderboard');
        const data = await res.json();
        const tbody = document.getElementById('leaderboardBody');
        tbody.innerHTML = '';

        if (!data || data.length === 0) {
            tbody.innerHTML = '<tr><td colspan="4" style="text-align: center;">No hay datos disponibles</td></tr>';
            return;
        }

        data.forEach((player, index) => {
            const row = document.createElement('tr');
            row.innerHTML = `
                <td style="font-weight: bold; color: #60a5fa;">#${index + 1}</td>
                <td>${player.username}</td>
                <td style="color: #ef4444; font-weight: 600;">${player.kills}</td>
                <td style="color: #10b981; font-weight: 600;">${player.wins}</td>
            `;
            tbody.appendChild(row);
        });
    } catch (e) {
        document.getElementById('leaderboardBody').innerHTML = '<tr><td colspan="4" style="text-align: center; color: #ef4444;">Error al cargar la clasificación</td></tr>';
    }
}

async function searchPlayer() {
    const uuid = document.getElementById('playerUuid').value.trim();
    if (!uuid) return;
    try {
        const res = await fetch(`/api/player/${uuid}`);
        if (!res.ok) {
            alert('Jugador no encontrado');
            return;
        }
        const data = await res.json();
        
        // Extraer campos según la estructura de Firebase Firestore
        const username = data.fields.username.stringValue;
        const statsMap = data.fields.stats.mapValue.fields.ttr.mapValue.fields;

        const kills = statsMap.kills.integerValue || 0;
        const wins = statsMap.wins.integerValue || 0;
        const played = statsMap.matches_played.integerValue || 0;

        document.getElementById('playerName').innerText = `Jugador: ${username}`;
        document.getElementById('statKills').innerText = kills;
        document.getElementById('statWins').innerText = wins;
        document.getElementById('statPlayed').innerText = played;
        document.getElementById('playerResult').style.display = 'block';
    } catch (e) {
        alert('Error al buscar estadísticas de jugador.');
    }
}

// Cargar leaderboard al abrir
loadLeaderboard();

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

        // Limpiar clases de estado
        statusEl.className = 'live-status-value';
        if (data.status === 'INGAME') {
            statusEl.classList.add('status-ingame');
            statusEl.innerText = 'En Partida';
        } else if (data.status === 'LOBBY') {
            statusEl.classList.add('status-lobby');
            statusEl.innerText = 'En Lobby';
        } else {
            statusEl.classList.add('status-offline');
            statusEl.innerText = 'Desconectado';
        }

        // Estilo del evento
        eventEl.className = 'live-status-value';
        if (data.active_event && data.active_event !== 'Ninguno') {
            eventEl.classList.add('event-active');
        } else {
            eventEl.classList.add('event-none');
        }
    } catch (e) {
        console.error("Error al cargar estado en vivo:", e);
    }
}

loadLiveStatus();
setInterval(loadLiveStatus, 5000);

function switchTab(game) {
    document.querySelectorAll('.tab-button').forEach(btn => btn.classList.remove('active'));
    document.querySelectorAll('.tab-content').forEach(content => content.classList.remove('active'));

    const clickedBtn = Array.from(document.querySelectorAll('.tab-button')).find(btn => btn.getAttribute('onclick').includes(game));
    if (clickedBtn) clickedBtn.classList.add('active');
    
    const contentEl = document.getElementById(`tab-${game}`);
    if (contentEl) contentEl.classList.add('active');
}
