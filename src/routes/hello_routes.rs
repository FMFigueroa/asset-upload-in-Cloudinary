use axum::{
    extract::Query,
    response::{Html, IntoResponse, Json},
};
use serde::{Deserialize, Serialize};

//============================================================
#[derive(Debug, Serialize)]
struct User {
    name: String,
    age: u32,
}
#[derive(Debug, Serialize)]
struct Response {
    user: User,
}

// e.g., `/user`
pub async fn hello_user() -> impl IntoResponse {
    let user = User {
        name: "John Doe".to_string(),
        age: 30,
    };
    println!("->> {:<6} - hello_user - {user:?}", "HANDLER");
    Json(Response { user })
}
//============================================================
#[derive(Debug, Deserialize)]
pub struct HelloParams {
    name: Option<String>,
}

// e.g., `/hello?name=felix`
pub async fn hello_params(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<6} - hello_params - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("Params");
    Html(format!("<strong>Hello {name}!</strong>"))
}
//============================================================
pub async fn hello_world() -> impl IntoResponse {
    println!("->> {:<6} - hello_world", "HANDLER");
    Html(format!("<strong>Hello World</strong>"))
}
