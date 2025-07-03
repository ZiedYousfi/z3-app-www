# z3 Stack – Promotional Site

This repository contains the promotional website for the **z3 Stack**: a modern Rust full-stack web app template.

- **Axum**: Fast, async web framework for Rust.
- **Askama**: Compile-time checked HTML templates (server-side rendering).
- **HTMX**: Progressive enhancement and dynamic content loading with minimal JavaScript.
- **Tailwind CSS**: Utility-first CSS for rapid, beautiful UI development.
- **Static assets**: Serves static files (including favicon) from the `/static` directory.

## What is this?

This is the official landing and promotional site for the z3 Stack template. It demonstrates:

- **Simplicity**: No SPA, no frontend build step, no JS frameworks—just Rust, HTML, and a sprinkle of HTMX.
- **Modern UI**: Beautiful, responsive design out of the box (see `templates/html/main.html`).
- **Server-side rendering**: All HTML is rendered on the server using Askama templates.
- **Minimal JS**: Only HTMX is included for interactivity; no client-side framework bloat.
- **Easy static file serving**: Place your assets (CSS, JS, images, favicon) in `/static`.

## Quick Start

1. **Clone the repository**

   ```bash
   git clone https://github.com/ZiedYousfi/z3-app-www.git
   cd z3-app-www
   ```

2. **Build and run**

   ```bash
   cargo run
   ```

3. **Open in your browser**

   Visit [http://localhost:3000](http://localhost:3000)

## Project Structure

- `src/main.rs` – Axum server setup and route definitions
- `src/templates/` – Askama template structs
- `templates/html/main.html` – Main HTML template (modern, Tailwind-powered UI)
- `static/` – Static files (e.g., favicon, CSS, JS)

## Features

- Modern, animated landing page UI (see the template for details)
- Favicon support (served from `/static/favicon.png`)
- HTMX for dynamic content loading (see the login card demo in the template)
- Ready for extension: add routes, templates, and static assets as needed

## What this is NOT (yet)

- No authentication, database, or ORM is included by default (but you can add them)
- No Diesel or Axum-login setup out of the box
- No Docker or .env files in this minimal template (add as needed)

## License

Licensed under the Apache License 2.0. See [LICENSE.md](LICENSE.md) for details.

---

Crafted with ❤️ & Rust by [Zied Yousfi](https://github.com/ZiedYousfi)
