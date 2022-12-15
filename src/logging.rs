use futures::future::BoxFuture;
use std::fmt::Debug;
use std::sync::Arc;
use teloxide::error_handlers::ErrorHandler;
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
    let tracing_filter = Targets::new().with_target("linkleaner", Level::DEBUG);
    configure_tracing(tracing_filter)
}

#[derive(Default)]
pub struct TeloxideLogger {}

impl<E> ErrorHandler<E> for TeloxideLogger
where
    E: Debug,
{
    #[track_caller]
    fn handle_error(self: Arc<Self>, error: E) -> BoxFuture<'static, ()> {
        tracing::error!(?error);
        Box::pin(async {})
    }
}
