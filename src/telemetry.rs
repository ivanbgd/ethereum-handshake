//! The logging/tracing support

use tracing_subscriber::{filter::LevelFilter, EnvFilter};

/// Initialize tracing with the INFO log level as default
pub fn init_tracing() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}
