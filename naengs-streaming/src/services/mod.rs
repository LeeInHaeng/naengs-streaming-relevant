mod chzzk;
mod common;

use std::{process::Command, time::Duration};

use axum::{body::Body, error_handling::HandleErrorLayer, http::{HeaderMap, HeaderValue, Response}, response::Html, routing::get_service, BoxError, Router};
use reqwest::{header, Client, ClientBuilder, StatusCode};
use serde::Serialize;
use tower::{timeout::TimeoutLayer, ServiceBuilder};
use tower_http::{compression::CompressionLayer, services::ServeDir};
use tracing::warn;

pub fn create_routes() -> Router {
    // sveltekit static files serve
    let static_files_service = get_service(ServeDir::new("_app"))
        .handle_error(|_| async { StatusCode::INTERNAL_SERVER_ERROR })
        .layer(
            ServiceBuilder::new()
                // content-encoding
                .layer(CompressionLayer::new())
                .map_response(|mut res: Response<Body>| {
                    // cache control max-age
                    res.headers_mut().insert(
                        header::CACHE_CONTROL,
                        HeaderValue::from_static("max-age=31536000"),
                    );
                    res
                }),
        );

    let default_timeout_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_timeout))
        .layer(TimeoutLayer::new(Duration::from_secs(30)))
        .timeout(Duration::from_secs(30));

    let sync_timeout_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(handle_timeout))
        .layer(TimeoutLayer::new(Duration::from_secs(150)))
        .timeout(Duration::from_secs(150));

    Router::new()
        .nest_service("/_app", static_files_service)
        .merge(common::get_routes().layer(sync_timeout_service))
        .merge(chzzk::get_routes().layer(default_timeout_service))

}

pub async fn handle_timeout(err: BoxError) -> Html<&'static str> {
    warn!("timeout: {}", err);
    Html("<h1>!!!!!!!!!!!!!!!!Timeout!!!!!!!!!!!!!!!!</h1>")
}

fn get_reqwest_client_builder() -> ClientBuilder {
    reqwest::Client
        ::builder()
        .timeout(std::time::Duration::from_secs(30))
}

pub async fn request_get_with_retry(url: &str, headers: HeaderMap, retry_count: i32) -> Result<String, reqwest::Error> {
    let client: Client;
    #[cfg(debug_assertions)]
    {
        client = get_reqwest_client_builder()
            .danger_accept_invalid_certs(true)
            .build()?;
    }
    #[cfg(not(debug_assertions))]
    {
        client = get_reqwest_client_builder()
            .build()?;
    }

    let mut repeat_count = 0;

    loop {
        let response = client
            .get(url)
            .headers(headers.clone())
            .send()
            .await?;
        if response.status().is_success() {
            return Ok(response.text().await?);
        }

        warn!("request_get_with_retry :: status fail. {} / {:?}", response.status(), response.text().await);

        if 0 < retry_count {
            repeat_count += 1;
            if retry_count <= repeat_count {
                return Ok("All http request fail".to_string());
            }
        }
    }
}

pub async fn request_post_with_retry<B>(url: &str, headers: HeaderMap, body: &B, retry_count: i32) -> Result<String, reqwest::Error>
where B: Serialize {
    let client: Client;
    #[cfg(debug_assertions)]
    {
        client = get_reqwest_client_builder()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(30))
            .build()?;
    }
    #[cfg(not(debug_assertions))]
    {
        client = get_reqwest_client_builder()
            .timeout(Duration::from_secs(30))
            .build()?;
    }

    let mut repeat_count = 0;

    loop {
        let response = client
            .post(url)
            .headers(headers.clone())
            .json(body)
            .send().await?;
        if response.status().is_success() {
            return Ok(response.text().await?);
        }

        warn!("request_post_with_retry :: status fail. {} / {:?}", response.status(), response.text().await);

        if 0 < retry_count {
            repeat_count += 1;
            if retry_count <= repeat_count {
                return Ok("All http request fail".to_string());
            }
        }
    }
}

pub async fn request_post_string_with_retry(url: &str, headers: HeaderMap, body: String, retry_count: i32) -> Result<String, reqwest::Error> {
    let client: Client;
    #[cfg(debug_assertions)]
    {
        client = get_reqwest_client_builder()
            .danger_accept_invalid_certs(true)
            .timeout(Duration::from_secs(30))
            .build()?;
    }
    #[cfg(not(debug_assertions))]
    {
        client = get_reqwest_client_builder()
            .timeout(Duration::from_secs(30))
            .build()?;
    }

    let mut repeat_count = 0;

    loop {
        let response = client
            .post(url)
            .headers(headers.clone())
            .body(body.clone())
            .send().await?;
        if response.status().is_success() {
            return Ok(response.text().await?);
        }

        warn!("request_post_string_with_retry :: status fail. {} / {:?}", response.status(), response.text().await);

        if 0 < retry_count {
            repeat_count += 1;
            if retry_count <= repeat_count {
                return Ok("All http request fail".to_string());
            }
        }
    }
}

// bash 스크립트 실행
pub async fn execute_script(command_path: &str) -> Result<(), String> {
    match Command::new("/bin/bash").arg(command_path).output() {
        Ok(v) => v,
        Err(err) => {
            warn!("execute_script :: command_path run error. {} / {}", command_path, err);
            return Err("err1".to_string());
        }
    };

    //if !script_output.status.success() {
    //    warn!("execute_script :: command_path run error. {} / {}", command_path, script_output.status);
    //    return Err("err2".to_string());
    //}

    Ok(())
}