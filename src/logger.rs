use std::path::Path;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initializes asynchronous, non-blocking file logging to `dhabatui.log`.
/// Returns a `WorkerGuard` that must be held in `main()` until application shutdown
/// to ensure that all queued log events are flushed to disk.
pub fn init_logger<P: AsRef<Path>>(log_dir: P) -> Result<WorkerGuard, Box<dyn std::error::Error>> {
    let file_appender = tracing_appender::rolling::never(log_dir, "dhabatui.log");
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,dhabatui=debug"));

    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(file_layer)
        .try_init()?;

    tracing::info!("Logger initialized successfully. Logging to dhabatui.log");
    Ok(guard)
}
