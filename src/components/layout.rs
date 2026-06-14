use super::{styles, navbar, footer};

pub fn render_page(title: &str, active_route: &str, body: &str) -> String {
    format!(
        r##"<!DOCTYPE html>
<html lang="es">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no">
<title>{} | DestinyOwner</title>
<link rel="icon" type="image/png" href="/icon.png">
<meta property="og:title" content="Destiny Network - Estadísticas en Tiempo Real">
<meta property="og:description" content="Estadísticas en tiempo real, clasificaciones globales y perfiles del servidor Destiny Network.">
<meta property="og:image" content="/icon.png">
<meta property="og:type" content="website">
<meta name="twitter:card" content="summary_large_image">
<meta name="twitter:image" content="/icon.png">
<script src="/skinview3d.bundle.js"></script>
<style>{}</style>
</head>
<body>
{}
<main class="page-content">
{}
</main>
{}
</body>
</html>"##,
        title,
        styles::get_styles(),
        navbar::render(active_route),
        body,
        footer::render()
    )
}
