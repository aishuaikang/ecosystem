use std::time::Duration;

use axum::{routing::get, Router};
use tokio::{
    net::TcpListener,
    time::{self, sleep},
};
use tracing::{debug, info, instrument, level_filters::LevelFilter, warn};
use tracing_subscriber::{
    fmt::format::FmtSpan, layer::SubscriberExt, util::SubscriberInitExt, Layer,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file_appender = tracing_appender::rolling::daily("/tmp/logs", "ecosystem.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // 此方式不可细微控制
    // tracing_subscriber::fmt()
    //     .with_writer(non_blocking)
    //     .with_span_events(FmtSpan::CLOSE)
    //     .pretty()
    //     .with_max_level(Level::INFO)
    //     .init();

    let append_file = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .pretty()
        .with_filter(LevelFilter::INFO);

    let console = tracing_subscriber::fmt::layer()
        .pretty()
        .with_span_events(FmtSpan::CLOSE)
        .with_filter(LevelFilter::TRACE);

    tracing_subscriber::registry()
        .with(console)
        .with(append_file)
        .init();

    let addr = "0.0.0.0:8080";
    info!(target: "listening on", ip = ?addr);

    let app = Router::new().route("/", get(index_handler));

    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;

    Ok(())
}

#[instrument]
async fn index_handler() -> &'static str {
    debug!("index handler started");
    sleep(Duration::from_millis(10)).await;
    info!(http.status = 200, "index handler completed");
    long_task().await
}

#[instrument]
async fn long_task() -> &'static str {
    let start_time = time::Instant::now();
    sleep(Duration::from_millis(112)).await;
    let dur = start_time.elapsed().as_millis();
    warn!(app.task_duration = dur, "task takes too long");
    "Hello, World!"
}
