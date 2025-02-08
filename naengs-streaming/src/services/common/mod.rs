mod git;

use std::env;

use axum::{routing::post, Router};
use git::{git_webhook, GitHookState};

pub fn get_routes() -> Router {
    // layer 설정들

    let git_webhook_secret_token = match env::var("GIT_WEBHOOK_SECRET") {
        Ok(v) => v,
        Err(_) => {
            "".to_string()
        }
    };
    let git_webhook_state = GitHookState {
        secret: git_webhook_secret_token,
    };

    Router::new()
        .route("/git/webhook", post(git_webhook))
            .with_state(git_webhook_state)
}