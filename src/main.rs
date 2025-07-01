use askama::Template;
use axum::{Router, response::Html, routing::get};
use std::net::SocketAddr;
use tower_http::{compression::CompressionLayer, services::ServeDir};
use z3_app::templates::MainTemplate;

/// Launches the Axum web server with HTML template rendering and static file serving.
///
/// Sets up application routes for the root path (`/`), a test page (`/test`), and static file serving at `/static`.
/// Binds to `127.0.0.1:3000` and serves requests asynchronously. Exits the process if the server encounters a runtime error.
///
/// # Examples
///
/// ```no_run
/// // Run the application by executing the binary.
/// // The server will be accessible at http://127.0.0.1:3000/
/// ```
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .nest_service("/static", ServeDir::new("static"))
        .layer(CompressionLayer::new());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{addr}");

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(addr).await.unwrap();
    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("Server error: {e}");
        std::process::exit(1);
    }
}

/// Handles requests to the root path (`/`) by rendering the main HTML template.
/// Returns a `Html` response containing the rendered template.
async fn root() -> Html<String> {
    let template = MainTemplate {
        // posts: vec![], // Uncomment and populate with actual data if needed
    };
    Html(template.render().unwrap())
}
