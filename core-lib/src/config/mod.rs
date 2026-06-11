use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubnetraConfig {
    #[serde(default = "default_negotiation_version")]
    pub negotiation_version: u8,
    #[serde(default = "default_tun_mtu")]
    pub local_tun_mtu: u16,
    #[serde(default = "default_listen_port")]
    pub listen_port: u16,
    #[serde(default = "default_virtual_subnet")]
    pub virtual_subnet: String,
    #[serde(default)]
    pub local_tun_ip: String,
    pub local_id: u32,
    #[serde(default = "default_role")]
    pub role: String,
    #[serde(default)]
    pub local_routes: Vec<String>,
    #[serde(default)]
    pub remote_routes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keepalive_secs: Option<u32>,
    #[serde(default)]
    pub peers: Vec<PeerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerConfig {
    pub id: u32,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub endpoint: String,
    #[serde(default = "default_allowed_src")]
    pub allowed_src: String,
    pub psk: String,
}

fn default_negotiation_version() -> u8 { 1 }
fn default_tun_mtu() -> u16 { 1452 }
fn default_listen_port() -> u16 { 51820 }
fn default_virtual_subnet() -> String { "10.0.0.0/24".to_string() }
fn default_allowed_src() -> String { "0.0.0.0/0".to_string() }
fn default_role() -> String { "manual".to_string() }

impl SubnetraConfig {
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: SubnetraConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}