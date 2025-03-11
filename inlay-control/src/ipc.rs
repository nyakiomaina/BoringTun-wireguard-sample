use anyhow::Result;
use inlay_protocol::messages::{ControlCommand, DataResponse};
use log::debug;
use std::net::{IpAddr, SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::UnixStream;

#[derive(Clone)]
pub struct DataPlaneClient {
    socket_path: String,
}

impl DataPlaneClient {
    // Create a new client
    pub fn new(socket_path: &str) -> Self {
        Self {
            socket_path: socket_path.to_string(),
        }
    }
    
    // Send a command to the data plane
    async fn send_command(&self, command: ControlCommand) -> Result<DataResponse> {
        // Connect to data plane socket
        let mut socket = UnixStream::connect(&self.socket_path).await?;
        
        // Serialize command
        let data = bincode::serialize(&command)?;
        let size = (data.len() as u32).to_le_bytes();
        
        // Send command
        socket.write_all(&size).await?;
        socket.write_all(&data).await?;
        
        // Read response size
        let mut size_buf = [0u8; 4];
        socket.read_exact(&mut size_buf).await?;
        let size = u32::from_le_bytes(size_buf) as usize;
        
        // Read response data
        let mut data = vec![0u8; size];
        socket.read_exact(&mut data).await?;
        
        // Deserialize response
        let response: DataResponse = bincode::deserialize(&data)?;
        debug!("Received response: {:?}", response);
        
        Ok(response)
    }
    
    // Add a peer to the WireGuard device
    pub async fn add_peer(
        &self,
        public_key: [u8; 32],
        allowed_ip: IpAddr,
        endpoint: Option<SocketAddr>,
    ) -> Result<()> {
        let command = ControlCommand::AddPeer {
            public_key,
            allowed_ip,
            endpoint,
        };
        
        let response = self.send_command(command).await?;
        
        match response {
            DataResponse::Success => Ok(()),
            _ => anyhow::bail!("Unexpected response: {:?}", response),
        }
    }
    
    // Remove a peer from the WireGuard device
    pub async fn remove_peer(&self, public_key: [u8; 32]) -> Result<()> {
        let command = ControlCommand::RemovePeer { public_key };
        
        let response = self.send_command(command).await?;
        
        match response {
            DataResponse::Success => Ok(()),
            _ => anyhow::bail!("Unexpected response: {:?}", response),
        }
    }
    
    // Get status from the data plane
    pub async fn get_status(&self) -> Result<DataResponse> {
        let command = ControlCommand::GetStatus;
        self.send_command(command).await
    }
}
