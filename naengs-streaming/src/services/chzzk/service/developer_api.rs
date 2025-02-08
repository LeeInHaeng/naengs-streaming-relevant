use axum::{body::Body, extract::Query, http::Response, response::IntoResponse};
use reqwest::StatusCode;

use crate::services::chzzk::models::developer_apis_model::OauthCallbackParam;

pub async fn oauthcallback(Query(params): Query<OauthCallbackParam>) -> impl IntoResponse {
    let body = Body::from(serde_json::to_string(&params).unwrap());

    return Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(body)
        .unwrap()
}