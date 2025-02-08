mod service;
mod models;

use axum::{http::HeaderValue, routing::get, Router};
use reqwest::{header, Method};
use service::{developer_api::oauthcallback, hc::hc};
use tower_http::{cors::CorsLayer, services::ServeFile};

pub fn get_routes() -> Router {
    // layer 설정들

    // cors
    let cors = get_cors_layer();

    Router::new()
        .route("/chzzk/hc", get(hc))
        .route("/chzzk/oauthcallback", get(oauthcallback))

        .route_service("/chzzk/hello", ServeFile::new("assets/chzzk/hello.html"))

        .layer(cors)
}

fn get_cors_layer() -> CorsLayer {
    // allow_credentials 를 true 로 명시 해주기 위해 allow origin 직접 명시 필요
    let allowed_origins = vec![
        "http://localhost:5173".parse::<HeaderValue>().unwrap(),
        "https://streaming.naengs.dev".parse::<HeaderValue>().unwrap(),
    ];

    // allow_credentials 를 true 로 명시 해주기 위해 allow header 직접 명시 필요
    let allowed_headers =  vec![
        header::ACCEPT,
        header::AUTHORIZATION,
        header::CACHE_CONTROL,
        header::CONTENT_LANGUAGE,
        header::CONTENT_TYPE,
    ];

    CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST])
        .allow_origin(allowed_origins)
        .allow_headers(allowed_headers)
        .allow_credentials(true)
}