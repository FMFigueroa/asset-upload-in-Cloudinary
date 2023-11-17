use axum::Router;
use routes::{data_routes, hello_routes};
use std::net::SocketAddr;

mod routes;
pub mod utils;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let routes_all = Router::new().merge(hello_routes().merge(data_routes()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("Server listening on http://{addr}\n");

    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .expect("server failed");

    Ok(())
}
