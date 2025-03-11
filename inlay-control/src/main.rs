mod blockchain;
mod ipc;

use anyhow::Result;
use log::{debug, error, info};
use tokio::net::TcpListener;
use tokio::signal;
use tokio::time::{self, Duration};

use ipc::DataPlaneClient;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    info!("Starting Inlay Control Plane Daemon");
    
    // Connect to data plane daemon
    info!("Connecting to data plane daemon...");
    let socket_path = "/tmp/inlay-data.sock";
    let data_plane = DataPlaneClient::new(socket_path);
    
    // Start API server
    let api_listener = TcpListener::bind("127.0.0.1:3000").await?;
    info!("API server listening on 127.0.0.1:3000");
    
    // For demonstration, add a test peer
    let mut test_public_key = [0u8; 32];
    test_public_key[0] = 1;
    data_plane.add_peer(
        test_public_key,
        "10.0.0.2".parse()?,
        None,
    ).await?;
    info!("Added test peer");
    
    // Handle API requests and signals
    loop {
        tokio::select! {
            Ok((socket, addr)) = api_listener.accept() => {
                debug!("Accepted API connection from {}", addr);
                let data_plane_clone = data_plane.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_api_request(socket, data_plane_clone).await {
                        error!("Error handling API request: {}", e);
                    }
                });
            }
            _ = signal::ctrl_c() => {
                info!("Received shutdown signal, exiting...");
                break;
            }
        }
    }
    
    Ok(())
}

// Handle API request
async fn handle_api_request(
    mut socket: tokio::net::TcpStream,
    data_plane: DataPlaneClient,
) -> Result<()> {
    // Simplified API handling - would be more complex in reality
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    
    let mut buffer = [0u8; 1024];
    let _ = socket.read(&mut buffer).await?;
    
    // Get status from data plane
    let status = data_plane.get_status().await?;
    
    // Send response
    let response = format!("Inlay VPC Status: {:?}\n", status);
    socket.write_all(response.as_bytes()).await?;
    
    Ok(())
}
