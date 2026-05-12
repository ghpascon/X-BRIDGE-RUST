use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let app = x_bridge_rust::app::build_app()?;
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server running at http://{address}");

    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
