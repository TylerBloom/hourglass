use axum::{routing::get, Router};

mod assets;
use assets::*;

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(landing))
        .route("/hourglass-frontend_bg.wasm", get(get_wasm))
        .route("/hourglass-frontend.js", get(get_js));

    Ok(router.into())
}
