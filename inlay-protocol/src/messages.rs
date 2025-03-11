use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};

/// Commands sent from control plane to data plane
#[derive(Debug, Serialize, Deserialize)]
pub enum ControlCommand {
    /// Add a peer to the WireGuard device
    AddPeer {
        /// WireGuard public key
        public_key: [u8; 32],
        /// Allowed IP address
        allowed_ip: IpAddr,
        /// Optional endpoint (for direct connections)
        endpoint: Option<SocketAddr>,
    },
    
    /// Remove a peer from the WireGuard device
    RemovePeer {
        /// WireGuard public key
        public_key: [u8; 32],
    },
    
    /// Get data plane status
    GetStatus,
}

/// Responses sent from data plane to control plane
#[derive(Debug, Serialize, Deserialize)]
pub enum DataResponse {
    /// Operation successful
    Success,
    
    /// Error occurred
    Error(String),
    
    /// Status information
    Status {
        /// Is the device connected?
        connected: bool,
        /// Number of peers
        num_peers: usize,
        /// Bytes sent
        bytes_sent: u64,
        /// Bytes received
        bytes_received: u64,
    },
}
