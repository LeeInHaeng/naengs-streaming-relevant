use naengs_streaming::axum_run;

#[tokio::main]
async fn main() {
    match axum_run().await {
        Ok(_) => {},
        Err(err) => println!("axum run err: {}", err)
    };
}