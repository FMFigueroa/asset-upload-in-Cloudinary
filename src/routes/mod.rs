use axum::{
    extract::DefaultBodyLimit,
    routing::{get, post},
    Router,
};
use data_routes::{form_data, upload_asset};
use hello_routes::{hello_params, hello_user, hello_world};

mod data_routes;
mod hello_routes;

pub fn hello_routes() -> Router {
    Router::new()
        .route("/", get(hello_world))
        .route("/user", get(hello_user))
        .route("/hello", get(hello_params))
}

pub fn data_routes() -> Router {
    Router::new()
        .route("/form-data", post(form_data))
        .route("/upload", post(upload_asset))
        // The default axum body size limit is 2MiB, so we increase it to 1GiB.
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024))
}
