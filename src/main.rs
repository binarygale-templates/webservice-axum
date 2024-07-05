use anyhow::Context;
use clap::Parser;
use tokio::net::TcpListener;
use tracing::info;

use webservice_axum::{routers::build_main_router, settings::LogFormat, AppState, Settings};

/// Sets up a relevant shutdown signals. This will exit on either SIGINT
/// (aka Ctrl+C) or SIGTERM.
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to create Ctrl+C handler");
    };

    let sigterm = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to create SIGTERM handler")
            .recv()
            .await;
    };

    tokio::select! {
        () = ctrl_c => {},
        () = sigterm => {},
    }

    info!("shutdown signal received")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = Settings::parse();
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(settings.log_level.tracing_level())
        .with_target(false);
    match settings.log_format {
        LogFormat::Text => subscriber.with_ansi(false).init(),
        LogFormat::TextColor => subscriber.with_ansi(true).init(),
        LogFormat::Json => subscriber.json().with_span_list(false).init(),
    }

    let router = build_main_router(AppState {
        settings: settings.clone(),
    });

    let listener = TcpListener::bind(settings.listen)
        .await
        .context(format!("could not listen to `{}`", settings.listen))?;

    info!("starting server on `{}`", settings.listen);
    axum::serve(listener, router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("failed to start server")?;

    Ok(())
}
