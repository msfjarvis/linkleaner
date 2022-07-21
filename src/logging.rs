use tracing::dispatcher::SetGlobalDefaultError;
use tracing::subscriber::set_global_default;
use tracing::Level;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::registry;

#[cfg(not(feature = "journald"))]
fn configure_tracing(filter: Targets) -> Result<(), SetGlobalDefaultError> {
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::{fmt, Layer};

    let registry = registry();
    #[cfg(feature = "console")]
    let registry = registry.with(console_subscriber::spawn());
    let stdout_log = fmt::layer().pretty();
    let subscriber = registry.with(stdout_log.with_filter(filter));
    set_global_default(subscriber)
}

#[cfg(feature = "journald")]
fn configure_tracing(filter: Targets) -> Result<(), SetGlobalDefaultError> {
    use tracing_journald::layer;
    use tracing_subscriber::layer::SubscriberExt;

    let registry = registry();
    #[cfg(feature = "console")]
    let registry = registry.with(console_subscriber::spawn());
    let subscriber = registry.with(filter).with(layer().unwrap());
    set_global_default(subscriber)
}

pub fn init() -> Result<(), SetGlobalDefaultError> {
    let tracing_filter = Targets::new().with_target("walls_bot_rs", Level::DEBUG);
    configure_tracing(tracing_filter)
}
