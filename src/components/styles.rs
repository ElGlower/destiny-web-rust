pub fn get_styles() -> &'static str {
    r##"
@import url('https://fonts.googleapis.com/css2?family=Outfit:wght@400;500;700;800&display=swap');
@import url('https://fonts.googleapis.com/css2?family=Material+Symbols+Rounded:opsz,wght,FILL,GRAD@24,400,1,0');

:root {
  --md-sys-color-background: #0b0b0d;
  --md-sys-color-surface: rgba(28, 28, 32, 0.45);
  --md-sys-color-surface-container: rgba(45, 45, 52, 0.3);
  --md-sys-color-on-surface: #e2e2e6;
  --md-sys-color-primary: #d0bcff;
  --md-sys-color-on-primary: #381e72;
  --md-sys-color-outline: rgba(255, 255, 255, 0.08);
  
  --md-sys-shape-corner-extra-large: 28px;
  --md-sys-shape-corner-large: 20px;
  --md-sys-shape-corner-medium: 12px;
  --md-sys-shape-corner-full: 9999px;
  
  --md-sys-motion-easing-emphasized: cubic-bezier(0.2, 0, 0, 1);
  --glass-blur: blur(28px);
}

* { box-sizing: border-box; }
body {
  margin: 0; padding: 0;
  font-family: 'Outfit', sans-serif;
  background-color: var(--md-sys-color-background);
  color: var(--md-sys-color-on-surface);
  background-image: 
    radial-gradient(circle at 80% 10%, rgba(120, 100, 150, 0.15) 0%, transparent 50%),
    radial-gradient(circle at 10% 90%, rgba(80, 80, 100, 0.12) 0%, transparent 50%);
  background-attachment: fixed;
  min-height: 100vh;
  overflow-x: hidden;
}

/* Animations */
@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}
@keyframes slideUp {
  from { opacity: 0; transform: translateY(30px); }
  to { opacity: 1; transform: translateY(0); }
}

.fade-in { animation: fadeIn 0.8s var(--md-sys-motion-easing-emphasized) forwards; }
.slide-up { animation: slideUp 0.8s var(--md-sys-motion-easing-emphasized) forwards; }

/* Global Typography */
h1 { font-size: clamp(2.5rem, 7vw, 4.5rem); font-weight: 800; letter-spacing: -0.03em; margin: 0; line-height: 1.1; }
h2 { font-weight: 700; }
p.subtitle { font-size: clamp(1rem, 2.5vw, 1.35rem); color: #9c9ca4; margin-top: 12px; font-weight: 400; max-width: 600px; line-height: 1.5; }

/* Layout Grid & Containers */
.container { width: 100%; max-width: 1100px; margin: 0 auto; padding: 0 24px; }
.d-flex { display: flex; }
.flex-column { flex-direction: column; }
.align-center { align-items: center; }
.justify-between { justify-content: space-between; }
.justify-center { justify-content: center; }
.flex-wrap { flex-wrap: wrap; }
.gap-2 { gap: 8px; }
.gap-3 { gap: 16px; }
.mt-5 { margin-top: 48px; }
.text-center { text-align: center; }
.page-content { padding-bottom: 80px; }

/* Glass Surface Styling */
.glass-surface {
  background: var(--md-sys-color-surface);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border: 1px solid var(--md-sys-color-outline);
}

/* Navbar */
.navbar {
  position: sticky; top: 0; z-index: 100;
  padding: 16px 0;
  background: rgba(11, 11, 13, 0.65);
  backdrop-filter: var(--glass-blur);
  -webkit-backdrop-filter: var(--glass-blur);
  border-bottom: 1px solid var(--md-sys-color-outline);
}
.navbar .container { display: flex; align-items: center; justify-content: space-between; }
.nav-brand img { height: 38px; border-radius: 8px; transition: transform 0.4s var(--md-sys-motion-easing-emphasized); }
.nav-brand img:hover { transform: scale(1.06); }
.nav-links { display: flex; gap: 28px; }
.nav-links a {
  color: #a0a0a8; text-decoration: none; font-weight: 600; font-size: 0.95rem;
  transition: all 0.3s var(--md-sys-motion-easing-emphasized);
  padding: 8px 16px; border-radius: var(--md-sys-shape-corner-full);
}
.nav-links a:hover { color: var(--md-sys-color-on-surface); background: rgba(255,255,255,0.05); }
.nav-links a.active { color: var(--md-sys-color-primary); background: rgba(208, 188, 255, 0.1); }

/* Buttons */
.btn {
  display: inline-flex; align-items: center; justify-content: center; gap: 10px;
  padding: 14px 28px; border-radius: var(--md-sys-shape-corner-full);
  font-family: inherit; font-weight: 600; font-size: 0.95rem;
  border: none; cursor: pointer; text-decoration: none;
  transition: all 0.4s var(--md-sys-motion-easing-emphasized);
}
.btn .material-symbols-rounded { font-size: 20px; }
.btn-primary { background: var(--md-sys-color-primary); color: var(--md-sys-color-on-primary); }
.btn-primary:hover { transform: scale(1.03) translateY(-2px); box-shadow: 0 12px 24px rgba(208, 188, 255, 0.22); }
.btn-glass {
  background: var(--md-sys-color-surface-container); color: var(--md-sys-color-on-surface);
  border: 1px solid var(--md-sys-color-outline);
}
.btn-glass:hover { background: rgba(255, 255, 255, 0.08); transform: scale(1.03); }
.btn-glass.active { background: var(--md-sys-color-on-surface); color: var(--md-sys-color-background); }

/* Mobile Menu */
.mobile-menu-btn { display: none; background: transparent; border: none; color: var(--md-sys-color-on-surface); font-size: 28px; cursor: pointer; }
.mobile-menu {
  display: none; flex-direction: column; background: rgba(11, 11, 13, 0.95);
  backdrop-filter: var(--glass-blur); border-bottom: 1px solid var(--md-sys-color-outline);
  padding: 20px; gap: 16px; position: fixed; top: 70px; left: 0; width: 100%; z-index: 99;
}
.mobile-menu a {
  color: #a0a0a8; text-decoration: none; font-weight: 600; font-size: 1.1rem;
  padding: 12px; border-radius: var(--md-sys-shape-corner-medium);
}
.mobile-menu a.active { color: var(--md-sys-color-primary); background: rgba(208, 188, 255, 0.1); }

/* Home / Hero Page */
.hero-section { padding: 60px 0 20px; }
.ip-card-wrapper { display: flex; justify-content: center; width: 100%; }
.ip-card {
  width: 100%; max-width: 500px; padding: 20px;
  border-radius: var(--md-sys-shape-corner-large);
  box-shadow: 0 16px 32px rgba(0,0,0,0.3);
  transition: transform 0.4s var(--md-sys-motion-easing-emphasized);
}
.ip-card:hover { transform: translateY(-4px); }

.features-grid {
  display: grid; grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 24px; margin-top: 64px;
}
.feature-card {
  padding: 36px 24px; border-radius: var(--md-sys-shape-corner-large);
  text-align: center;
  transition: all 0.4s var(--md-sys-motion-easing-emphasized);
}
.feature-card:hover { transform: translateY(-8px); background: rgba(255,255,255,0.03); }
.feature-icon { font-size: 40px; color: var(--md-sys-color-primary); margin-bottom: 16px; }
.feature-card h3 { font-size: 1.4rem; margin: 0 0 12px; font-weight: 700; }
.feature-card p { color: #9c9ca4; margin: 0; line-height: 1.6; }

/* Leaderboard Page */
.tabs-scroll { display: flex; justify-content: center; gap: 12px; }
.leaderboard-card {
  border-radius: var(--md-sys-shape-corner-extra-large);
  padding: 32px;
  box-shadow: 0 32px 64px rgba(0,0,0,0.5);
}
.search-bar {
  background: rgba(0,0,0,0.25); border: 1px solid var(--md-sys-color-outline);
  border-radius: var(--md-sys-shape-corner-full);
  padding: 12px 20px; display: flex; align-items: center; gap: 12px;
  transition: all 0.3s var(--md-sys-motion-easing-emphasized);
  width: 100%; max-width: 320px;
}
.search-bar:focus-within { border-color: var(--md-sys-color-primary); box-shadow: 0 0 0 4px rgba(208, 188, 255, 0.15); }
.search-bar input {
  background: transparent; border: none; color: var(--md-sys-color-on-surface);
  font-family: inherit; font-size: 0.95rem; outline: none; width: 100%;
}
.search-bar .material-symbols-rounded { color: #a0a0a8; }

.table-wrapper { overflow-x: auto; margin-top: 24px; }
table { width: 100%; border-collapse: collapse; text-align: left; }
th { color: #a0a0a8; font-weight: 600; font-size: 0.8rem; text-transform: uppercase; letter-spacing: 0.08em; padding: 18px 16px; border-bottom: 1px solid var(--md-sys-color-outline); }
td { padding: 18px 16px; font-size: 1rem; font-weight: 500; border-bottom: 1px solid rgba(255,255,255,0.03); }
tr { transition: background 0.3s var(--md-sys-motion-easing-emphasized); }
tr.clickable-row { cursor: pointer; }
tr:hover td { background: rgba(255,255,255,0.04); }
tr:hover td:first-child { border-top-left-radius: 12px; border-bottom-left-radius: 12px; }
tr:hover td:last-child { border-top-right-radius: 12px; border-bottom-right-radius: 12px; }

.player-profile { display: flex; align-items: center; gap: 12px; }
.player-head {
  width: 28px; height: 28px; border-radius: 6px;
  box-shadow: 0 4px 8px rgba(0,0,0,0.3);
  transition: transform 0.3s var(--md-sys-motion-easing-emphasized);
}
tr:hover .player-head { transform: scale(1.18) rotate(4deg); }
.rating-val { font-weight: 700; color: var(--md-sys-color-primary); }
.rank-badge { display: inline-flex; align-items: center; justify-content: center; width: 28px; height: 28px; background: rgba(255,255,255,0.06); border-radius: 50%; font-weight: 700; font-size: 0.85rem; }

/* Staff Page */
.staff-rank-title { font-size: 1.8rem; margin: 48px 0 24px; text-align: center; color: var(--md-sys-color-primary); letter-spacing: -0.01em; }
.staff-grid {
  display: grid; grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  gap: 24px; justify-content: center;
}
.staff-card {
  padding: 32px 24px; border-radius: var(--md-sys-shape-corner-large);
  text-align: center; display: flex; flex-direction: column; align-items: center;
  transition: all 0.4s var(--md-sys-motion-easing-emphasized);
}
.staff-card:hover { transform: translateY(-6px); background: rgba(255,255,255,0.04); }
.staff-skin-img {
  height: 120px; object-fit: contain; margin-bottom: 16px;
  filter: drop-shadow(0 10px 15px rgba(0,0,0,0.5));
  transition: transform 0.4s var(--md-sys-motion-easing-emphasized);
}
.staff-card:hover .staff-skin-img { transform: scale(1.1) rotate(-2deg); }
.staff-card h3 { font-size: 1.3rem; margin: 0 0 8px; font-weight: 700; }
.rank {
  display: inline-block; padding: 6px 14px; border-radius: var(--md-sys-shape-corner-full);
  font-size: 0.8rem; font-weight: 700; text-transform: uppercase; letter-spacing: 0.05em;
}
.rank.admin { background: rgba(239, 83, 80, 0.15); color: #ef5350; }
.rank.developer { background: rgba(41, 182, 246, 0.15); color: #29b6f6; }
.rank.moderator { background: rgba(102, 187, 106, 0.15); color: #66bb6a; }

/* Responsive Mobile Rules */
@media (max-width: 768px) {
  .nav-links { display: none; }
  .mobile-menu-btn { display: block; }
  .features-grid { grid-template-columns: 1fr; }
  .leaderboard-card { padding: 20px; border-radius: var(--md-sys-shape-corner-large); }
  th:nth-child(3), td:nth-child(3) { display: none; }
  .staff-grid { grid-template-columns: 1fr; }
}

footer { text-align: center; padding: 48px 24px; margin-top: 80px; border-top: 1px solid var(--md-sys-color-outline); color: #7c7c84; font-size: 0.9rem; }
footer a { color: var(--md-sys-color-on-surface); text-decoration: none; margin: 0 12px; font-weight: 600; transition: color 0.3s; }
footer a:hover { color: var(--md-sys-color-primary); }
footer p { margin-top: 16px; line-height: 1.5; }

/* Stats Profile Card & Events */
.profile-container { display: flex; flex-direction: column; gap: 32px; margin-top: 32px; }
.profile-card { border-radius: var(--md-sys-shape-corner-extra-large); padding: 40px; }
.profile-header-layout {
  display: grid;
  grid-template-columns: 220px 1fr 280px;
  gap: 36px;
  align-items: center;
}
.profile-avatar-wrapper {
  background: rgba(255,255,255,0.03); border: 1px solid var(--md-sys-color-outline);
  border-radius: var(--md-sys-shape-corner-large); padding: 24px;
  display: flex; justify-content: center; align-items: center;
  box-shadow: inset 0 0 20px rgba(0,0,0,0.3);
  width: 100%;
}
.profile-skin {
  height: 160px; object-fit: contain;
  filter: drop-shadow(0 10px 15px rgba(0,0,0,0.6));
  transition: transform 0.4s var(--md-sys-motion-easing-emphasized);
}
.profile-card:hover .profile-skin { transform: scale(1.1) rotate(2deg); }

.profile-info { flex: 1; display: flex; flex-direction: column; gap: 12px; }
.badge-vip {
  align-self: flex-start; background: rgba(208, 188, 255, 0.15); color: var(--md-sys-color-primary);
  padding: 6px 14px; border-radius: var(--md-sys-shape-corner-full);
  font-weight: 700; font-size: 0.8rem; text-transform: uppercase; letter-spacing: 0.05em;
  border: 1px solid rgba(208, 188, 255, 0.2);
}
.profile-name { font-size: 3rem; font-weight: 800; margin: 0; letter-spacing: -0.02em; }
.profile-status { display: flex; align-items: center; gap: 8px; color: #a0a0a8; margin: 0; font-size: 0.95rem; }
.status-dot { width: 8px; height: 8px; background: #66bb6a; border-radius: 50%; display: inline-block; box-shadow: 0 0 10px #66bb6a; animation: pulse 2s infinite; }
@keyframes pulse {
  0% { transform: scale(0.95); box-shadow: 0 0 0 0 rgba(102, 187, 106, 0.7); }
  70% { transform: scale(1); box-shadow: 0 0 0 6px rgba(102, 187, 106, 0); }
  100% { transform: scale(0.95); box-shadow: 0 0 0 0 rgba(102, 187, 106, 0); }
}

.quick-stats-row { display: flex; gap: 16px; margin-top: 16px; flex-wrap: wrap; }
.quick-stat-box {
  background: rgba(0,0,0,0.2); border: 1px solid var(--md-sys-color-outline);
  border-radius: var(--md-sys-shape-corner-large); padding: 16px 24px;
  display: flex; flex-direction: column; align-items: center; flex: 1; min-width: 100px;
}
.quick-stat-box .num { font-size: 1.8rem; font-weight: 800; color: var(--md-sys-color-on-surface); }
.quick-stat-box .lbl { font-size: 0.8rem; color: #888890; text-transform: uppercase; font-weight: 600; margin-top: 4px; }

/* Timeline Events */
.events-card { border-radius: var(--md-sys-shape-corner-extra-large); padding: 40px; }
.section-title { font-size: 1.8rem; font-weight: 800; margin: 0 0 32px; display: flex; align-items: center; gap: 12px; }
.section-title .material-symbols-rounded { font-size: 32px; color: var(--md-sys-color-primary); }

.events-timeline { display: flex; flex-direction: column; gap: 20px; }
.event-item {
  border-radius: var(--md-sys-shape-corner-large); padding: 24px;
  display: flex; gap: 24px; align-items: flex-start;
  transition: all 0.3s var(--md-sys-motion-easing-emphasized);
}
.event-item:hover { transform: translateX(6px); background: rgba(255,255,255,0.03); }
.event-meta { display: flex; flex-direction: column; gap: 6px; min-width: 140px; }
.event-badge {
  display: inline-flex; align-items: center; justify-content: center;
  padding: 6px 12px; border-radius: var(--md-sys-shape-corner-full);
  font-size: 0.8rem; font-weight: 700; text-transform: uppercase;
}
.event-badge.gold { background: rgba(255, 215, 0, 0.15); color: #ffd700; border: 1px solid rgba(255, 215, 0, 0.2); }
.event-badge.bronze { background: rgba(205, 127, 50, 0.15); color: #cd7f32; border: 1px solid rgba(205, 127, 50, 0.2); }
.event-date { font-size: 0.85rem; color: #888890; font-weight: 600; text-align: center; }

.event-details { flex: 1; }
.event-details h3 { font-size: 1.25rem; margin: 0 0 8px; font-weight: 700; }
.event-details p { color: #9c9ca4; margin: 0 0 16px; font-size: 0.95rem; line-height: 1.5; }

.event-stats-chips { display: flex; gap: 12px; flex-wrap: wrap; }
.chip {
  display: inline-flex; align-items: center; gap: 6px;
  background: rgba(0,0,0,0.25); border: 1px solid var(--md-sys-color-outline);
  padding: 6px 14px; border-radius: var(--md-sys-shape-corner-full);
  font-size: 0.85rem; font-weight: 600; color: #d2d2d6;
}
.chip .material-symbols-rounded { font-size: 16px; color: var(--md-sys-color-primary); }

.profile-chart-wrapper {
  flex: 1;
  min-width: 250px;
  display: flex;
  justify-content: center;
  align-items: center;
}
.radar-chart {
  width: 100%;
  max-width: 260px;
  height: auto;
  filter: drop-shadow(0 8px 16px rgba(0,0,0,0.3));
}

@media (max-width: 992px) {
  .profile-header-layout {
    grid-template-columns: 1fr;
    text-align: center;
    gap: 32px;
    justify-items: center;
  }
  .profile-avatar-wrapper { width: 100%; max-width: 240px; }
  .badge-vip { align-self: center; }
  .quick-stats-row { width: 100%; }
  .event-item { flex-direction: column; gap: 16px; }
  .event-meta { min-width: auto; flex-direction: row; justify-content: space-between; width: 100%; align-items: center; }
}
"##
}
