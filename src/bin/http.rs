use ai_api::{
    api::{self, observe::init_tracing_subscriber},
    settings::Settings,
};
use anyhow::{Context, Result};
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    let settings = Settings::new().context("Failed to load settings")?;
    let _guard = init_tracing_subscriber(&settings);

    let app = api::routes();

    let addr = settings.serve.addr();
    info!("Starting http server at http://{addr}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("Tcp listener {addr} bind failed")?;
    axum::serve(listener, app)
        .await
        .context("Start axum serve failed")?;
    Ok(())
}
