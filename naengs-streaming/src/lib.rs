mod services;

use std::env;

use deadpool_postgres::{Config, Pool, Runtime};
use dotenv::dotenv;
use once_cell::sync::Lazy;
use services::create_routes;
use tokio::{signal, task::AbortHandle};
use tower_sessions::{cookie::{time::Duration, SameSite}, Expiry, SessionManagerLayer};
use tower_sessions_redis_store::{fred::prelude::{ClientLike, Config as RedisConfig, Pool as RedisPool}, RedisStore};
use tokio_postgres::NoTls;
use tracing::info;
use tracing_appender::rolling;

pub async fn axum_run() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // redis config
    // 혹시 window 환경 에서의 redis 사용 이라면 wsl 로 띄워야됨
    let redis_pool = REDIS_POOL.clone();
    let redis_connect = redis_pool.connect();
    redis_pool.wait_for_connect().await?;

    let redis_store = RedisStore::new(redis_pool);

    // redis session storage
    let session_layer = SessionManagerLayer::new(redis_store)
        // axum 과 sveltekit 의 도메인이 다를 수 있기 때문에 SameSite 는 None 으로 설정
        .with_same_site(SameSite::None)
        .with_expiry(Expiry::OnInactivity(Duration::hours(2)));

    // app routes
    let app = create_routes().layer(session_layer);

    // SSL 환경은 reverse proxy 방식 사용 (ex : ngingx)
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081")
        .await?;

    // log 관련
    let log_path = env::var("LOG_PATH").expect("LOG_PATH is not set");
    let log_file_appender = rolling::daily(log_path, "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(log_file_appender);

    tracing_subscriber::fmt()
        .json()
        .with_writer(non_blocking)
        .with_max_level(tracing::Level::DEBUG) // ERROR, WARN, INFO, DEBUG 출력 (DEBUG 너무 많이 찍히면 Level::INFO 로 변경)
        .init();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal(redis_connect.abort_handle()))
        .await?;

    redis_connect.await??;

    Ok(())
}

// DB 싱글톤 pool
pub static DB_POOL_REVIEW: Lazy<Pool> = Lazy::new(|| {
    let cfg = get_db_pool_config("POSTGRES", "STREAMING");
    cfg.create_pool(Some(Runtime::Tokio1), NoTls).expect("Failed to create db pool")
});

// redis 싱글톤 pool
pub static REDIS_POOL: Lazy<RedisPool> = Lazy::new(|| {
    RedisPool::new(RedisConfig::default(), None, None, None, 16)
        .expect("Failed to create redis pool")
});

fn get_db_pool_config(db_type: &str, db_name: &str) -> Config {
    let host = env::var(format!("{}_HOST_{}", db_type, db_name)).expect("DB_HOST is not set");
    let dbport = env::var(format!("{}_PORT_{}", db_type, db_name)).expect("DB_PORT is not set")
        .parse::<u16>()
        .unwrap();
    let dbname = env::var(format!("{}_NAME_{}", db_type, db_name)).expect("DB_NAME is not set");
    let user = env::var(format!("{}_USER_{}", db_type, db_name)).expect("DB_USER is not set");
    let password = env::var(format!("{}_PASSWORD_{}", db_type, db_name)).expect("DB_PASSWORD is not set");

    let mut cfg = Config::new();
    cfg.host = Some(host);
    cfg.port = Some(dbport);
    cfg.dbname = Some(dbname);
    cfg.user = Some(user);
    cfg.password = Some(password);

    cfg
}

async fn shutdown_signal(redis_abort: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { redis_abort.abort() },
        _ = terminate => { redis_abort.abort() },
    }
}