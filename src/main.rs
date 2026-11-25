use axum::{
    Router,
    body::{Body, to_bytes},
    http::Request,
};
use blog::{App, BASE_PATH, all_posts, shell};
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

    let _ = copy_public_assets(Path::new("public"), Path::new(&site_root));

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
    copy_public_assets(Path::new("public"), site_root)?;

    let root_uri = if BASE_PATH.is_empty() {
        "/".to_string()
    } else {
        format!("{BASE_PATH}/")
    };
    render_to_file(&mut app, &root_uri, &site_root.join("index.html")).await?;

    let blog_dir = site_root.join("blog");
    std::fs::create_dir_all(&blog_dir)?;
    render_to_file(
        &mut app,
        &format!("{BASE_PATH}/blog"),
        &blog_dir.join("index.html"),
    )
    .await?;

    render_to_file(
        &mut app,
        &format!("{BASE_PATH}/404"),
        &site_root.join("404.html"),
    )
    .await?;

    for (series, post) in all_posts() {
        let uri = format!("{BASE_PATH}/posts/{}/{}", series.slug, post.slug);
        let out_dir = site_root.join("posts").join(series.slug).join(post.slug);
        std::fs::create_dir_all(&out_dir)?;
        render_to_file(&mut app, &uri, &out_dir.join("index.html")).await?;
    }

    println!("exported static site to {}", site_root.display());
    Ok(())
}

fn copy_public_assets(public_dir: &Path, site_root: &Path) -> Result<(), std::io::Error> {
    let pkg_dir = site_root.join("pkg");
    let wasm = pkg_dir.join("blog.wasm");
    let wasm_bg = pkg_dir.join("blog_bg.wasm");
    if wasm.is_file() && !wasm_bg.exists() {
        let _ = std::fs::copy(&wasm, &wasm_bg);
    }
    if !public_dir.exists() {
        return Ok(());
    }
    std::fs::create_dir_all(site_root)?;
    for entry in std::fs::read_dir(public_dir)? {
        let entry = entry?;
        let src_path = entry.path();
        let dest_path = site_root.join(entry.file_name());
        if src_path.is_dir() {
            copy_public_assets(&src_path, &dest_path)?;
        } else if src_path.is_file() {
            std::fs::copy(&src_path, &dest_path)?;
        }
    }
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
