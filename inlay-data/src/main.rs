mod wireguard;
mod ipc;

use anyhow::Result;
use log::{debug, error, info};
use tokio::net::UnixListener;
use tokio::signal;

use ipc::handle_connection;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    info!("Starting Inlay Data Plane Daemon");
    
    // For testing, we'll create a simulated device
    // In production, this would be a real WireGuard device
    let wg_device = wireguard::create_simulated_device("wg0").await?;
    
    // Create IPC socket for control plane communication
    let socket_path = "/tmp/inlay-data.sock";
    // Remove socket if it exists
    let _ = std::fs::remove_file(socket_path);
    let listener = UnixListener::bind(socket_path)?;
    info!("Listening for control plane connections on {}", socket_path);
    
    // Handle incoming connections and signals
    loop {
        tokio::select! {
            Ok((socket, _)) = listener.accept() => {
                debug!("Accepted connection from control plane");
                let device_clone = wg_device.clone();
                tokio::spawn(async move {
                    if let Err(e) = handle_connection(socket, device_clone).await {
                        error!("Error handling control connection: {}", e);
                    }
                });
            }
            _ = signal::ctrl_c() => {
                info!("Received shutdown signal, exiting...");
                break;
            }
        }
    }
    
    // Clean up
    std::fs::remove_file(socket_path).ok();
    
    Ok(())
}
