pub fn render() -> &'static str {
    r##"
<div class="container mt-5 fade-in">
<div class="d-flex flex-column align-center text-center">
<h1 class="slide-up">Nuestro Equipo</h1>
<p class="subtitle slide-up">Los encargados de mantener y mejorar DestinyOwners a diario.</p>
</div>

<div class="staff-section mt-5">
<h2 class="staff-rank-title slide-up">DestinyTeam</h2>
<div class="staff-grid">
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('ElGlower')">
    <div class="staff-skin-3d-wrapper" id="skin3d-ElGlower"></div>
    <div class="staff-info-box">
      <h3>ElGlower</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('ElBalam15')">
    <div class="staff-skin-3d-wrapper" id="skin3d-ElBalam15"></div>
    <div class="staff-info-box">
      <h3>ElBalam15</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('espiral_')">
    <div class="staff-skin-3d-wrapper" id="skin3d-espiral_"></div>
    <div class="staff-info-box">
      <h3>espiral_</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('pilahd14')">
    <div class="staff-skin-3d-wrapper" id="skin3d-pilahd14"></div>
    <div class="staff-info-box">
      <h3>pilahd14</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('prismangames')">
    <div class="staff-skin-3d-wrapper" id="skin3d-prismangames"></div>
    <div class="staff-info-box">
      <h3>prismangames</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('sombradr')">
    <div class="staff-skin-3d-wrapper" id="skin3d-sombradr"></div>
    <div class="staff-info-box">
      <h3>sombradr</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('ripkyn')">
    <div class="staff-skin-3d-wrapper" id="skin3d-ripkyn"></div>
    <div class="staff-info-box">
      <h3>ripkyn</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('Cestart')">
    <div class="staff-skin-3d-wrapper" id="skin3d-Cestart"></div>
    <div class="staff-info-box">
      <h3>Cestart</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
  <div class="staff-card glass-surface slide-up" onclick="openStaffModal('zarahoria')">
    <div class="staff-skin-3d-wrapper" id="skin3d-zarahoria"></div>
    <div class="staff-info-box">
      <h3>zarahoria</h3>
      <p class="rank team">DestinyTeam</p>
    </div>
  </div>
</div>
</div>

</div>

<!-- Incluir la librería de skinview3d para visualización 3D interactiva -->
<script src="/skinview3d.bundle.js"></script>

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
    desc: "Diseñador de DestinyOwners.No description .",
    socials: [
      { type: "Discord", name: "@elbalam15", url: "#" },
      { type: "Twitter/X", name: "@elbalam15", url: "https://x.com/ElBalam15" }
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

      skinViewer = new skinview3d.SkinViewer({
        canvas: canvas,
        width: 200,
        height: 250,
        skin: `/api/skin/${username}`
      });

      // Configurar controles orbitales (incorporados en skinview3d v3.x)
      skinViewer.controls.enableZoom = false;
      skinViewer.controls.enablePan = false;

      // Animaciones y velocidad
      skinViewer.autoRotate = true;
      skinViewer.autoRotateSpeed = 0.6;

      // Aplicar animación de saludar (WaveAnimation) al abrir
      skinViewer.animation = new skinview3d.WaveAnimation("right");
      skinViewer.animation.speed = 1.0;

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

const viewers = {};

function initListSkins() {
  const usernames = ["ElGlower", "ElBalam15", "espiral_", "pilahd14", "prismangames", "sombradr", "ripkyn", "Cestart", "zarahoria"];
  
  usernames.forEach(username => {
    try {
      const container = document.getElementById(`skin3d-${username}`);
      if (!container) return;
      
      const canvas = document.createElement("canvas");
      container.appendChild(canvas);
      
      const viewer = new skinview3d.SkinViewer({
        canvas: canvas,
        width: 80,
        height: 100,
        skin: `/api/skin/${username}`
      });
      
      // Ajustar posición inicial de frente
      viewer.camera.position.x = 0;
      viewer.camera.position.y = 0;
      viewer.camera.position.z = 15;
      
      // Estático por defecto (sin rotar)
      viewer.autoRotate = false;
      viewer.animation = null;
      
      viewers[username] = viewer;
      
      // Agregar eventos hover sobre la tarjeta
      const card = container.closest('.staff-card');
      if (card) {
        card.addEventListener('mouseenter', () => {
          // Cambiar pose/animación al pasar el mouse (saludar, nadar o correr)
          if (username === "ElBalam15" || username === "prismangames" || username === "ripkyn") {
            viewer.animation = new skinview3d.SwimAnimation();
          } else if (username === "espiral_") {
            viewer.animation = new skinview3d.WalkingAnimation();
            viewer.animation.speed = 1.5; // Correr
          } else if (username === "pilahd14" || username === "zarahoria") {
            viewer.animation = new skinview3d.WaveAnimation("left");
            viewer.animation.speed = 1.5;
          } else {
            viewer.animation = new skinview3d.WaveAnimation("right");
            viewer.animation.speed = 1.5;
          }
          viewer.autoRotate = false;
        });
        
        card.addEventListener('mouseleave', () => {
          // Retornar a estático (sin animación)
          viewer.animation = null;
          viewer.autoRotate = false;
          viewer.camera.position.x = 0;
          viewer.camera.position.y = 0;
          viewer.camera.position.z = 15;
        });
      }
      
    } catch(e) {
      console.error(`Error al inicializar skin 3D de ${username}:`, e);
      const container = document.getElementById(`skin3d-${username}`);
      if (container) {
        container.innerHTML = `<img src="https://mc-heads.net/body/${username}/80" class="staff-skin-img" style="height: 80px;" alt="${username}">`;
      }
    }
  });
}

// Polling de inicialización seguro para evitar race condition con script CDN
function checkAndInit() {
  if (typeof skinview3d !== 'undefined') {
    initListSkins();
  } else {
    setTimeout(checkAndInit, 50);
  }
}

checkAndInit();
</script>
"##
}
