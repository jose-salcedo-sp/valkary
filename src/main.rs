use axum::{
    Router,
    body::{Body, to_bytes},
    http::Request,
};
use blog::{App, BASE_PATH, POSTS, shell};
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use std::path::Path;
use tower_service::Service;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let leptos_options = get_configuration(Some("Cargo.toml"))
        .map(|conf| conf.leptos_options)
        .unwrap_or_else(|_| LeptosOptions::builder().output_name("blog").build());
    let addr = leptos_options.site_addr;
    let site_root = leptos_options.site_root.to_string();
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    if std::env::args().any(|arg| arg == "--export") {
        return export_static_site(app, Path::new(&site_root)).await;
    }

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app.into_make_service()).await
}

async fn export_static_site(mut app: Router, site_root: &Path) -> Result<(), std::io::Error> {
    std::fs::create_dir_all(site_root)?;
    std::fs::write(site_root.join(".nojekyll"), "")?;

    let root_uri = if BASE_PATH.is_empty() {
        "/".to_string()
    } else {
        format!("{BASE_PATH}/")
    };
    render_to_file(&mut app, &root_uri, &site_root.join("index.html")).await?;
    render_to_file(
        &mut app,
        &format!("{BASE_PATH}/404"),
        &site_root.join("404.html"),
    )
    .await?;

    for post in POSTS {
        let uri = format!("{BASE_PATH}/posts/{}", post.slug);
        let out_dir = site_root.join("posts").join(post.slug);
        std::fs::create_dir_all(&out_dir)?;
        render_to_file(&mut app, &uri, &out_dir.join("index.html")).await?;
    }

    println!("exported static site to {}", site_root.display());
    Ok(())
}

async fn render_to_file(app: &mut Router, uri: &str, dest: &Path) -> Result<(), std::io::Error> {
    let req = Request::builder()
        .uri(uri)
        .body(Body::empty())
        .map_err(std::io::Error::other)?;
    let resp = app
        .as_service()
        .call(req)
        .await
        .map_err(std::io::Error::other)?;
    let bytes = to_bytes(resp.into_body(), usize::MAX)
        .await
        .map_err(std::io::Error::other)?;
    std::fs::write(dest, bytes)
}
