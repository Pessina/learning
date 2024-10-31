use tracing::{debug, error, info, warn};
use tracing_stackdriver::layer as stackdriver_layer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

fn main() -> anyhow::Result<()> {
    let base_subscriber = Registry::default().with(EnvFilter::from_default_env());

    let is_gcp = true;
    let subscriber = if is_gcp {
        let stackdriver = stackdriver_layer().with_writer(std::io::stderr);
        base_subscriber.with(None).with(Some(stackdriver))
    } else {
        let fmt_layer = tracing_subscriber::fmt::layer().with_thread_ids(true);
        base_subscriber.with(Some(fmt_layer)).with(None)
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
