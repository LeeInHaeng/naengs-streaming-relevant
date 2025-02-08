use axum::{http::Response, response::IntoResponse};
use reqwest::StatusCode;

pub async fn hc() -> impl IntoResponse {
    return Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body("hc".to_string())
        .unwrap()
}