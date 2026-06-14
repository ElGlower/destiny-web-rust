pub fn render() -> &'static str {
    r##"
<div class="container mt-5 fade-in">
<div class="d-flex flex-column align-center text-center">
<h1 class="slide-up">Nuestro Equipo</h1>
<p class="subtitle slide-up">Los encargados de mantener y mejorar DestinyNetwork a diario.</p>
</div>

<div class="staff-section mt-5">
<h2 class="staff-rank-title slide-up">DestinyTeam</h2>
<div class="staff-grid">
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('ElGlower')">
    <div class="staff-skin-wrapper">
      <img src="https://mc-heads.net/body/ElGlower/100" class="staff-skin-img normal-pose" alt="ElGlower">
      <img src="https://mc-heads.net/player/ElGlower/100" class="staff-skin-img hover-pose" alt="ElGlower">
    </div>
    <div class="staff-info-box">
      <h3>ElGlower</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('ElBalam15')">
    <div class="staff-skin-wrapper">
      <img src="https://mc-heads.net/body/ElBalam15/100" class="staff-skin-img normal-pose" alt="ElBalam15">
      <img src="https://mc-heads.net/player/ElBalam15/100" class="staff-skin-img hover-pose" alt="ElBalam15">
    </div>
    <div class="staff-info-box">
      <h3>ElBalam15</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('espiral_')">
    <div class="staff-skin-wrapper">
      <img src="https://mc-heads.net/body/espiral_/100" class="staff-skin-img normal-pose" alt="espiral_">
      <img src="https://mc-heads.net/player/espiral_/100" class="staff-skin-img hover-pose" alt="espiral_">
    </div>
    <div class="staff-info-box">
      <h3>espiral_</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('pilahd14')">
    <div class="staff-skin-wrapper">
      <img src="https://mc-heads.net/body/pilahd14/100" class="staff-skin-img normal-pose" alt="pilahd14">
      <img src="https://mc-heads.net/player/pilahd14/100" class="staff-skin-img hover-pose" alt="pilahd14">
    </div>
    <div class="staff-info-box">
      <h3>pilahd14</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('prismangames')">
    <div class="staff-skin-wrapper">
      <img src="https://mc-heads.net/body/prismangames/100" class="staff-skin-img normal-pose" alt="prismangames">
      <img src="https://mc-heads.net/player/prismangames/100" class="staff-skin-img hover-pose" alt="prismangames">
    </div>
    <div class="staff-info-box">
      <h3>prismangames</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('sombradr')">
    <div class="staff-skin-wrapper">
      <img src="https://mc-heads.net/body/sombradr/100" class="staff-skin-img normal-pose" alt="sombradr">
      <img src="https://mc-heads.net/player/sombradr/100" class="staff-skin-img hover-pose" alt="sombradr">
    </div>
    <div class="staff-info-box">
      <h3>sombradr</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('ripkyn')">
    <div class="staff-skin-wrapper">
      <img src="https://mc-heads.net/body/ripkyn/100" class="staff-skin-img normal-pose" alt="ripkyn">
      <img src="https://mc-heads.net/player/ripkyn/100" class="staff-skin-img hover-pose" alt="ripkyn">
    </div>
    <div class="staff-info-box">
      <h3>ripkyn</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('Cestart')">
    <div class="staff-skin-wrapper">
      <img src="https://mc-heads.net/body/Cestart/100" class="staff-skin-img normal-pose" alt="Cestart">
      <img src="https://mc-heads.net/player/Cestart/100" class="staff-skin-img hover-pose" alt="Cestart">
    </div>
    <div class="staff-info-box">
      <h3>Cestart</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('zarahoria')">
    <div class="staff-skin-wrapper">
      <img src="https://mc-heads.net/body/zarahoria/100" class="staff-skin-img normal-pose" alt="zarahoria">
      <img src="https://mc-heads.net/player/zarahoria/100" class="staff-skin-img hover-pose" alt="zarahoria">
    </div>
    <div class="staff-info-box">
      <h3>zarahoria</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
</div>
</div>

</div>

<!-- Incluir la librería de skinview3d para visualización 3D interactiva -->
<script src="https://unpkg.com/skinview3d@3.0.0/dist/bundle/skinview3d.bundle.js"></script>

<!-- Modal emergente para detalles de Staff -->
<div id="staffModal" class="staff-modal-overlay">
  <div class="staff-modal-card glass-surface">
    <button class="staff-modal-close" onclick="closeStaffModal()">×</button>
    <div class="staff-modal-left">
      <div class="badge-vip" style="align-self: flex-start;">DestinyTeam</div>
      <h2 id="modalName" style="margin: 8px 0 4px; font-size: 2rem; font-weight: 800;">Nombre</h2>
      <p id="modalDesc" style="color: #9c9ca4; line-height: 1.5; font-size: 0.95rem; margin: 0 0 16px;">Descripción del staff...</p>
      
      <div style="font-size:0.8rem; color:#888890; text-transform:uppercase; font-weight:700; margin-bottom: 8px;">Contacto y Redes</div>
      <div id="modalSocials" class="staff-modal-socials">
        <!-- Social links loaded dynamically -->
      </div>
    </div>
    <div class="staff-modal-right" id="modalSkinContainer">
      <!-- Aquí se creará el canvas de skinview3d -->
    </div>
  </div>
</div>

<script>
const staffData = {
  "ElGlower": {
    desc: "Fundador y desarrollador principal de DestinyNetwork. Encargado de la creación de sistemas del servidor y el portal web.",
    socials: [
      { type: "Discord", name: "@elglower", url: "#" },
      { type: "Twitter/X", name: "@ElGlower", url: "https://twitter.com/ElGlower" }
    ]
  },
  "ElBalam15": {
    desc: "Administrador general. Encargado de la gestión de la comunidad, soporte técnico y moderación del servidor.",
    socials: [
      { type: "Discord", name: "@elbalam15", url: "#" }
    ]
  },
  "espiral_": {
    desc: "Desarrollador y administrador de sistemas. Responsable de la infraestructura cloud y base de datos de DestinyNetwork.",
    socials: [
      { type: "Discord", name: "@espiral_", url: "#" }
    ]
  },
  "pilahd14": {
    desc: "Moderador y soporte. Encargado de mantener el orden y asegurar que todos jueguen de manera justa.",
    socials: [
      { type: "Discord", name: "@pilahd14", url: "#" }
    ]
  },
  "prismangames": {
    desc: "Gestor de comunidad y creador de eventos. Encargado de organizar dinámicas de la comunidad y redes sociales.",
    socials: [
      { type: "Discord", name: "@prismangames", url: "#" }
    ]
  },
  "sombradr": {
    desc: "Desarrollador web y de plugins. Diseñador de interfaces, experiencia de usuario y mecánicas personalizadas.",
    socials: [
      { type: "Discord", name: "@sombradr", url: "#" }
    ]
  },
  "ripkyn": {
    desc: "Control de calidad y moderación. Tester oficial de mapas, detección de bugs y gestión de reportes.",
    socials: [
      { type: "Discord", name: "@ripkyn", url: "#" }
    ]
  },
  "Cestart": {
    desc: "Administrador. Gestor de torneos especiales de The Towers y encargado de la coordinación interna de moderadores.",
    socials: [
      { type: "Discord", name: "@cestart", url: "#" }
    ]
  },
  "zarahoria": {
    desc: "Soporte comunitario y moderación. Encargada de asistir a los usuarios nuevos y velar por el buen chat.",
    socials: [
      { type: "Discord", name: "@zarahoria", url: "#" }
    ]
  }
};

let skinViewer = null;

function openStaffModal(username) {
  const data = staffData[username];
  if (!data) return;

  document.getElementById('modalName').innerText = username;
  document.getElementById('modalDesc').innerText = data.desc;

  const socialsContainer = document.getElementById('modalSocials');
  socialsContainer.innerHTML = '';
  data.socials.forEach(s => {
    const link = document.createElement('a');
    link.className = 'staff-social-link';
    link.href = s.url;
    if (s.url !== '#') link.target = '_blank';
    link.innerHTML = `<span class="material-symbols-rounded" style="font-size: 16px;">share</span> <strong>${s.type}:</strong> ${s.name}`;
    socialsContainer.appendChild(link);
  });

  document.getElementById('staffModal').classList.add('active');

  // Inicializar visor 3D con delay para esperar el renderizado del modal
  setTimeout(() => {
    try {
      if (skinViewer) {
        skinViewer.dispose();
      }

      const container = document.getElementById('modalSkinContainer');
      container.innerHTML = ''; // Limpiar canvas previo
      
      const canvas = document.createElement("canvas");
      container.appendChild(canvas);

      // Usar mineskin.eu para obtener el skin layout de forma segura
      skinViewer = new skinview3d.SkinViewer({
        canvas: canvas,
        width: 200,
        height: 250,
        skin: `https://mineskin.eu/skin/${username}`
      });

      // Configurar controles orbitales (arrastrar para rotar)
      const orbitControl = skinview3d.createOrbitControls(skinViewer);
      orbitControl.enableZoom = false;
      orbitControl.enablePan = false;

      // Animaciones y velocidad
      skinViewer.autoRotate = true;
      skinViewer.autoRotateSpeed = 0.6;

      // Aplicar animación de caminar de Minecraft
      skinViewer.animation = new skinview3d.WalkingAnimation();
      skinViewer.animation.speed = 0.7;

    } catch (e) {
      console.error("Error al cargar visor de skin 3D:", e);
      // Fallback a imagen estática si WebGL falla o no es compatible
      const container = document.getElementById('modalSkinContainer');
      container.innerHTML = `<img src="https://mc-heads.net/player/${username}/150" class="staff-modal-skin" alt="${username}">`;
    }
  }, 100);
}

function closeStaffModal() {
  document.getElementById('staffModal').classList.remove('active');
  if (skinViewer) {
    skinViewer.dispose();
    skinViewer = null;
  }
}

// Cerrar modal al hacer click fuera de la tarjeta
document.getElementById('staffModal').addEventListener('click', (e) => {
  if (e.target.id === 'staffModal') {
    closeStaffModal();
  }
});
</script>
"##
}
