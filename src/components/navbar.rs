pub fn render(active_route: &str) -> String {
    let home_active = if active_route == "/" { "active" } else { "" };
    let lb_active = if active_route == "/leaderboard" { "active" } else { "" };
    let staff_active = if active_route == "/staff" { "active" } else { "" };

    format!(
        r##"
<nav class="navbar">
<div class="container">
<a class="nav-brand" href="/"><img src="/icon.png" alt="DestinyOwner"></a>
<div class="nav-links">
<a class="{}" href="/">Inicio</a>
<a class="{}" href="/leaderboard">Leaderboard</a>
<a class="{}" href="/staff">Staff</a>
</div>
<div class="d-flex align-center gap-3">
<a href="/login" class="btn btn-primary">Iniciar Sesión</a>
<button class="mobile-menu-btn" onclick="toggleMobileMenu()"><span class="material-symbols-rounded">menu</span></button>
</div>
</div>
</nav>
<div class="mobile-menu" id="mobileMenu">
<a class="{}" href="/">Inicio</a>
<a class="{}" href="/leaderboard">Leaderboard</a>
<a class="{}" href="/staff">Staff</a>
<a href="/login">Iniciar Sesión</a>
</div>
<script>
function toggleMobileMenu() {{
  var menu = document.getElementById("mobileMenu");
  if (menu.style.display === "flex") {{
    menu.style.display = "none";
  }} else {{
    menu.style.display = "flex";
  }}
}}
</script>
"##,
        home_active, lb_active, staff_active, home_active, lb_active, staff_active
    )
}
