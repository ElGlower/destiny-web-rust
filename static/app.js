let currentLbGame = 'ttr';

async function loadLeaderboard(game = 'ttr') {
    currentLbGame = game;
    try {
        const res = await fetch(`/api/leaderboard?game=${game}`);
        const data = await res.json();
        const tbody = document.getElementById('leaderboardBody');
        tbody.innerHTML = '';

        if (!data || data.length === 0) {
            tbody.innerHTML = '<tr><td colspan="7" style="text-align: center;">No hay datos disponibles</td></tr>';
            return;
        }

        data.forEach((player, index) => {
            const row = document.createElement('tr');
            row.innerHTML = `
                <td style="font-weight: bold; color: #60a5fa;">#${index + 1}</td>
                <td>${player.username}</td>
                <td style="color: #ef4444; font-weight: 600;">${player.kills}</td>
                <td style="color: #f59e0b; font-weight: 600;">${player.assists}</td>
                <td style="color: #10b981; font-weight: 600;">${player.wins}</td>
                <td style="color: #9ca3af; font-weight: 600;">${player.losses}</td>
                <td style="color: #60a5fa; font-weight: 600;">${player.played}</td>
            `;
            tbody.appendChild(row);
        });
    } catch (e) {
        document.getElementById('leaderboardBody').innerHTML = '<tr><td colspan="7" style="text-align: center; color: #ef4444;">Error al cargar la clasificación</td></tr>';
    }
}

function switchLeaderboard(game) {
    document.getElementById('btn-lb-ttr').classList.toggle('active', game === 'ttr');
    document.getElementById('btn-lb-uhc').classList.toggle('active', game === 'uhc');
    document.getElementById('leaderboardTitle').innerText = `Leaderboard - ${game === 'uhc' ? 'UHC' : 'The Towers'}`;
    loadLeaderboard(game);
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
        document.getElementById('playerName').innerText = `Jugador: ${username}`;

        // TTR Stats
        if (data.fields.stats && data.fields.stats.mapValue && data.fields.stats.mapValue.fields.ttr) {
            const statsMap = data.fields.stats.mapValue.fields.ttr.mapValue.fields;
            document.getElementById('statKills').innerText = statsMap.kills.integerValue || 0;
            document.getElementById('statAssists').innerText = (statsMap.assists && statsMap.assists.integerValue) || 0;
            document.getElementById('statWins').innerText = statsMap.wins.integerValue || 0;
            document.getElementById('statLosses').innerText = (statsMap.losses && statsMap.losses.integerValue) || 0;
            document.getElementById('statPlayed').innerText = statsMap.matches_played.integerValue || 0;
        } else {
            document.getElementById('statKills').innerText = 0;
            document.getElementById('statAssists').innerText = 0;
            document.getElementById('statWins').innerText = 0;
            document.getElementById('statLosses').innerText = 0;
            document.getElementById('statPlayed').innerText = 0;
        }

        // UHC Stats
        if (data.fields.stats && data.fields.stats.mapValue && data.fields.stats.mapValue.fields.uhc) {
            const uhcMap = data.fields.stats.mapValue.fields.uhc.mapValue.fields;
            document.getElementById('statUhcKills').innerText = uhcMap.kills.integerValue || 0;
            document.getElementById('statUhcAssists').innerText = (uhcMap.assists && uhcMap.assists.integerValue) || 0;
            document.getElementById('statUhcWins').innerText = uhcMap.wins.integerValue || 0;
            document.getElementById('statUhcLosses').innerText = (uhcMap.losses && uhcMap.losses.integerValue) || 0;
            document.getElementById('statUhcPlayed').innerText = uhcMap.matches_played.integerValue || 0;
        } else {
            document.getElementById('statUhcKills').innerText = 0;
            document.getElementById('statUhcAssists').innerText = 0;
            document.getElementById('statUhcWins').innerText = 0;
            document.getElementById('statUhcLosses').innerText = 0;
            document.getElementById('statUhcPlayed').innerText = 0;
        }

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
