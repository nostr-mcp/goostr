use anyhow::{Context, Result};
use tracing::{error, info};
use tracing_subscriber::{fmt, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    info!("Goostr runtime starting");

    if let Err(e) = run().await {
        error!(error = %e, "runtime error");
    }

    info!("Goostr runtime stopped");
    Ok(())
}

fn init_tracing() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_ansi(true)
        .init();
}

async fn run() -> Result<()> {
    shutdown_signal().await?;
    Ok(())
}

async fn shutdown_signal() -> Result<()> {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut term = signal(SignalKind::terminate())
            .context("install SIGTERM handler")?;
        tokio::select! {
            res = tokio::signal::ctrl_c() => { res.context("listen for Ctrl+C")?; }
            _ = term.recv() => {}
        }
    }

    #[cfg(not(unix))]
    {
        tokio::signal::ctrl_c().await.context("listen for Ctrl+C")?;
    }

    Ok(())
}
