use super::{styles, navbar, footer};

pub fn render_page(title: &str, active_route: &str, body: &str) -> String {
    format!(
        r##"<!DOCTYPE html>
<html lang="es">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1, maximum-scale=1, user-scalable=no">
<title>{} | DestinyOwner</title>
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
