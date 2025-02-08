use axum::{extract::State, http::HeaderMap, response::IntoResponse};
use reqwest::StatusCode;
use serde_json::Value;
use sha2::Sha256;
use tracing::{info, warn};
use hmac::{Hmac, Mac};

use crate::services::execute_script;

type HmacSha256 = Hmac<Sha256>;

#[derive(Clone)]
pub struct GitHookState {
    pub secret: String
}

pub async fn git_webhook(
    State(state): State<GitHookState>,
    headers: HeaderMap,
    body: String,
) -> impl IntoResponse {
    
    // X-Hub-Signature-256
    let signature_header = match headers.get("X-Hub-Signature-256") {
        Some(v) => {
            match v.to_str() {
                Ok(v) => v,
                Err(err) => {
                    warn!("git_webhook :: X-Hub-Signature-256 header to_str error. {}", err);
                    return (StatusCode::FORBIDDEN, format!("Forbidden!")).into_response();                                
                }
            }
        },
        None => {
            warn!("git_webhook :: X-Hub-Signature-256 header is none");
            return (StatusCode::FORBIDDEN, format!("Forbidden!")).into_response();            
        }
    };

    // hmac
    let mut mac = match HmacSha256::new_from_slice(state.secret.as_bytes()) {
        Ok(v) => {
            v
        },
        Err(err) => {
            warn!("git_webhook :: HmacSha256 new_from_slice err: {}", err);
            return (StatusCode::FORBIDDEN, format!("Forbidden!")).into_response();
        }
    };
    mac.update(body.as_bytes());

    // 해시 비교
    let expected_signature = format!("sha256={:x}", mac.finalize().into_bytes());
    if expected_signature != signature_header {
        warn!("git_webhook :: expected_signature signature_header not equal. {} / {}", expected_signature, signature_header);
        return (StatusCode::FORBIDDEN, format!("Forbidden!")).into_response();
    }

    // JSON 페이로드 파싱
    let payload: Value = serde_json::from_slice(body.as_bytes()).unwrap();
    if let Some(branch) = payload.get("ref").and_then(|r| r.as_str()) {
        if branch.ends_with("main") {
            // 작업을 비동기로 처리
            tokio::spawn(async {
                match execute_script("/axum_project/naengs-streaming-relevant/naengs-streaming/direct_build.sh").await {
                    Ok(_)  => {
                        info!("git_webhook :: script execute success");
                    },
                    Err(err) => {
                        warn!("git_webhook :: script execute fail: {}", err);
                    }
                }
            });
        }
    }

    return (StatusCode::OK, format!("success")).into_response();
}