mod server;

use server::{run_server, ServerConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ServerConfig {
        host: "127.0.0.1".to_string(),
        port: 8080,
    };

    if let Err(err) = run_server(config).await {
        eprintln!("Server error: {:?}", err);
    }

    Ok(())
}
