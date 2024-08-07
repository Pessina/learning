use tracing::{debug, error, info, warn};
use tracing_stackdriver::layer as stackdriver_layer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

fn main() -> anyhow::Result<()> {
    let stackdriver = stackdriver_layer().with_writer(std::io::stderr);

    let fmt_layer = tracing_subscriber::fmt::layer().with_thread_ids(true);

    let env_filter = EnvFilter::from_default_env();
    let base_subscriber = Registry::default().with(env_filter);

    let is_gcp = true;
    let subscriber = if is_gcp {
        base_subscriber
            .with(Some(stackdriver))
            .with(fmt_layer.with_ansi(false))
    } else {
        base_subscriber.with(None).with(fmt_layer.with_ansi(true))
    };

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    let _span = tracing::trace_span!("main").entered();

    // let subscriber = tracing_subscriber::fmt()
    //     .with_thread_ids(true)
    //     .with_env_filter(EnvFilter::from_default_env())
    //     .with_ansi(false);

    // subscriber.init();
    // let _span = tracing::trace_span!("cli").entered();

    info!("User logged in");
    debug!("User session created");
    warn!("Some warning");
    error!("Critical operation failed");

    Ok(())
}
