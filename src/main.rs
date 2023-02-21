use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry};
use tracing_subscriber::layer::SubscriberExt;
use web_microservice::startup::run;
use web_microservice::configuration::get_configuration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    LogTracer::init().expect("Unable to init Log Tracer");
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new(
        "web-microservice".into(),
        || std::io::stdout()
    );
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");
    let configuration = get_configuration().expect("Failed to read configuration");
    let address = format!("{}:{}", configuration.application_host, configuration.application_port);
    let listener = TcpListener::bind(address).expect("Failed to bind port");
    run(listener)?.await
}
