use std::net::SocketAddr;

use axum::{
    routing::{on, MethodFilter},
    Router,
};

#[tokio::main]
async fn main() {
    let router = Router::new()
        .route("/upload", on(MethodFilter::GET, upload))
        .route("/download", on(MethodFilter::GET, download));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();
}

async fn upload() -> Result<(), ()> {
    println!("Hello upload");

    Ok(())
}

async fn _put_file() -> Result<(), ()> {
    Ok(())
}

pub async fn download() -> Result<(), ()> {
    println!("Hello download");

    Ok(())
}
