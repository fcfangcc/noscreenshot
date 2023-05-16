use tracing_appender::rolling;
use tracing_subscriber::{self, filter::Targets, fmt, layer::SubscriberExt, EnvFilter, Layer};

pub fn init(log_dir: &str) {
    let app_file = rolling::daily(log_dir, "app.log");
    let web_file = rolling::daily(log_dir, "webapp.log");

    let subscriber = tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::TRACE.into()))
        .with(
            fmt::Layer::new()
                .with_writer(app_file)
                .json()
                .flatten_event(true)
                .with_timer(fmt::time::LocalTime::rfc_3339())
                .with_filter(
                    Targets::default()
                        .with_target("webapp", tracing::Level::ERROR)
                        .with_default(tracing::Level::TRACE),
                ),
        )
        .with(
            fmt::Layer::new()
                .with_writer(web_file)
                .json()
                .flatten_event(true)
                .with_timer(fmt::time::LocalTime::rfc_3339())
                .with_filter(
                    tracing_subscriber::filter::Targets::new()
                        .with_target("webapp", tracing::Level::TRACE),
                ),
        );

    tracing::subscriber::set_global_default(subscriber).expect("Unable to set a global subscriber");
}
