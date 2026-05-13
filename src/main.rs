use x_bridge_rust::core::config::AppConfig;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = AppConfig::from_default_path()?;
    let app = x_bridge_rust::app::build_app()?;
    let address = config.socket_addr();

    println!("Server running at http://{address}");

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
