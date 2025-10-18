use crate::util;
use std::sync::OnceLock;
use tracing_subscriber::{
    filter::LevelFilter, fmt, layer::SubscriberExt, EnvFilter, Layer, Registry,
};

static LOG_GUARD: OnceLock<tracing_appender::non_blocking::WorkerGuard> = OnceLock::new();

pub fn init() {
    let want_json = std::env::var_os("GOOSTR_JSON").is_some();
    let log_dir = util::nostr_config_root().join("logs");
    let _ = std::fs::create_dir_all(&log_dir);

    let file_appender = tracing_appender::rolling::daily(log_dir, "goostr.log");
    let (file_nb, guard) = tracing_appender::non_blocking(file_appender);
    let _ = LOG_GUARD.set(guard);

    let file_layer = {
        let layer = fmt::layer().with_writer(file_nb).with_ansi(false);
        let layer = if want_json {
            layer.json().boxed()
        } else {
            layer.boxed()
        };
        layer.with_filter(LevelFilter::TRACE)
    };

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::default().add_directive(LevelFilter::INFO.into()));

    let stderr_layer = {
        let layer = fmt::layer().with_writer(std::io::stderr).with_ansi(true);
        let layer = if want_json {
            layer.json().with_ansi(false).boxed()
        } else {
            layer.boxed()
        };
        layer.with_filter(env_filter)
    };

    let base = Registry::default().with(file_layer);

    if std::env::var_os("GOOSTR_NO_STDERR").is_none() {
        let subscriber = base.with(stderr_layer);
        let _ = tracing::subscriber::set_global_default(subscriber);
    } else {
        let _ = tracing::subscriber::set_global_default(base);
    }
}
