use anyhow::{Context, Result};
use log::{debug, info};
use std::sync::{Arc, Mutex};

// Simulated WireGuard device for testing
pub struct SimulatedDevice {
    name: String,
    // Make peers public or keep it private and add a method
    peers: Vec<Peer>,
}

pub struct Peer {
    pub public_key: [u8; 32],
    pub allowed_ip: std::net::IpAddr,
    pub endpoint: Option<std::net::SocketAddr>,
}

impl SimulatedDevice {
    fn new(name: String) -> Self {
        Self {
            name,
            peers: Vec::new(),
        }
    }

    fn add_peer(&mut self, peer: Peer) -> Result<()> {
        self.peers.push(peer);
        Ok(())
    }

    fn remove_peer(&mut self, public_key: &[u8; 32]) -> Result<()> {
        self.peers.retain(|p| p.public_key != *public_key);
        Ok(())
    }

    // Add a method to get the number of peers
    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }
}

// Create simulated WireGuard device (for testing)
pub async fn create_simulated_device(interface_name: &str) -> Result<Arc<Mutex<SimulatedDevice>>> {
    info!("Creating simulated WireGuard device {}", interface_name);

    let device = SimulatedDevice::new(interface_name.to_string());

    Ok(Arc::new(Mutex::new(device)))
}

// Add a peer to the simulated WireGuard device
pub async fn add_peer(
    device: &Arc<Mutex<SimulatedDevice>>,
    public_key: [u8; 32],
    allowed_ip: std::net::IpAddr,
    endpoint: Option<std::net::SocketAddr>,
) -> Result<()> {
    debug!("Adding peer with public key: {:?}", public_key);

    let peer = Peer {
        public_key,
        allowed_ip,
        endpoint,
    };

    let mut device_lock = device.lock().unwrap();
    device_lock.add_peer(peer)?;

    Ok(())
}

// Remove a peer from the simulated WireGuard device
pub async fn remove_peer(
    device: &Arc<Mutex<SimulatedDevice>>,
    public_key: [u8; 32],
) -> Result<()> {
    debug!("Removing peer with public key: {:?}", public_key);

    let mut device_lock = device.lock().unwrap();
    device_lock.remove_peer(&public_key)?;

    Ok(())
}