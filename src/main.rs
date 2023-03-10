use color_eyre::Result;

fn main() -> Result<()> {
    setup()?;
    tracing::debug!("Debug logging enabled.");

    tracing::info!("Hello raytracing");

    Ok(())
}

fn setup() -> Result<()> {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    // Install error report and panic hooks
    color_eyre::install()?;

    // Default logging level
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    let filter_layer = EnvFilter::try_from_default_env().or_else(|e| {
        eprintln!("Invalid EnvFilter env: {e}, defaulting to info");
        EnvFilter::try_new("info")
    })?;

    let time_format =
        time::format_description::parse("[hour]:[minute]:[second].[subsecond digits:5]")?;
    let fmt_layer = fmt::layer()
        .compact()
        .with_timer(fmt::time::UtcTime::new(time_format));

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(tracing_error::ErrorLayer::default())
        .init();

    Ok(())
}
