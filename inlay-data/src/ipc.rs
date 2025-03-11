use anyhow::Result;
use inlay_protocol::messages::{ControlCommand, DataResponse};
use log::{debug, error};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

use crate::wireguard::{self, SimulatedDevice};

// Handle a connection from the control plane
pub async fn handle_connection(
    mut socket: UnixStream,
    device: Arc<Mutex<SimulatedDevice>>,
) -> Result<()> {
    loop {
        // Read command size (u32)
        let mut size_buf = [0u8; 4];
        match socket.read_exact(&mut size_buf).await {
            Ok(_) => {},
            Err(_) => break, // Connection closed
        }
        let size = u32::from_le_bytes(size_buf) as usize;

        // Read command data
        let mut data = vec![0u8; size];
        if socket.read_exact(&mut data).await.is_err() {
            break; // Connection closed
        }

        // Deserialize command
        let command: ControlCommand = bincode::deserialize(&data)?;
        debug!("Received command: {:?}", command);

        // Process command
        let response = process_command(command, &device).await?;

        // Serialize and send response
        let response_data = bincode::serialize(&response)?;
        let response_size = (response_data.len() as u32).to_le_bytes();

        socket.write_all(&response_size).await?;
        socket.write_all(&response_data).await?;
    }

    Ok(())
}

// Process a command from the control plane
async fn process_command(
    command: ControlCommand,
    device: &Arc<Mutex<SimulatedDevice>>,
) -> Result<DataResponse> {
    match command {
        ControlCommand::AddPeer { public_key, allowed_ip, endpoint } => {
            wireguard::add_peer(device, public_key, allowed_ip, endpoint).await?;
            Ok(DataResponse::Success)
        }

        ControlCommand::RemovePeer { public_key } => {
            wireguard::remove_peer(device, public_key).await?;
            Ok(DataResponse::Success)
        }

        ControlCommand::GetStatus => {
            // Get device status - simplified for example
            let device_lock = device.lock().unwrap();
            // Use the peer_count method instead of accessing the field directly
            Ok(DataResponse::Status {
                connected: true,
                num_peers: device_lock.peer_count(),
                bytes_sent: 0,
                bytes_received: 0,
            })
        }
    }
}