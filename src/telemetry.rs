//! src/telemetry.rs

use tracing_subscriber::{EnvFilter, filter::LevelFilter};

/// Initialize tracing with the INFO log level as default
pub fn init_subscriber() {
    let env_filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}
